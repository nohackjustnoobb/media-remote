use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex, RwLock, RwLockReadGuard,
    },
    thread::{self, JoinHandle},
    time::{Duration, SystemTime},
};

use crate::{get_bundle_info, get_raw_info, ListenerToken, NowPlayingInfo, Subscription};

use super::controller::Controller;

pub fn get_info() -> Option<NowPlayingInfo> {
    let raw = get_raw_info()?;

    let mut bundle_id = raw["client"]["parentApplicationBundleIdentifier"].as_str();
    if bundle_id.is_none() {
        bundle_id = raw["client"]["bundleIdentifier"].as_str();
    }
    let bundle_id = bundle_id?;

    let bundle_info = get_bundle_info(bundle_id)?;

    Some(NowPlayingInfo {
        is_playing: raw["isPlaying"].as_bool(),
        title: raw["info"]["kMRMediaRemoteNowPlayingInfoTitle"]
            .as_str()
            .map(|s| s.to_string()),
        artist: raw["info"]["kMRMediaRemoteNowPlayingInfoArtist"]
            .as_str()
            .map(|s| s.to_string()),
        album: raw["info"]["kMRMediaRemoteNowPlayingInfoAlbum"]
            .as_str()
            .map(|s| s.to_string()),
        album_cover: None,
        elapsed_time: raw["info"]["kMRMediaRemoteNowPlayingInfoElapsedTime"].as_f64(),
        duration: raw["info"]["kMRMediaRemoteNowPlayingInfoDuration"].as_f64(),
        info_update_time: raw["info"]["kMRMediaRemoteNowPlayingInfoTimestamp"]
            .as_u64()
            .and_then(|t| Some(SystemTime::UNIX_EPOCH + Duration::from_millis(t)))
            .or(Some(SystemTime::now())),
        bundle_id: Some(bundle_id.to_string()),
        bundle_name: Some(bundle_info.name),
        bundle_icon: Some(bundle_info.icon),
    })
}

pub struct NowPlayingJXA {
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
    stop_flag: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl NowPlayingJXA {
    fn update(&mut self, update_interval: Duration) {
        let info_clone = Arc::clone(&self.info);
        let stop_clone = Arc::clone(&self.stop_flag);
        let listeners = Arc::clone(&self.listeners);

        self.handle = Some(thread::spawn(move || {
            while !stop_clone.load(Ordering::Relaxed) {
                thread::sleep(update_interval);
                if let Some(new_info) = get_info() {
                    let mut current = info_clone.write().unwrap();
                    if current.as_ref() != Some(&new_info) {
                        *current = Some(new_info);
                        drop(current);

                        for (_, listener) in listeners.clone().lock().unwrap().iter() {
                            listener(info_clone.read().unwrap());
                        }
                    }
                }
            }
        }));
    }

    /// Creates a new instance of `NowPlayingJXA` and registers for playback notifications.
    ///
    /// This function initializes a new `NowPlayingJXA` object, sets up necessary observers,
    /// and ensures that media metadata is updated upon creation.
    ///
    /// # Returns
    /// - `NowPlayingJXA`: A new instance of the `NowPlayingJXA` struct.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::NowPlayingJXA;
    /// use std::time::Duration;
    ///
    /// let now_playing = NowPlayingJXA::new(Duration::from_secs(3));
    /// ```
    pub fn new(update_interval: Duration) -> Self {
        let mut new_instance = NowPlayingJXA {
            info: Arc::new(RwLock::new(None)),
            listeners: Arc::new(Mutex::new(HashMap::new())),
            token_counter: Arc::new(AtomicU64::new(0)),
            stop_flag: Arc::new(AtomicBool::new(false)),
            handle: None,
        };

        new_instance.update(update_interval);

        new_instance
    }

    /// Retrieves the latest now playing information.
    ///
    /// This function provides a read-locked view of the current playing media metadata.
    ///
    /// # Note
    /// - The lock should be released as soon as possible to minimize blocking time.
    ///
    /// # Returns
    /// - `RwLockReadGuard<'_, Option<NowPlayingInfo>>`: A guard to the now playing metadata.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::NowPlayingJXA;
    /// use std::time::Duration;
    ///
    /// let now_playing = NowPlayingJXA::new(Duration::from_secs(3));
    /// let guard = now_playing.get_info();
    /// let info = guard.as_ref();
    ///
    /// if let Some(info) = info {
    ///     println!("Currently playing: {:?}", info.title);
    /// }
    ///
    /// drop(guard);
    /// ```
    pub fn get_info(&self) -> RwLockReadGuard<'_, Option<NowPlayingInfo>> {
        let mut info_guard = self.info.write().unwrap();
        let info = info_guard.as_mut();

        if info.is_some() {
            let info = info.unwrap();
            if info.is_playing.is_some_and(|x| x)
                && info.elapsed_time.is_some()
                && info.info_update_time.is_some()
            {
                info.elapsed_time = Some(
                    info.elapsed_time.unwrap()
                        + info.info_update_time.unwrap().elapsed().unwrap().as_secs() as f64,
                );
                info.info_update_time = Some(SystemTime::now())
            }
        }

        drop(info_guard);

        self.info.read().unwrap()
    }
}

impl Drop for NowPlayingJXA {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Controller for NowPlayingJXA {
    fn is_info_some(&self) -> bool {
        self.info.read().unwrap().as_ref().is_some()
    }
}

impl Subscription for NowPlayingJXA {
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
                super::subscription::ListenerToken,
                Box<dyn Fn(RwLockReadGuard<'_, Option<NowPlayingInfo>>) + Send + Sync>,
            >,
        >,
    > {
        self.listeners.clone()
    }
}
