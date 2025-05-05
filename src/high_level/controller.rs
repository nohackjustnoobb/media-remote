use crate::{send_command, Command};

pub trait Controller {
    /// Toggles between play and pause states.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::{Controller, NowPlaying};
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.toggle();
    /// ```
    fn toggle(&self) -> bool {
        send_command(Command::TogglePlayPause)
    }

    /// Play the currently playing media.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the operation failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::{Controller, NowPlaying};
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.play();
    /// ```
    fn play(&self) -> bool {
        send_command(Command::Play)
    }

    /// Pauses the currently playing media.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::{Controller, NowPlaying};
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.pause();
    /// ```
    fn pause(&self) -> bool {
        send_command(Command::Pause)
    }

    /// Skips to the next track in the playback queue.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::{Controller, NowPlaying};
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.next();
    /// ```
    fn next(&self) -> bool {
        send_command(Command::NextTrack)
    }

    /// Returns to the previous track in the playback queue.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::{Controller, NowPlaying};
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.previous();
    /// ```
    fn previous(&self) -> bool {
        send_command(Command::PreviousTrack)
    }
}
