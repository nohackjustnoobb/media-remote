use std::{
    fmt::{Display, Formatter, Result},
    time::{SystemTime, UNIX_EPOCH},
};

use image::DynamicImage;
use objc2::{rc::Retained, runtime::AnyObject};

pub type Id = *const AnyObject;

#[derive(Debug, Clone)]
pub enum Number {
    Signed(i64),
    Unsigned(u64),
    Floating(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Number::Signed(i) => write!(f, "{}", i),
            Number::Unsigned(u) => write!(f, "{}", u),
            Number::Floating(fl) => write!(f, "{}", fl),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InfoTypes {
    String(String),
    SystemTime(SystemTime),
    Data(Vec<u8>),
    Number(Number),
    Unsupported,
}

impl Display for InfoTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            InfoTypes::String(s) => write!(f, "{}", s),
            InfoTypes::SystemTime(time) => match time.duration_since(UNIX_EPOCH) {
                Ok(duration) => write!(f, "{} seconds since UNIX_EPOCH", duration.as_secs()),
                Err(_) => write!(f, "Time is before UNIX_EPOCH"),
            },
            InfoTypes::Data(data) => write!(f, "[{} bytes of data]", data.len()),
            InfoTypes::Number(num) => write!(f, "{}", num),
            InfoTypes::Unsupported => write!(f, "Unsupported"),
        }
    }
}

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

impl Into<i32> for Command {
    fn into(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone)]
pub enum Notification {
    NowPlayingInfoDidChange,
    NowPlayingPlaybackQueueDidChange,
    NowPlayingApplicationDidChange,
    NowPlayingApplicationIsPlayingDidChange,
    PickableRoutesDidChange,
    RouteStatusDidChange,
    NowPlayingPlaybackQueueChanged,
    PlaybackQueueContentItemsChanged,
    NowPlayingApplicationClientStateDidChange,
}

pub type Observer = Retained<AnyObject>;

impl Notification {
    pub fn as_str(&self) -> &'static str {
        match self {
            Notification::NowPlayingInfoDidChange => {
                "kMRMediaRemoteNowPlayingInfoDidChangeNotification"
            }
            Notification::NowPlayingPlaybackQueueDidChange => {
                "kMRMediaRemoteNowPlayingPlaybackQueueDidChangeNotification"
            }
            Notification::NowPlayingApplicationDidChange => {
                "kMRMediaRemoteNowPlayingApplicationDidChangeNotification"
            }
            Notification::NowPlayingApplicationIsPlayingDidChange => {
                "kMRMediaRemoteNowPlayingApplicationIsPlayingDidChangeNotification"
            }
            Notification::PickableRoutesDidChange => {
                "kMRMediaRemotePickableRoutesDidChangeNotification"
            }
            Notification::RouteStatusDidChange => "kMRMediaRemoteRouteStatusDidChangeNotification",
            Notification::NowPlayingPlaybackQueueChanged => {
                "kMRNowPlayingPlaybackQueueChangedNotification"
            }
            Notification::PlaybackQueueContentItemsChanged => {
                "kMRPlaybackQueueContentItemsChangedNotification"
            }
            Notification::NowPlayingApplicationClientStateDidChange => {
                "kMRMediaRemoteNowPlayingApplicationClientStateDidChange"
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BundleInfo {
    pub name: String,
    pub icon: DynamicImage,
}

#[derive(Debug, Clone)]
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
