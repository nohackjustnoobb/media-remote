use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Cursor},
    path::PathBuf,
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex, RwLock, RwLockReadGuard,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[cfg(feature = "artwork")]
use base64::{engine::general_purpose, Engine as _};
use flate2::read::GzDecoder;
#[cfg(feature = "artwork")]
use image::ImageReader;
use serde_json::Value;
use tar::Archive;
use tempfile::TempDir;

use crate::{Command as MediaCommand, Controller, ListenerToken, NowPlayingInfo, Subscription};

const ADAPTER_ASSET: &[u8] = include_bytes!("../../assets/mediaremote-adapter.tar.gz");

pub struct NowPlayingPerl {
    info: Arc<RwLock<Option<NowPlayingInfo>>>,
    listeners: Arc<
        Mutex<
            HashMap<
                ListenerToken,
                Box<dyn Fn(RwLockReadGuard<'_, Option<NowPlayingInfo>>) + Send + Sync>,
            >,
        >,
    >,
    token_counter: Arc<AtomicU64>,
    _temp_dir: Arc<TempDir>,
    running: Arc<AtomicBool>,
    adapter_script: PathBuf,
    framework_path: PathBuf,
}

impl NowPlayingPerl {
    pub fn new() -> Self {
        let temp_dir = tempfile::Builder::new()
            .prefix("mediaremote-adapter")
            .tempdir()
            .expect("Failed to create temporary directory");

        let tar = GzDecoder::new(Cursor::new(ADAPTER_ASSET));
        let mut archive = Archive::new(tar);
        archive
            .unpack(temp_dir.path())
            .expect("Failed to unpack adapter assets");

        let adapter_script = temp_dir.path().join("mediaremote-adapter.pl");
        let framework_path = temp_dir.path().join("MediaRemoteAdapter.framework");

        // Clones for the spawned thread, which takes ownership via `move`.
        let adapter_script_thread = adapter_script.clone();
        let framework_path_thread = framework_path.clone();

        let info = Arc::new(RwLock::new(None));
        let listeners = Arc::new(Mutex::new(HashMap::new()));
        let token_counter = Arc::new(AtomicU64::new(0));
        let running = Arc::new(AtomicBool::new(true));

        let info_clone = info.clone();
        let listeners_clone = listeners.clone();
        let running_clone = running.clone();

        // Spawn reading thread
        thread::spawn(move || {
            let mut command = Command::new("/usr/bin/perl");
            command
                .arg(&adapter_script_thread)
                .arg(&framework_path_thread)
                .arg("stream")
                .arg("--no-diff");

            #[cfg(not(feature = "artwork"))]
            command.arg("--no-artwork");

            let mut child = command
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to start mediaremote-adapter");

            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                if !running_clone.load(Ordering::Relaxed) {
                    break;
                }

                if let Ok(line) = line {
                    if let Ok(json) = serde_json::from_str::<Value>(&line) {
                        if let Some(payload) = json.get("payload") {
                            Self::update_info(&info_clone, &listeners_clone, payload);
                        }
                    }
                }
            }

            let _ = child.kill();
        });

        Self {
            info,
            listeners,
            token_counter,
            _temp_dir: Arc::new(temp_dir),
            running,
            adapter_script,
            framework_path,
        }
    }

    #[cfg_attr(not(feature = "artwork"), allow(unused_mut))]
    fn update_info(
        info: &Arc<RwLock<Option<NowPlayingInfo>>>,
        listeners: &Arc<
            Mutex<
                HashMap<
                    ListenerToken,
                    Box<dyn Fn(RwLockReadGuard<'_, Option<NowPlayingInfo>>) + Send + Sync>,
                >,
            >,
        >,
        payload: &Value,
    ) {
        let mut new_info = NowPlayingInfo {
            is_playing: payload["playing"].as_bool(),
            title: payload["title"].as_str().map(|s| s.to_string()),
            artist: payload["artist"].as_str().map(|s| s.to_string()),
            album: payload["album"].as_str().map(|s| s.to_string()),
            #[cfg(feature = "artwork")]
            album_cover: None,
            elapsed_time: payload["elapsedTime"].as_f64(),
            duration: payload["duration"].as_f64(),
            playback_rate: payload["playbackRate"].as_f64(),
            info_update_time: payload["timestamp"]
                .as_str()
                .and_then(|s| speedate::DateTime::parse_str(s).ok())
                .and_then(|dt| {
                    u64::try_from(dt.timestamp())
                        .ok()
                        .and_then(|secs| UNIX_EPOCH.checked_add(Duration::from_secs(secs)))
                })
                .or(Some(SystemTime::now())),
            bundle_id: {
                let mut bid = payload["parentApplicationBundleIdentifier"].as_str();
                if bid.is_none() {
                    bid = payload["bundleIdentifier"].as_str();
                }
                bid.map(|s| s.to_string())
            },
        };

        // Handle artwork
        #[cfg(feature = "artwork")]
        if let Some(artwork_base64) = payload["artworkData"].as_str() {
            // Clean up main string which might have newlines
            let clean_base64 = artwork_base64.replace("\n", "");
            if let Ok(data) = general_purpose::STANDARD.decode(&clean_base64) {
                new_info.album_cover = ImageReader::new(Cursor::new(data))
                    .with_guessed_format()
                    .ok()
                    .and_then(|img| img.decode().ok());
            }
        }

        {
            let mut info_guard = info.write().unwrap();
            *info_guard = Some(new_info);
        }

        // Notify listeners
        for (_, listener) in listeners.lock().unwrap().iter() {
            listener(info.read().unwrap());
        }
    }

    pub fn get_info(&self) -> RwLockReadGuard<'_, Option<NowPlayingInfo>> {
        let mut info_guard = self.info.write().unwrap();

        // Logic to update elapsed time estimation if playing
        if let Some(ref mut info) = *info_guard {
            if info.is_playing == Some(true) {
                if let (Some(elapsed), Some(update_time)) =
                    (info.elapsed_time, info.info_update_time)
                {
                    if let Ok(duration) = SystemTime::now().duration_since(update_time) {
                        info.elapsed_time = Some(elapsed + duration.as_secs_f64());
                        info.info_update_time = Some(SystemTime::now());
                    }
                }
            }
        }
        drop(info_guard);

        self.info.read().unwrap()
    }
}

impl Drop for NowPlayingPerl {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl NowPlayingPerl {
    /// Runs the perl adapter with the `send` command and the given MediaRemote command id.
    /// Returns `true` only when the script exits successfully.
    fn run_send(&self, command: MediaCommand) -> bool {
        let status = Command::new("/usr/bin/perl")
            .arg(&self.adapter_script)
            .arg(&self.framework_path)
            .arg("send")
            .arg((command as i32).to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        matches!(status, Ok(s) if s.success())
    }

    /// Runs the perl adapter with the `seek` command. `position_micros` is in microseconds.
    fn run_seek(&self, position_micros: u64) -> bool {
        let status = Command::new("/usr/bin/perl")
            .arg(&self.adapter_script)
            .arg(&self.framework_path)
            .arg("seek")
            .arg(position_micros.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        matches!(status, Ok(s) if s.success())
    }

    /// Runs the perl adapter with the `speed` command.
    fn run_speed(&self, speed: i32) -> bool {
        let status = Command::new("/usr/bin/perl")
            .arg(&self.adapter_script)
            .arg(&self.framework_path)
            .arg("speed")
            .arg(speed.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        matches!(status, Ok(s) if s.success())
    }
}

impl Controller for NowPlayingPerl {
    fn is_info_some(&self) -> bool {
        self.info.read().unwrap().as_ref().is_some()
    }

    fn toggle(&self) -> bool {
        self.run_send(MediaCommand::TogglePlayPause)
    }

    fn play(&self) -> bool {
        self.run_send(MediaCommand::Play)
    }

    fn pause(&self) -> bool {
        self.run_send(MediaCommand::Pause)
    }

    fn next(&self) -> bool {
        self.run_send(MediaCommand::NextTrack)
    }

    fn previous(&self) -> bool {
        self.run_send(MediaCommand::PreviousTrack)
    }

    fn toggle_shuffle(&self) -> bool {
        self.run_send(MediaCommand::ToggleShuffle)
    }

    fn toggle_repeat(&self) -> bool {
        self.run_send(MediaCommand::ToggleRepeat)
    }

    fn start_forward_seek(&self) -> bool {
        self.run_send(MediaCommand::StartForwardSeek)
    }

    fn end_forward_seek(&self) -> bool {
        self.run_send(MediaCommand::EndForwardSeek)
    }

    fn start_backward_seek(&self) -> bool {
        self.run_send(MediaCommand::StartBackwardSeek)
    }

    fn end_backward_seek(&self) -> bool {
        self.run_send(MediaCommand::EndBackwardSeek)
    }

    fn go_back_fifteen_seconds(&self) -> bool {
        self.run_send(MediaCommand::GoBackFifteenSeconds)
    }

    fn skip_fifteen_seconds(&self) -> bool {
        self.run_send(MediaCommand::SkipFifteenSeconds)
    }

    fn set_playback_speed(&self, speed: i32) {
        self.run_speed(speed);
    }

    fn set_elapsed_time(&self, elapsed_time: f64) {
        // The perl adapter's `seek` command expects a positive integer in microseconds.
        let position_micros = (elapsed_time.max(0.0) * 1_000_000.0) as u64;
        self.run_seek(position_micros);
    }
}

impl Subscription for NowPlayingPerl {
    fn get_info(&self) -> RwLockReadGuard<'_, Option<NowPlayingInfo>> {
        self.get_info()
    }

    fn get_token_counter(&self) -> Arc<AtomicU64> {
        self.token_counter.clone()
    }

    fn get_listeners(
        &self,
    ) -> Arc<
        Mutex<
            HashMap<
                crate::high_level::subscription::ListenerToken,
                Box<dyn Fn(RwLockReadGuard<'_, Option<NowPlayingInfo>>) + Send + Sync>,
            >,
        >,
    > {
        self.listeners.clone()
    }
}
