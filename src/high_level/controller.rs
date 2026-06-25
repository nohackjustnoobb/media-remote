/// A trait for controlling media playback.
///
/// Every method is implemented by the backing media-remote adapter
/// (e.g. [`crate::NowPlayingPerl`]).
pub trait Controller {
    fn is_info_some(&self) -> bool;

    /// Toggles between play and pause states.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.toggle();
    /// ```
    fn toggle(&self) -> bool;

    /// Play the currently playing media.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the operation failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.play();
    /// ```
    fn play(&self) -> bool;

    /// Pauses the currently playing media.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.pause();
    /// ```
    fn pause(&self) -> bool;

    /// Skips to the next track in the playback queue.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.next();
    /// ```
    fn next(&self) -> bool;

    /// Returns to the previous track in the playback queue.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.previous();
    /// ```
    fn previous(&self) -> bool;

    /// Toggles the shuffle state of the playback queue.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.toggle_shuffle();
    /// ```
    fn toggle_shuffle(&self) -> bool;

    /// Toggles the repeat state of the playback queue.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.toggle_repeat();
    /// ```
    fn toggle_repeat(&self) -> bool;

    /// Starts a forward seek operation.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.start_forward_seek();
    /// ```
    fn start_forward_seek(&self) -> bool;

    /// Ends a forward seek operation.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.end_forward_seek();
    /// ```
    fn end_forward_seek(&self) -> bool;

    /// Starts a backward seek operation.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.start_backward_seek();
    /// ```
    fn start_backward_seek(&self) -> bool;

    /// Ends a backward seek operation.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.end_backward_seek();
    /// ```
    fn end_backward_seek(&self) -> bool;

    /// Seeks backward by fifteen seconds.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.go_back_fifteen_seconds();
    /// ```
    fn go_back_fifteen_seconds(&self) -> bool;

    /// Skips forward by fifteen seconds.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.skip_fifteen_seconds();
    /// ```
    fn skip_fifteen_seconds(&self) -> bool;

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
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.set_playback_speed(2);
    /// ```
    fn set_playback_speed(&self, speed: i32);

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
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlayingPerl::new();
    /// now_playing.set_elapsed_time(1.0);
    /// ```
    fn set_elapsed_time(&self, elapsed_time: f64);
}
