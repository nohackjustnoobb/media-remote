<div align="center">

# MediaRemote in Rust

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nohackjustnoobb/media-remote/main.yml?style=for-the-badge&label=test)](https://github.com/nohackjustnoobb/media-remote/actions/workflows/main.yml)
[![GitHub License](https://img.shields.io/github/license/nohackjustnoobb/media-remote?style=for-the-badge)](https://github.com/nohackjustnoobb/media-remote/blob/master/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/media-remote?style=for-the-badge)](https://crates.io/crates/media-remote)

</div>

This library provides bindings for Apple's private framework, **MediaRemote**. It is primarily designed to access information about media that is currently playing. Therefore, not all methods from the MediaRemote framework are included in these bindings.

This library **should** be safe to use. However, it is the first attempt at building these bindings, so there is a high chance of unexpected errors. If you encounter any issues, please report them in the issue tracker or submit a pull request to help improve the library.

**Warning:** Since MediaRemote is a private Apple framework, using it may introduce compatibility or stability issues, and your app may not be approved for distribution on the App Store. Use this library at your own risk.

## Quick Start

To get started, first ensure that the library is installed.

```rust
use media_remote::NowPlaying;

fn main() {
    // Create an instance of NowPlaying to interact with the media.
    let now_playing = NowPlaying::new();

    // Use a guard lock to safely access media information within this block.
    // The guard should be released as soon as possible to avoid blocking.
    {
        let guard = now_playing.get_info();
        let info = guard.as_ref();

        // If information is available, print the title of the currently playing media.
        if let Some(info) = info {
            println!("Currently playing: {:?}", info.title);
        }
    }

    // Toggle the play/pause state of the media.
    now_playing.toggle();
}
```

## API Documentation

_This is a brief documentation. More detailed documentation, including examples, is written inside the code documentation. Hover over the function to check the documentation._

<details>
  <summary>High Level API</summary>

### `NowPlaying::new() -> NowPlaying`

Creates a new instance of `NowPlaying` and registers for playback notifications.

- **Returns**:

  - `NowPlaying`: A new instance of the `NowPlaying` struct.

### `NowPlaying::get_info(&self) -> RwLockReadGuard<'_, Option<NowPlayingInfo>>`

Retrieves the latest now playing information.

- **Returns**:

  - `RwLockReadGuard<'_, Option<NowPlayingInfo>>`: A guard to the now playing metadata.

- **Note**:

  - The lock should be released as soon as possible to minimize blocking time.

### `NowPlaying::subscribe<F: Fn(RwLockReadGuard<'_, Option<NowPlayingInfo>>) + Send + Sync + 'static>(&self, listener: F) -> ListenerToken`

Subscribes a listener to receive updates when the "Now Playing" information changes.

- **Arguments**:

  - `listener`: A function or closure that accepts a `RwLockReadGuard<'_, Option<NowPlayingInfo>>`.

- **Returns**:

  - `ListenerToken`: A token representing the listener, which can later be used to unsubscribe.

### `NowPlaying::unsubscribe(&self, token: ListenerToken)`

Unsubscribes a previously registered listener using the provided `ListenerToken`.

- **Arguments**:

  - `token`: The `ListenerToken` returned when the listener was subscribed.

### `NowPlayingInfo`

```rust
pub struct NowPlayingInfo {
    pub is_playing: Option<bool>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_cover: Option<DynamicImage>,
    pub elapsed_time: Option<f64>,
    pub duration: Option<f64>,
    pub info_update_time: Option<SystemTime>,
    pub bundle_id: Option<String>,
    pub bundle_name: Option<String>,
    pub bundle_icon: Option<DynamicImage>,
}
```

### Media Control Functions

These functions allow you to control the currently playing media.

- `NowPlaying::toggle(&self) -> bool`

  Toggles between play and pause states.

- `NowPlaying::play(&self) -> bool`

  Starts playing the media.

- `NowPlaying::pause(&self) -> bool`

  Pauses the media.

- `NowPlaying::next(&self) -> bool`

  Skips to the next track.

- `NowPlaying::previous(&self) -> bool`

  Goes back to the previous track.
  </details>

<details>
  <summary>Low Level API</summary>

### `get_now_playing_application_is_playing() -> Option<bool>`

Checks whether the currently playing media application is actively playing.

- **Returns**:

  - `Some(true)`: If a media application is playing.
  - `Some(false)`: If no media is currently playing.
  - `None`: If the function times out (e.g., due to an API failure or missing response).

### `get_now_playing_client() -> Option<Id>`

Retrieves the current "now playing" client ID (which is a reference).

- **Returns**:

  - `Some(Id)`: If a valid client ID is found.
  - `None`: If no client ID is found or the request times out.

- **Note**:

  - This function should not be used as the returned ID is short-lived and may cause undefined behavior when used outside of the block.

### `get_now_playing_application_pid() -> Option<i32>`

Retrieves the current "now playing" application PID.

- **Returns**:

  - `Some(PID)`: If a valid application PID is found.
  - `None`: If no application PID is found or the request times out.

### `get_now_playing_info() -> Option<HashMap<String, InfoTypes>>`

Retrieves the currently playing media information as a `HashMap<String, InfoTypes>`. The function interacts with Apple's CoreFoundation API to extract metadata related to the currently playing media.

- **Returns**:

  - `Some(HashMap<String, InfoTypes>)`: If metadata is successfully retrieved.
  - `None`: If no metadata is available or retrieval fails.

### `get_now_playing_client_parent_app_bundle_identifier() -> Option<String>`

Retrieves the bundle identifier of the parent app for the current "now playing" client.

- **Returns**:

  - `Some(String)`: The bundle identifier of the parent app if successfully retrieved.
  - `None`: If the client ID is invalid, the bundle identifier is null, or retrieval fails.

### `get_now_playing_client_bundle_identifier() -> Option<String>`

Retrieves the bundle identifier of the current "now playing" client.

- **Returns**:

  - `Some(String)`: The bundle identifier of the client app if successfully retrieved.
  - `None`: If the client ID is invalid, the bundle identifier is null, or retrieval fails.

### `send_command(command: Command) -> bool`

Sends a media command to the currently active media client.

- **Arguments**:

  - `command`: The Command to be sent, representing an action like play, pause, skip, etc.

- **Returns**:

  - `true`: If the command was successfully sent and processed.
  - `false`: If the operation failed or the command was not recognized.

- **Notes**:
  - The `useInfo` argument is not supported by this function and is not used in the current implementation.
  - If no media is currently playing, this function may open iTunes (or the default media player) to handle the command.

### `set_playback_speed(speed: i32)`

Sets the playback speed of the currently active media client.

- **Arguments**:

  - `speed`: The playback speed multiplier.

- **Note**:

  - Playback speed changes typically do not work most of the time. Depending on the media client or content, setting the playback speed may not have the desired effect.

### `set_elapsed_time(elapsed_time: f64)`

Sets the elapsed time of the currently playing media.

- **Arguments**:

  - `elapsed_time`: The elapsed time in seconds to set the current position of the media.

- **Note**:

  - Setting the elapsed time can often cause the media to pause. Be cautious when using this function, as the playback might be interrupted and require manual resumption.

### `register_for_now_playing_notifications()`

Registers the caller for "Now Playing" notifications.

- **Note**:
  - Must be called before adding observers to ensure notifications are received.

### `unregister_for_now_playing_notifications()`

Unregisters the caller for "Now Playing" notifications.

- **Note**:

  - Should be called when notifications are no longer needed to free resources.

  </details>

  <details>
  <summary>Helper Functions</summary>

### `add_observer(notification: Notification, closure: F) -> Observer`

Adds an observer for a specific media notification.

- **Arguments**:

  - `notification`: The Notification type representing the event to observe.
  - `closure`: A closure to execute when the notification is received.

- **Returns**:

  - An Observer handle that can be used to remove the observer later.

- **Note**:
  - `register_for_now_playing_notifications()` **must** be called before using this function, or notifications may not be received.

### `remove_observer(observer: Observer)`

Removes a previously added observer.

- **Arguments**:

  - `observer`: The Observer handle returned from add_observer().

### `get_bundle_info(id: &str) -> Option<BundleInfo>`

Retrieves information about an application based on its bundle identifier, including the application's name and icon.

- **Arguments**:

  - `id`: A string slice representing the bundle identifier of the application.

- **Returns**:

  - `Some(BundleInfo)`: If the application is found, containing the application's name and icon.
  - `None`: If the application cannot be found, or if there is an error retrieving the information.

</details>
