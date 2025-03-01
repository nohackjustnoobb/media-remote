use block2::Block;
use core::ffi::c_int;
use dispatch2::ffi::dispatch_queue_s;
use objc2_core_foundation::CFDictionary;
use objc2_foundation::NSString;
use std::ptr::NonNull;

use crate::types::Id;

#[link(name = "MediaRemote", kind = "framework")]
extern "C" {

    pub fn MRMediaRemoteGetNowPlayingApplicationIsPlaying(
        queue: *mut dispatch_queue_s,
        block: &Block<dyn Fn(c_int)>,
    );

    pub fn MRMediaRemoteGetNowPlayingClient(
        queue: *mut dispatch_queue_s,
        block: &Block<dyn Fn(Id)>,
    );

    pub fn MRMediaRemoteGetNowPlayingApplicationPID(
        queue: *mut dispatch_queue_s,
        block: &Block<dyn Fn(c_int)>,
    );

    pub fn MRMediaRemoteGetNowPlayingInfo(
        queue: *mut dispatch_queue_s,
        block: &Block<dyn Fn(NonNull<CFDictionary>)>,
    );

    pub fn MRNowPlayingClientGetParentAppBundleIdentifier(id: Id) -> *const NSString;

    pub fn MRNowPlayingClientGetBundleIdentifier(id: Id) -> *const NSString;

}
