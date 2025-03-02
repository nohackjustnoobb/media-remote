# MediaRemote in Rust

This library provides bindings for Apple's private framework, MediaRemote. It is primarily designed to access information about media that is currently playing. As a result, not all methods from the MediaRemote framework are included in these bindings.

This library **should** be safe to use, but it is the first attempt at building these bindings, and unexpected errors may occur. If you encounter any issues, please report them in the issue tracker or submit a pull request to help improve the library.

**Warning:** Since MediaRemote is a private Apple framework, using it may introduce compatibility or stability issues, and your app may not be approved for distribution on the App Store. Use this library at your own risk.

## API Documentation

_This is a brief documentation. More detailed documentation, including examples, is written inside the code documentation. Hover over the function to check the documentation._

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

## TODO

- [-] Support NSNotificationCenter Observer
- [ ] Higher level API
- [-] Helper functions for getting app name and icon
