use std::{
    collections::HashMap,
    io::Cursor,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, RwLock, RwLockReadGuard,
    },
    time::SystemTime,
};

use image::{DynamicImage, ImageReader};

use crate::{
    add_observer, get_bundle_info, get_now_playing_application_is_playing,
    get_now_playing_client_bundle_identifier, get_now_playing_client_parent_app_bundle_identifier,
    get_now_playing_info, register_for_now_playing_notifications, remove_observer, send_command,
    unregister_for_now_playing_notifications, Command, InfoTypes, Notification, Number, Observer,
};

/// A struct for managing and interacting with the "Now Playing" media session.
///
/// The `NowPlaying` struct allows access to the currently playing media information,
/// and provides functionality to control playback (e.g., play, pause, skip).
///
///
/// # Example
/// ```rust
/// use media_remote::NowPlaying;
///
/// let now_playing = NowPlaying::new();
/// now_playing.play();
/// ```
pub struct NowPlaying {
    info: Arc<RwLock<Option<NowPlayingInfo>>>,
    observers: Vec<Observer>,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListenerToken(u64);

#[derive(Debug, Clone)]
pub struct NowPlayingInfo {
    pub is_playing: Option<bool>,

    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_cover: Option<DynamicImage>,
    pub elapsed_time: Option<f64>,
    pub duration: Option<f64>,
    info_update_time: Option<SystemTime>,

    pub bundle_id: Option<String>,
    pub bundle_name: Option<String>,
    pub bundle_icon: Option<DynamicImage>,
}

macro_rules! send_command {
    ($self:expr,$command:expr) => {{
        if $self.info.read().unwrap().as_ref().is_some() {
            send_command($command)
        } else {
            false
        }
    }};
}

fn update_all(info: Arc<RwLock<Option<NowPlayingInfo>>>) {
    let mut info_guard = info.write().unwrap();
    *info_guard = Some(NowPlayingInfo {
        is_playing: None,
        title: None,
        artist: None,
        album: None,
        album_cover: None,
        elapsed_time: None,
        duration: None,
        info_update_time: None,
        bundle_id: None,
        bundle_name: None,
        bundle_icon: None,
    });
    drop(info_guard);

    update_state(info.clone());
    update_app(info.clone());
    update_info(info.clone());
}

fn update_state(info: Arc<RwLock<Option<NowPlayingInfo>>>) {
    let mut info_guard = info.write().unwrap();
    if info_guard.as_ref().is_none() {
        drop(info_guard);
        return update_all(info.clone());
    }

    let is_playing = get_now_playing_application_is_playing();
    if let Some(is_playing) = is_playing {
        info_guard.as_mut().unwrap().is_playing = Some(is_playing);
    }
}

fn update_info(info: Arc<RwLock<Option<NowPlayingInfo>>>) {
    let mut info_guard = info.write().unwrap();
    if info_guard.as_ref().is_none() {
        drop(info_guard);
        return update_all(info.clone());
    }

    let now_playing_info = get_now_playing_info();
    if let Some(info) = now_playing_info {
        macro_rules! update_string_info {
            ($key:expr, $field:expr) => {
                if let Some(InfoTypes::String(s)) = info.get($key) {
                    $field = Some(s.clone());
                }
            };
        }

        update_string_info!(
            "kMRMediaRemoteNowPlayingInfoTitle",
            info_guard.as_mut().unwrap().title
        );
        update_string_info!(
            "kMRMediaRemoteNowPlayingInfoArtist",
            info_guard.as_mut().unwrap().artist
        );
        update_string_info!(
            "kMRMediaRemoteNowPlayingInfoAlbum",
            info_guard.as_mut().unwrap().album
        );

        macro_rules! update_float_info {
            ($key:expr, $field:expr) => {
                if let Some(InfoTypes::Number(Number::Floating(f))) = info.get($key) {
                    $field = Some(f.clone());
                }
            };
        }

        update_float_info!(
            "kMRMediaRemoteNowPlayingInfoDuration",
            info_guard.as_mut().unwrap().duration
        );
        update_float_info!(
            "kMRMediaRemoteNowPlayingInfoElapsedTime",
            info_guard.as_mut().unwrap().elapsed_time
        );

        if let Some(InfoTypes::Data(d)) = info.get("kMRMediaRemoteNowPlayingInfoArtworkData") {
            info_guard.as_mut().unwrap().album_cover = ImageReader::new(Cursor::new(d))
                .with_guessed_format()
                .ok()
                .and_then(|img| img.decode().ok());
        }

        info_guard.as_mut().unwrap().info_update_time = info
            .get("kMRMediaRemoteNowPlayingInfoTimestamp")
            .and_then(|f| match f {
                InfoTypes::SystemTime(t) => Some(t.clone()),
                _ => None,
            })
            .or(Some(SystemTime::now()));
    }
}

fn update_app(info: Arc<RwLock<Option<NowPlayingInfo>>>) {
    let mut info_guard = info.write().unwrap();
    if info_guard.as_ref().is_none() {
        drop(info_guard);
        return update_all(info.clone());
    }

    let mut bundle_id = get_now_playing_client_parent_app_bundle_identifier();
    if bundle_id.is_none() {
        bundle_id = get_now_playing_client_bundle_identifier();
    }

    if let Some(id) = bundle_id {
        let bundle_info = get_bundle_info(id.as_str());
        if let Some(info) = bundle_info {
            info_guard.as_mut().unwrap().bundle_id = Some(id);
            info_guard.as_mut().unwrap().bundle_name = Some(info.name);
            info_guard.as_mut().unwrap().bundle_icon = Some(info.icon);
        }
    }
}

impl NowPlaying {
    fn register(&mut self) {
        register_for_now_playing_notifications();

        // initialize with current state
        let info = Arc::clone(&self.info);
        update_all(info.clone());

        macro_rules! add_observer_macro {
            ($notification:expr, $update_fn:expr) => {{
                let info = Arc::clone(&self.info);
                let listeners = Arc::clone(&self.listeners);

                self.observers.push(add_observer($notification, move || {
                    $update_fn(info.clone());
                    for (_, listener) in listeners.clone().lock().unwrap().iter() {
                        listener(info.read().unwrap());
                    }
                }));
            }};
        }

        add_observer_macro!(Notification::NowPlayingApplicationDidChange, update_app);

        add_observer_macro!(Notification::NowPlayingInfoDidChange, update_info);
        // add_observer_macro!(
        //     Notification::NowPlayingApplicationClientStateDidChange,
        //     update_info
        // );
        // add_observer_macro!(Notification::PlaybackQueueContentItemsChanged, update_info);
        // add_observer_macro!(Notification::NowPlayingPlaybackQueueChanged, update_info);

        add_observer_macro!(
            Notification::NowPlayingApplicationIsPlayingDidChange,
            update_state
        );
    }

    /// Creates a new instance of `NowPlaying` and registers for playback notifications.
    ///
    /// This function initializes a new `NowPlaying` object, sets up necessary observers,
    /// and ensures that media metadata is updated upon creation.
    ///
    /// # Returns
    /// - `NowPlaying`: A new instance of the `NowPlaying` struct.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::NowPlaying;
    ///
    /// let now_playing = NowPlaying::new();
    /// ```
    pub fn new() -> Self {
        let mut new_instance = Self {
            info: Arc::new(RwLock::new(None)),
            observers: vec![],
            listeners: Arc::new(Mutex::new(HashMap::new())),
            token_counter: Arc::new(AtomicU64::new(0)),
        };

        new_instance.register();

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
    /// use media_remote::NowPlaying;
    ///
    /// let now_playing: NowPlaying = NowPlaying::new();
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
    /// use media_remote::NowPlaying;
    ///
    /// let now_playing: NowPlaying = NowPlaying::new();
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
    /// use media_remote::NowPlaying;
    ///
    /// let now_playing = NowPlaying::new();
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

    /// Toggles between play and pause states.
    ///
    /// # Returns
    /// - `true` if the command was successfully sent.
    /// - `false` if the command failed.
    ///
    /// # Example
    /// ```rust
    /// use media_remote::NowPlaying;
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.toggle();
    /// ```
    pub fn toggle(&self) -> bool {
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
    /// use media_remote::NowPlaying;
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.play();
    /// ```
    pub fn play(&self) -> bool {
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
    /// use media_remote::NowPlaying;
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.pause();
    /// ```
    pub fn pause(&self) -> bool {
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
    /// use media_remote::NowPlaying;
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.next();
    /// ```
    pub fn next(&self) -> bool {
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
    /// use media_remote::NowPlaying;
    ///
    /// let now_playing = NowPlaying::new();
    /// now_playing.previous();
    /// ```
    pub fn previous(&self) -> bool {
        send_command!(self, Command::PreviousTrack)
    }
}

impl Drop for NowPlaying {
    fn drop(&mut self) {
        unregister_for_now_playing_notifications();

        while let Some(observer) = self.observers.pop() {
            remove_observer(observer);
        }
    }
}
