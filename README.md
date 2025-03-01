# MediaRemote in Rust

This library provides bindings for Apple's private framework, MediaRemote. It is primarily designed to access information about media that is currently playing. As a result, not all methods from the MediaRemote framework are included in these bindings.

This library **should** be safe to use, but it is the first attempt at building these bindings, and unexpected errors may occur. If you encounter any issues, please report them in the issue tracker or submit a pull request to help improve the library.

**Warning:** Since MediaRemote is a private Apple framework, using it may introduce compatibility or stability issues, and your app may not be approved for distribution on the App Store. Use this library at your own risk.

## API Documentation

_All the examples for usage are written inside the code documentation._

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

**Note:** This function should not be used as the returned ID is short-lived and may cause undefined behavior when used outside of the block.

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
