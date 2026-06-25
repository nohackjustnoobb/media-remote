use media_remote::prelude::*;
use std::time::Duration;

fn main() {
    let now_playing = NowPlayingPerl::new();

    // Give the adapter process a moment to start streaming.
    std::thread::sleep(Duration::from_secs(2));

    let guard = now_playing.get_info();
    match guard.as_ref() {
        Some(info) => {
            println!("Is Playing:    {:?}", info.is_playing);
            println!("Title:         {:?}", info.title);
            println!("Artist:        {:?}", info.artist);
            println!("Album:         {:?}", info.album);
            #[cfg(feature = "artwork")]
            if let Some(cover) = &info.album_cover {
                println!("Album Cover:   {}x{}px", cover.width(), cover.height());
            }
            println!("Elapsed Time:  {:?}", info.elapsed_time);
            println!("Duration:      {:?}", info.duration);
            println!("Playback Rate: {:?}", info.playback_rate);
            println!("Bundle ID:     {:?}", info.bundle_id);
        }
        None => println!("No now playing info available."),
    }
}
