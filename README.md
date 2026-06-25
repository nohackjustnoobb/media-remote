<div align="center">

# MediaRemote in Rust

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nohackjustnoobb/media-remote/main.yml?style=for-the-badge&label=test)](https://github.com/nohackjustnoobb/media-remote/actions/workflows/main.yml)
[![GitHub License](https://img.shields.io/github/license/nohackjustnoobb/media-remote?style=for-the-badge)](https://github.com/nohackjustnoobb/media-remote/blob/master/LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/media-remote?style=for-the-badge)](https://crates.io/crates/media-remote)

</div>

> [!IMPORTANT]  
> After macOS 15.4, Apple introduced entitlement verification in the mediaremoted daemon. Clients without the required entitlement are denied access to NowPlaying information. This library works around this by using a bundled Perl script that interfaces with the system's `mediaremote` via [mediaremote-adapter](https://github.com/ungive/mediaremote-adapter/tree/master). This does **not** require SIP to be disabled. See [macOS 15.4+](#macos-154) for more information.
>
> This is the **perl-only** branch: all non-perl backends (the direct MediaRemote.framework bindings, JXA/AppleScript, and `get_bundle_info`) have been removed to eliminate the `objc2` dependencies and reduce build time.

> [!WARNING]
> Since MediaRemote is a private Apple framework, using it may introduce compatibility or stability issues, and your app may not be approved for distribution on the App Store. Use this library at your own risk.

This library provides access to Apple's private framework, **MediaRemote**, via a bundled Perl adapter. It is primarily designed to access information about media that is currently playing.

This library **should** be safe to use. However, it is the first attempt at building these bindings, so there is a high chance of unexpected errors. If you encounter any issues, please report them in the issue tracker or submit a pull request to help improve the library.

## Quick Start

To get started, first ensure that the library is installed.

```toml
[dependencies]
media-remote = { git = "https://github.com/nohackjustnoobb/media-remote.git", branch = "perl-only" }
```

> [!NOTE]
> ### Cargo Features
>
> The `artwork` feature is **enabled by default**. It enables album cover decoding via the `image` crate.
>
> To disable artwork (reduce dependencies and memory usage):
>
> ```toml
> [dependencies]
> media-remote = { git = "https://github.com/nohackjustnoobb/media-remote.git", branch = "perl-only", default-features = false }
> ```
>
> When disabled:
> - `NowPlayingInfo.album_cover` is removed.
> - The `image` and `base64` crates are not compiled.

Minimal example:

```rust
use media_remote::prelude::*;

fn main() {
    // Create an instance of NowPlayingPerl to interact with the media remote.
    let now_playing = NowPlayingPerl::new();

    // Give the adapter process a moment to start streaming.
    std::thread::sleep(std::time::Duration::from_secs(2));

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

### `NowPlayingPerl::new() -> NowPlayingPerl`

Creates a new instance of `NowPlayingPerl`, unpacks the bundled adapter to a temporary directory, and spawns a background Perl process that streams now playing information.

- **Returns**:
  - `NowPlayingPerl`: A new instance of the `NowPlayingPerl` struct.

### `NowPlayingPerl::get_info(&self) -> RwLockReadGuard<'_, Option<NowPlayingInfo>>`

Retrieves the latest now playing information.

- **Returns**:
  - `RwLockReadGuard<'_, Option<NowPlayingInfo>>`: A guard to the now playing metadata.

- **Note**:
  - The lock should be released as soon as possible to minimize blocking time.

### `NowPlayingPerl::subscribe<F: Fn(RwLockReadGuard<'_, Option<NowPlayingInfo>>) + Send + Sync + 'static>(&self, listener: F) -> ListenerToken`

Subscribes a listener to receive updates when the "Now Playing" information changes.

- **Arguments**:
  - `listener`: A function or closure that accepts a `RwLockReadGuard<'_, Option<NowPlayingInfo>>`.

- **Returns**:
  - `ListenerToken`: A token representing the listener, which can later be used to unsubscribe.

### `NowPlayingPerl::unsubscribe(&self, token: ListenerToken)`

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
    #[cfg(feature = "artwork")]
    pub album_cover: Option<DynamicImage>,
    pub elapsed_time: Option<f64>,
    pub duration: Option<f64>,
    pub playback_rate: Option<f64>,
    pub info_update_time: Option<SystemTime>,
    pub bundle_id: Option<String>,
}
```

> [!NOTE]
> The `album_cover` field is only available when the `artwork` feature is enabled (default). Disable default features to remove it.

### Media Control Functions

These functions allow you to control the currently playing media. To use these functions, import `media_remote::Controller`.

- `NowPlayingPerl::toggle(&self) -> bool`

  Toggles between play and pause states.

- `NowPlayingPerl::play(&self) -> bool`

  Starts playing the media.

- `NowPlayingPerl::pause(&self) -> bool`

  Pauses the media.

- `NowPlayingPerl::next(&self) -> bool`

  Skips to the next track.

- `NowPlayingPerl::previous(&self) -> bool`

  Goes back to the previous track.

- `NowPlayingPerl::toggle_shuffle(&self) -> bool`

  Toggles the shuffle state of the playback queue.

- `NowPlayingPerl::toggle_repeat(&self) -> bool`

  Toggles the repeat state of the playback queue.

- `NowPlayingPerl::start_forward_seek(&self) -> bool`

  Starts a forward seek operation.

- `NowPlayingPerl::end_forward_seek(&self) -> bool`

  Ends a forward seek operation.

- `NowPlayingPerl::start_backward_seek(&self) -> bool`

  Starts a backward seek operation.

- `NowPlayingPerl::end_backward_seek(&self) -> bool`

  Ends a backward seek operation.

- `NowPlayingPerl::go_back_fifteen_seconds(&self) -> bool`

  Seeks backward by fifteen seconds.

- `NowPlayingPerl::skip_fifteen_seconds(&self) -> bool`

  Skips forward by fifteen seconds.

- `NowPlayingPerl::set_playback_speed(&self, speed: i32)`

  Sets the playback speed of the currently active media client.

  - **Arguments**:
    - `speed`: The playback speed multiplier.

  - **Note**:
    - Playback speed changes typically do not work most of the time. Depending on the media client or content, setting the playback speed may not have the desired effect.

- `NowPlayingPerl::set_elapsed_time(&self, elapsed_time: f64)`

  Sets the elapsed time of the currently playing media.

  - **Arguments**:
    - `elapsed_time`: The elapsed time in seconds to set the current position of the media.

  - **Note**:
    - Setting the elapsed time can often cause the media to pause. Be cautious when using this function, as the playback might be interrupted and require manual resumption.
  </details>

## macOS 15.4+

For macOS 15.4+, this library uses `NowPlayingPerl`. This method uses an embedded Perl script to interface with a custom adapter, allowing it to bypass the entitlement check. This is based on [mediaremote-adapter](https://github.com/ungive/mediaremote-adapter/tree/master) by [ungive](https://github.com/ungive).

**Pros:**

- Supports real-time updates.
- **Supports retrieval of artwork** (disable with the `artwork` Cargo feature).

**Cons:**

- Spawns a background process (`perl`).

```rust
use media_remote::NowPlayingPerl;

let now_playing = NowPlayingPerl::new();
// Use it via the Controller and Subscription traits
```

## Development

### Update MediaRemote Adapter

To update the `mediaremote-adapter` submodule and rebuild the assets:

1. Update the submodule:

   ```bash
   git submodule update --remote
   ```

2. Run the build script:

   ```bash
   ./build.sh
   ```

### Testing

There are some tests that run indefinitely to test subscriptions. Since they run forever, they are ignored by default.

To run these tests, use:

```bash
cargo test --test test_now_playing_perl -- --nocapture --exact --ignored
```
