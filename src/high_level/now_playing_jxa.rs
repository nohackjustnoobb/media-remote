use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, RwLock, RwLockReadGuard,
    },
    time::{Duration, SystemTime},
};

use crate::{get_bundle_info, get_raw_info, NowPlayingInfo};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListenerToken(u64);

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
}

impl NowPlayingJXA {
    fn update(&mut self, update_interval: Duration) {
        // TODO
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
    ///
    /// let now_playing = NowPlayingJXA::new(Duration::from_secs(3));
    /// ```
    pub fn new(update_interval: Duration) -> Self {
        let mut new_instance = NowPlayingJXA {
            info: Arc::new(RwLock::new(None)),
            listeners: Arc::new(Mutex::new(HashMap::new())),
            token_counter: Arc::new(AtomicU64::new(0)),
        };

        new_instance.update(update_interval);

        new_instance
    }

    /// Subscribes a listener to receive updates when the "Now Playing" information changes.
    ///
    /// # Arguments
    /// - `listener`: A function or closure that accepts a `RwLockReadGuard<'_, Option<NowPlayingInfo>>`.
    ///   The function will be invoked with the current "Now Playing" info whenever the data is updated.
    ///
    /// # Returns
    /// - `ListenerToken`: A token representing the listener, which can later be used to unsubscribe.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::NowPlayingJXA;
    ///
    /// let now_playing = NowPlayingJXA::new(Duration::from_secs(3));
    ///
    /// now_playing.subscribe(|guard| {
    ///     let info = guard.as_ref();
    ///     if let Some(info) = info {
    ///         println!("Currently playing: {:?}", info.title);
    ///     }    
    /// });
    /// ```
    pub fn subscribe<F: Fn(RwLockReadGuard<'_, Option<NowPlayingInfo>>) + Send + Sync + 'static>(
        &self,
        listener: F,
    ) -> ListenerToken {
        listener(self.get_info());

        let token = ListenerToken(self.token_counter.fetch_add(1, Ordering::Relaxed));

        self.listeners
            .lock()
            .unwrap()
            .insert(token.clone(), Box::new(listener));

        token
    }

    /// Unsubscribes a previously registered listener using the provided `ListenerToken`.
    ///
    ///
    /// # Arguments
    /// - `token`: The `ListenerToken` returned when the listener was subscribed. It is used to identify
    ///   and remove the listener.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::NowPlayingJXA;
    ///
    /// let now_playing = NowPlayingJXA::new(Duration::from_secs(3));
    ///
    /// let token = now_playing.subscribe(|guard| {
    ///     let info = guard.as_ref();
    ///     if let Some(info) = info {
    ///         println!("Currently playing: {:?}", info.title);
    ///     }    
    /// });
    ///
    /// now_playing.unsubscribe(token);
    /// ```
    pub fn unsubscribe(&self, token: ListenerToken) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.remove(&token);
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
    ///
    /// let now_playing = NowPlayingJXA::new();
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

impl Controller for NowPlayingJXA {}
