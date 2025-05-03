use block2::RcBlock;
use core::ffi::c_int;
use dispatch2::ffi::{dispatch_queue_create, DISPATCH_QUEUE_SERIAL};
use objc2::{runtime::AnyObject, Encoding};
use objc2_core_foundation::{
    CFData, CFDate, CFDictionary, CFDictionaryGetCount, CFDictionaryGetKeysAndValues,
};
use objc2_foundation::{NSNumber, NSString};
use std::{
    collections::HashMap,
    ffi::c_void,
    ptr::{self, NonNull},
    rc::Rc,
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

mod c_functions;
use c_functions::*;

mod types;
pub use types::*;

mod helpers;
pub use helpers::*;

mod now_playing;
pub use now_playing::*;

/// Timeout duration for waiting on the media remote response.
const TIMEOUT_DURATION: Duration = Duration::from_secs(5);

macro_rules! safely_dispatch_and_wait {
    ($closure:expr, $type:ty, $func:ident) => {{
        let result = Arc::new((Mutex::new(None), Condvar::new()));

        let result_clone = Arc::clone(&result);
        let block = RcBlock::new(move |arg: $type| {
            let (lock, cvar) = &*result_clone;
            let mut result_guard = lock.lock().unwrap();

            *result_guard = $closure(arg);

            cvar.notify_one();
        });

        unsafe {
            let queue = dispatch_queue_create(ptr::null(), DISPATCH_QUEUE_SERIAL);
            if queue.is_null() {
                return None;
            }

            $func(queue, &block);

            // TODO ChatGPT: If necessary, release queue after usage (depending on API behavior)
            // IDK if that is true
        }

        let (lock, cvar) = &*result;
        let result_guard = match lock.lock() {
            Ok(guard) => guard,
            Err(_) => return None,
        };

        let (result_guard, timeout_result) = match cvar.wait_timeout(result_guard, TIMEOUT_DURATION)
        {
            Ok(res) => res,
            Err(_) => return None,
        };

        if timeout_result.timed_out() {
            None
        } else {
            result_guard.clone()
        }
    }};
}

/// Checks whether the currently playing media application is actively playing.
///
/// The check is performed asynchronously using a callback mechanism,
/// but the function blocks the calling thread until a result is available or a timeout occurs.
///
/// # Returns
/// - `Some(true)`: If a media application is playing.
/// - `Some(false)`: If no media is currently playing.
/// - `None`: If the function times out (e.g., due to an API failure or missing response).
///
///
/// # Example
/// ```rust
/// use media_remote::get_now_playing_application_is_playing;
///
/// if let Some(is_playing) = get_now_playing_application_is_playing() {
///     println!("Is playing: {}", is_playing);
/// } else {
///     println!("Failed to retrieve playing state.");
/// }
/// ```
pub fn get_now_playing_application_is_playing() -> Option<bool> {
    safely_dispatch_and_wait!(
        |is_playing: c_int| Some(is_playing != 0),
        c_int,
        MRMediaRemoteGetNowPlayingApplicationIsPlaying
    )
}

/// Retrieves the current "now playing" client ID (which is a reference).
///
/// This function **should not be used** because the ID is a short-lived reference,
/// likely only valid within the block where it is obtained.
/// Using it outside the block could lead to undefined behavior or dangling references.
///
/// If client identification is needed, consider an alternative approach
/// that ensures the ID remains valid for the required duration.
///
/// # Example (Do not use)
/// ```rust
/// use media_remote::get_now_playing_client;
///
/// let client_id = get_now_playing_client();
/// match client_id {
///     Some(client) => println!("Now playing client: {:?}", client),
///     None => println!("No client found or timed out."),
/// }
/// ```
pub fn get_now_playing_client() -> Option<Id> {
    safely_dispatch_and_wait!(
        |id: Id| {
            if !id.is_null() {
                Some(id)
            } else {
                None
            }
        },
        Id,
        MRMediaRemoteGetNowPlayingClient
    )
}

/// Retrieves the current "now playing" application PID.
///
/// The check is performed asynchronously using a callback mechanism,
/// but the function blocks the calling thread until a result is available or a timeout occurs.
/// If a application PID ID is received, it will be returned as `Some(PID)`, otherwise, it returns `None`.
///
/// # Returns
/// - `Option<PID>`:
///     - `Some(PID)` if a valid application PID is found.
///     - `None` if the client PID retrieval failed (due to timeout or invalid result).
///
/// # Example
/// ```rust
/// use media_remote::get_now_playing_application_pid;
///
/// let pid = get_now_playing_application_pid();
/// match pid {
///     Some(pid) => println!("Now playing application PID: {:?}", pid),
///     None => println!("No application found or timed out."),
/// }
/// ```
pub fn get_now_playing_application_pid() -> Option<i32> {
    safely_dispatch_and_wait!(
        |pid: c_int| {
            if pid != 0 {
                Some(pid)
            } else {
                None
            }
        },
        c_int,
        MRMediaRemoteGetNowPlayingApplicationPID
    )
}

/// Retrieves the currently playing media information as a `HashMap<String, InfoTypes>`.
///
/// The function interacts with Apple's CoreFoundation API to extract metadata
/// related to the currently playing media. It blocks execution until the data is retrieved.
///
/// The keys used in the returned `HashMap` can be found in the [EnergyBar repository](https://github.com/billziss-gh/EnergyBar/blob/master/src/System/NowPlaying.m),
/// but note that the types may not be correct.
///
/// # Returns
/// - `Some(HashMap<String, InfoTypes>)`: If metadata is successfully retrieved.
/// - `None`: If no metadata is available or retrieval fails.
///
///
/// # Example
/// ```rust
/// use media_remote::get_now_playing_info;
///
/// if let Some(info) = get_now_playing_info() {
///     for (key, value) in info.iter() {
///         println!("{}: {:?}", key, value);
///     }
/// } else {
///     println!("No now playing info available.");
/// }
/// ```
pub fn get_now_playing_info() -> Option<HashMap<String, InfoTypes>> {
    #![allow(useless_ptr_null_checks)]
    let info = safely_dispatch_and_wait!(
        |dict: NonNull<CFDictionary>| {
            if dict.as_ptr().is_null() {
                return None;
            }

            unsafe {
                let count = CFDictionaryGetCount(dict.as_ref());

                let mut keys: Vec<*const c_void> = vec![ptr::null(); count.try_into().unwrap()];
                let mut values: Vec<*const c_void> = vec![ptr::null(); count.try_into().unwrap()];

                CFDictionaryGetKeysAndValues(dict.as_ref(), keys.as_mut_ptr(), values.as_mut_ptr());

                let mut info = HashMap::<String, InfoTypes>::new();
                for i in 0..count.try_into().unwrap() {
                    let key_ptr = keys[i];
                    let val_ptr = values[i];

                    let key_ref = &*(key_ptr as *const NSString);
                    let val_ref = &*(val_ptr as *const AnyObject);

                    let class_name = val_ref.class().name().to_str().unwrap_or_default();

                    let value = match class_name {
                        "__NSCFNumber" => {
                            let num_ref = &*(val_ptr as *const NSNumber);
                            let number = match num_ref.encoding() {
                                Encoding::Char
                                | Encoding::Short
                                | Encoding::Int
                                | Encoding::Long
                                | Encoding::LongLong => Number::Signed(num_ref.as_i64()),
                                Encoding::UChar
                                | Encoding::UShort
                                | Encoding::UInt
                                | Encoding::ULong
                                | Encoding::ULongLong => Number::Unsigned(num_ref.as_u64()),
                                Encoding::Float | Encoding::Double => {
                                    Number::Floating(num_ref.as_f64())
                                }
                                _ => unreachable!(),
                            };

                            InfoTypes::Number(number)
                        }
                        "__NSCFString" | "__NSCFConstantString" | "NSTaggedPointerString" => {
                            let str_ref = &*(val_ptr as *const NSString);
                            InfoTypes::String(str_ref.to_string())
                        }
                        "__NSTaggedDate" => {
                            let date_ref = &*(val_ptr as *const CFDate);
                            InfoTypes::SystemTime(date_ref.to_system_time().unwrap())
                        }
                        "NSSubrangeData" | "_NSInlineData" => {
                            let data_ref = &*(val_ptr as *const CFData);
                            InfoTypes::Data(data_ref.to_vec())
                        }
                        _ => InfoTypes::Unsupported,
                    };

                    info.insert(key_ref.to_string(), value);
                }

                Some(Rc::new(info))
            }
        },
        NonNull<CFDictionary>,
        MRMediaRemoteGetNowPlayingInfo
    );

    info.and_then(|info| Rc::try_unwrap(info).ok())
}

macro_rules! get_bundle_identifier {
    ($getter:ident) => {
        safely_dispatch_and_wait!(
            |id: Id| {
                if !id.is_null() {
                    unsafe {
                        let property = $getter(id);
                        if !property.is_null() {
                            return Some((*property).to_string());
                        }
                    }
                }
                None
            },
            Id,
            MRMediaRemoteGetNowPlayingClient
        )
    };
}

/// Retrieves the bundle identifier of the parent app for the current "now playing" client.
///
/// This function attempts to get the parent application's bundle identifier
/// for the currently active media client. The operation is performed asynchronously
/// but blocks the calling thread until a result is available or a timeout occurs.
///
/// Because the original `NSString` reference is short-lived, it is converted into a `String`
/// to ensure safe usage beyond the scope of the function.
///
/// # Returns
/// - `Option<String>`:
///     - `Some(String)` containing the bundle identifier if retrieval is successful.
///     - `None` if the client ID is invalid, the bundle identifier is null, or retrieval fails.
///
/// # Example
/// ```rust
/// use media_remote::get_now_playing_client_parent_app_bundle_identifier;
///
/// if let Some(bundle_id) = get_now_playing_client_parent_app_bundle_identifier() {
///     println!("Now playing client parent app: {}", bundle_id);
/// } else {
///     println!("No parent app found or retrieval failed.");
/// }
/// ```
pub fn get_now_playing_client_parent_app_bundle_identifier() -> Option<String> {
    get_bundle_identifier!(MRNowPlayingClientGetParentAppBundleIdentifier)
}

/// Retrieves the bundle identifier of the current "now playing" client.
///
/// This function attempts to get the application's bundle identifier
/// for the currently active media client. The operation is performed asynchronously
/// but blocks the calling thread until a result is available or a timeout occurs.
///
/// Because the original `NSString` reference is short-lived, it is converted into a `String`
/// to ensure safe usage beyond the scope of the function.
///
/// # Returns
/// - `Option<String>`:
///     - `Some(String)` containing the bundle identifier if retrieval is successful.
///     - `None` if the client ID is invalid, the bundle identifier is null, or retrieval fails.
///
/// # Example
/// ```rust
/// use media_remote::get_now_playing_client_bundle_identifier;
///
/// if let Some(bundle_id) = get_now_playing_client_bundle_identifier() {
///     println!("Now playing client app: {}", bundle_id);
/// } else {
///     println!("No app found or retrieval failed.");
/// }
/// ```
pub fn get_now_playing_client_bundle_identifier() -> Option<String> {
    get_bundle_identifier!(MRNowPlayingClientGetBundleIdentifier)
}

/// Sends a media command to the currently active media client.
///
/// This function sends a command to the media remote system, instructing the currently
/// active media client to perform a specific action.
///
/// # Note
/// - The `useInfo` argument is not supported by this function and is not used in
/// the current implementation.
/// - If no media is currently playing, this function may open iTunes (or the default media player)
/// to handle the command.
///
/// # Arguments
/// - `command`: The `Command` to be sent to the media client. This can represent various
///   actions such as play, pause, skip, or volume control.
///
/// # Returns
/// - `bool`:
///     - `true` if the command was successfully sent and processed.
///     - `false` if the operation failed or the command was not recognized.
///
/// # Example
/// ```rust
/// use media_remote::{send_command, Command};
///
/// if send_command(Command::Play) {
///     println!("Command sent successfully.");
/// } else {
///     println!("Failed to send command.");
/// }
/// ```
pub fn send_command(command: Command) -> bool {
    unsafe { MRMediaRemoteSendCommand(command.into(), ptr::null()) }
}

/// Sets the playback speed of the currently active media client.
///
/// # Arguments
/// - `speed`: The playback speed multiplier.
///
/// # Note
/// - Playback speed changes typically do not work most of the time.
///   Depending on the media client or content, setting the playback speed may not have the desired effect.
///
/// # Example
/// ```rust
/// use media_remote::set_playback_speed;
///
/// let speed = 2;
/// set_playback_speed(speed);
/// println!("Playback speed set to: {}", speed);
/// ```
pub fn set_playback_speed(speed: i32) {
    unsafe { MRMediaRemoteSetPlaybackSpeed(speed) }
}

/// Sets the elapsed time of the currently playing media.
///
/// # Arguments
/// - `elapsed_time`: The elapsed time in seconds to set the current position of the media.
///
/// # Note
/// - **Limitations**: Setting the elapsed time can often cause the media to pause. Be cautious
///   when using this function, as the playback might be interrupted and require manual resumption.
///
/// # Example
/// ```rust
/// use media_remote::set_elapsed_time;
///
/// let elapsed = 1.0;  
/// set_elapsed_time(elapsed);
/// println!("Elapsed time set to: {} seconds", elapsed);
pub fn set_elapsed_time(elapsed_time: f64) {
    unsafe { MRMediaRemoteSetElapsedTime(elapsed_time) }
}

/// Registers for "Now Playing" notifications.
///
/// This function **must** be called before adding any observers using [`add_observer`],
/// or notifications may not be received.
///
/// # Example
/// ```rust
/// use media_remote::register_for_now_playing_notifications;
///
/// register_for_now_playing_notifications();
/// ```
pub fn register_for_now_playing_notifications() {
    unsafe {
        let queue = dispatch_queue_create(ptr::null(), DISPATCH_QUEUE_SERIAL);
        MRMediaRemoteRegisterForNowPlayingNotifications(queue);
    }
}

/// Unregisters from "Now Playing" notifications.
///
///
/// # Example
/// ```rust
/// use media_remote::unregister_for_now_playing_notifications;
///
/// unregister_for_now_playing_notifications();
/// ```
pub fn unregister_for_now_playing_notifications() {
    unsafe {
        MRMediaRemoteUnregisterForNowPlayingNotifications();
    }
}
