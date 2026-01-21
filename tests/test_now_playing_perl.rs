use media_remote::{prelude::*, NowPlayingInfo};

fn print_info(info: &NowPlayingInfo) {
    println!("Is Playing: {:?}", info.is_playing);

    println!("Title: {:?}", info.title);
    println!("Artist: {:?}", info.artist);
    println!("Album: {:?}", info.album);
    if let Some(album_cover) = &info.album_cover {
        println!(
            "Album Cover: {:?}x{:?}px",
            album_cover.width(),
            album_cover.height()
        );
    }
    println!("Elapsed Time: {:?}", info.elapsed_time);
    println!("Duration: {:?}", info.duration);
    println!("Bundle ID: {:?}", info.bundle_id);
    println!("Bundle Name: {:?}", info.bundle_name);
    if let Some(bundle_icon) = &info.bundle_icon {
        println!(
            "Bundle Icon: {:?}x{:?}px",
            bundle_icon.width(),
            bundle_icon.height()
        );
    }
}

#[test]
fn test_now_playing_perl_get_info() {
    let now_playing = NowPlayingPerl::new();

    // Give it some time to start the process
    std::thread::sleep(std::time::Duration::from_secs(2));

    let guard = now_playing.get_info();
    let info = guard.as_ref();

    if let Some(info) = info {
        print_info(info);
    }
}

#[test]
fn test_now_playing_perl_send_command() {
    let now_playing: NowPlayingPerl = NowPlayingPerl::new();

    // Give it some time to start the process
    std::thread::sleep(std::time::Duration::from_secs(2));

    now_playing.pause();
    now_playing.play();
    now_playing.toggle();
    now_playing.previous();
    now_playing.next();
}

#[test]
fn test_now_playing_perl_subscribe() {
    let now_playing: NowPlayingPerl = NowPlayingPerl::new();

    // Give it some time to start the process
    std::thread::sleep(std::time::Duration::from_secs(2));

    let token = now_playing.subscribe(|info| {
        if let Some(info) = info.as_ref() {
            print_info(info);
        }
    });

    now_playing.unsubscribe(token);
}

use std::sync::{Arc, Condvar, Mutex};

#[test]
#[ignore]
fn test_now_playing_perl_loop() {
    let now_playing = NowPlayingPerl::new();

    now_playing.subscribe(|info| {
        if let Some(info) = info.as_ref() {
            print_info(info);
        }
    });

    // Blocks forever
    let pair = Arc::new((Mutex::new(()), Condvar::new()));
    let (lock, cvar) = &*pair;

    let guard = lock.lock().unwrap();
    let _unused = cvar.wait(guard).unwrap();
}
