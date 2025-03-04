use media_remote::{NowPlaying, NowPlayingInfo};

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
fn test_now_playing_get_info() {
    let now_playing = NowPlaying::new();

    let guard = now_playing.get_info();
    let info = guard.as_ref();

    if let Some(info) = info {
        print_info(info);
    }
}

#[test]
fn test_now_playing_send_command() {
    let now_playing: NowPlaying = NowPlaying::new();

    now_playing.pause();
    now_playing.play();
    now_playing.toggle();
    now_playing.previous();
    now_playing.next();
}

#[test]
#[ignore]
fn test_now_playing_loop() {
    let now_playing = NowPlaying::new();

    loop {
        {
            let guard = now_playing.get_info();
            let info = guard.as_ref();

            if let Some(info) = info {
                print_info(info);
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
