use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Cursor},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex, RwLock, RwLockReadGuard,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose, Engine as _};
use flate2::read::GzDecoder;
use image::ImageReader;
use serde_json::Value;
use tar::Archive;
use tempfile::TempDir;

use crate::{Controller, ListenerToken, NowPlayingInfo, Subscription};

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

        let info = Arc::new(RwLock::new(None));
        let listeners = Arc::new(Mutex::new(HashMap::new()));
        let token_counter = Arc::new(AtomicU64::new(0));
        let running = Arc::new(AtomicBool::new(true));

        let info_clone = info.clone();
        let listeners_clone = listeners.clone();
        let running_clone = running.clone();

        // Spawn reading thread
        thread::spawn(move || {
            let mut child = Command::new("/usr/bin/perl")
                .arg(&adapter_script)
                .arg(&framework_path)
                .arg("stream")
                .arg("--no-diff")
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
        }
    }

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
            album_cover: None,
            elapsed_time: payload["elapsedTime"].as_f64(),
            duration: payload["duration"].as_f64(),
            info_update_time: payload["timestamp"]
                .as_f64()
                .map(|ts| UNIX_EPOCH + Duration::from_secs_f64(ts)),
            bundle_id: payload["bundleIdentifier"].as_str().map(|s| s.to_string()),
            bundle_name: None,
            bundle_icon: None,
        };

        // Handle artwork
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

        if let Some(bundle_id) = &new_info.bundle_id {
            if let Some(bundle_info) = crate::get_bundle_info(bundle_id) {
                new_info.bundle_name = Some(bundle_info.name);
                new_info.bundle_icon = Some(bundle_info.icon);
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

impl Controller for NowPlayingPerl {
    fn is_info_some(&self) -> bool {
        self.info.read().unwrap().as_ref().is_some()
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
