use std::time::SystemTime;

#[cfg(feature = "artwork")]
use image::DynamicImage;

#[derive(Debug, Clone)]
pub enum Command {
    Play = 0,
    Pause = 1,
    TogglePlayPause = 2,
    Stop = 3,
    NextTrack = 4,
    PreviousTrack = 5,
    ToggleShuffle = 6,
    ToggleRepeat = 7,
    StartForwardSeek = 8,
    EndForwardSeek = 9,
    StartBackwardSeek = 10,
    EndBackwardSeek = 11,
    GoBackFifteenSeconds = 12,
    SkipFifteenSeconds = 13,
}

impl From<Command> for i32 {
    fn from(command: Command) -> i32 {
        command as i32
    }
}

#[derive(PartialEq, Debug, Clone)]
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
