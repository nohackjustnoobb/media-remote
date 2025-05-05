use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, RwLockReadGuard,
    },
};

use crate::NowPlayingInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListenerToken(u64);

pub trait Subscription {
    fn get_info(&self) -> RwLockReadGuard<'_, Option<NowPlayingInfo>>;

    fn get_token_counter(&self) -> Arc<AtomicU64>;

    fn get_listeners(
        &self,
    ) -> Arc<
        Mutex<
            HashMap<
                ListenerToken,
                Box<dyn Fn(RwLockReadGuard<'_, Option<NowPlayingInfo>>) + Send + Sync>,
            >,
        >,
    >;

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
    /// use media_remote::prelude::*;
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
    fn subscribe<F: Fn(RwLockReadGuard<'_, Option<NowPlayingInfo>>) + Send + Sync + 'static>(
        &self,
        listener: F,
    ) -> ListenerToken {
        listener(self.get_info());

        let token = ListenerToken(self.get_token_counter().fetch_add(1, Ordering::Relaxed));

        self.get_listeners()
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
    /// use media_remote::prelude::*;
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
    fn unsubscribe(&self, token: ListenerToken) {
        let binding = self.get_listeners();
        let mut listeners = binding.lock().unwrap();
        listeners.remove(&token);
    }
}
