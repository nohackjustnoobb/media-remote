use std::{io::Cursor, ptr::NonNull};

use block2::RcBlock;
use image::ImageReader;
use objc2::rc::Retained;
use objc2_app_kit::NSWorkspace;
use objc2_foundation::{NSFileManager, NSNotification, NSNotificationCenter, NSString};

use crate::{BundleInfo, Notification, Observer};

// Retrieves information about an application based on its bundle identifier.
///
///
/// If the application is found, its name and icon are returned as a `BundleInfo` struct.
/// Otherwise, `None` is returned if the application cannot be located or an error occurs
/// during data retrieval.
///
/// # Arguments
/// - `id`: A string slice representing the application's bundle identifier.
///
/// # Returns
/// - `Option<BundleInfo>`:
///     - `Some(BundleInfo)` containing the application's name and icon if retrieval is successful.
///     - `None` if the application cannot be found or if an error occurs during processing.
///
/// # Example
/// ```rust
/// use media_remote::{get_now_playing_client_parent_app_bundle_identifier, get_bundle_info};
///
/// let bundle_id = get_now_playing_client_parent_app_bundle_identifier();
///
/// if let Some(id) = bundle_id {
///     if let Some(bundle) = get_bundle_info(id.as_str()) {
///         println!("App Name: {}", bundle.name);
///     } else {
///         println!("Application not found.");
///     }
/// }
/// ```
pub fn get_bundle_info(id: &str) -> Option<BundleInfo> {
    unsafe {
        let workspace = NSWorkspace::sharedWorkspace();
        let url = workspace.URLForApplicationWithBundleIdentifier(&NSString::from_str(id))?;

        let absolute = &url.absoluteString()?;

        let file_manager = NSFileManager::defaultManager();
        let name = file_manager.displayNameAtPath(absolute);

        let icon = workspace.iconForFile(absolute);
        let icon_data = icon.TIFFRepresentation()?;
        let icon = ImageReader::new(Cursor::new(icon_data.to_vec()))
            .with_guessed_format()
            .ok()?
            .decode()
            .ok()?;

        Some(BundleInfo {
            name: name.to_string(),
            icon,
        })
    }
}

/// Adds an observer for a specific media notification.
///
/// This function registers a closure to be executed when the specified `notification`
/// is posted. It listens for notifications related to media playback state changes.
///
/// **Note:** [`register_for_now_playing_notifications`] **must** be called before using
/// this function to ensure notifications are received.
///
/// # Arguments
/// - `notification`: The [`Notification`] type representing the event to observe.
/// - `closure`: A closure to execute when the notification is received.
///
/// # Returns
/// - An [`Observer`] handle that can be used to remove the observer later.
///
/// # Example
/// ```rust
/// use media_remote::{register_for_now_playing_notifications, add_observer, Notification};
///
/// register_for_now_playing_notifications();
///
/// let observer = add_observer(Notification::NowPlayingApplicationIsPlayingDidChange, || {
///     println!("Now playing status changed.");
/// });
/// ```
pub fn add_observer<F: Fn() + 'static>(notification: Notification, closure: F) -> Observer {
    unsafe {
        let observer = NSNotificationCenter::defaultCenter()
            .addObserverForName_object_queue_usingBlock(
                Some(NSString::from_str(notification.as_str()).as_ref()),
                None,
                None,
                &RcBlock::new(move |_: NonNull<NSNotification>| closure()),
            );

        Retained::cast_unchecked(observer)
    }
}

/// Removes a previously added observer.
///
/// This function removes an observer registered with [`add_observer`], preventing further
/// notifications from being received.
///
/// # Arguments
/// - `observer`: The [`Observer`] handle returned from [`add_observer`].
///
/// # Example
/// ```rust
/// use media_remote::{register_for_now_playing_notifications, add_observer, remove_observer, Notification};
///
/// register_for_now_playing_notifications();
///
/// let observer = add_observer(Notification::NowPlayingApplicationIsPlayingDidChange, || {
///     println!("Now playing status changed.");
/// });
///
/// // Later, when no longer needed:
/// remove_observer(observer);
/// ```
pub fn remove_observer(observer: Observer) {
    unsafe {
        NSNotificationCenter::defaultCenter().removeObserver(&observer);
    }
}
