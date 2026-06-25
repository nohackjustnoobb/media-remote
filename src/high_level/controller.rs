use crate::{send_command, Command};

macro_rules! send_command {
    ($self:expr,$command:expr) => {{
        if $self.is_info_some() {
            send_command($command)
        } else {
            false
        }
    }};
}

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.toggle();
    /// ```
    fn toggle(&self) -> bool {
        send_command!(self, Command::TogglePlayPause)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.play();
    /// ```
    fn play(&self) -> bool {
        send_command!(self, Command::Play)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.pause();
    /// ```
    fn pause(&self) -> bool {
        send_command!(self, Command::Pause)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.next();
    /// ```
    fn next(&self) -> bool {
        send_command!(self, Command::NextTrack)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.previous();
    /// ```
    fn previous(&self) -> bool {
        send_command!(self, Command::PreviousTrack)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.toggle_shuffle();
    /// ```
    fn toggle_shuffle(&self) -> bool {
        send_command!(self, Command::ToggleShuffle)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.toggle_repeat();
    /// ```
    fn toggle_repeat(&self) -> bool {
        send_command!(self, Command::ToggleRepeat)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.start_forward_seek();
    /// ```
    fn start_forward_seek(&self) -> bool {
        send_command!(self, Command::StartForwardSeek)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.end_forward_seek();
    /// ```
    fn end_forward_seek(&self) -> bool {
        send_command!(self, Command::EndForwardSeek)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.start_backward_seek();
    /// ```
    fn start_backward_seek(&self) -> bool {
        send_command!(self, Command::StartBackwardSeek)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.end_backward_seek();
    /// ```
    fn end_backward_seek(&self) -> bool {
        send_command!(self, Command::EndBackwardSeek)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.go_back_fifteen_seconds();
    /// ```
    fn go_back_fifteen_seconds(&self) -> bool {
        send_command!(self, Command::GoBackFifteenSeconds)
    }

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
    /// let now_playing = NowPlaying::new();
    /// now_playing.skip_fifteen_seconds();
    /// ```
    fn skip_fifteen_seconds(&self) -> bool {
        send_command!(self, Command::SkipFifteenSeconds)
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
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.set_playback_speed(2);
    /// ```
    fn set_playback_speed(&self, speed: i32) {
        if self.is_info_some() {
            crate::set_playback_speed(speed);
        }
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
    /// use media_remote::prelude::*;
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.set_elapsed_time(1.0);
    /// ```
    fn set_elapsed_time(&self, elapsed_time: f64) {
        if self.is_info_some() {
            crate::set_elapsed_time(elapsed_time);
        }
    }
}
