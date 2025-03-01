/// The "C" function declarations below is copied from these repositories:
/// 1. https://github.com/billziss-gh/EnergyBar/blob/master/src/System/NowPlaying.m
/// 2. https://github.com/davidmurray/ios-reversed-headers/blob/master/MediaRemote/MediaRemote.h
/// 3. https://github.com/PrivateFrameworks/MediaRemote
use block2::Block;
use core::ffi::c_int;
use dispatch2::ffi::dispatch_queue_s;
use objc2_core_foundation::CFDictionary;
use objc2_foundation::NSString;
use std::{ffi::c_double, ptr::NonNull};

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

    pub fn MRMediaRemoteSendCommand(command: c_int, userInfo: Id) -> bool;

    pub fn MRMediaRemoteSetPlaybackSpeed(speed: c_int);

    pub fn MRMediaRemoteSetElapsedTime(elapsedTime: c_double);
}
