use media_remote::{get_bundle_info, get_now_playing_client_parent_app_bundle_identifier};

#[test]
fn test_get_bundle_info() {
    let bundle_id = get_now_playing_client_parent_app_bundle_identifier();

    if let Some(id) = bundle_id {
        let info = get_bundle_info(id.as_str());

        if let Some(info) = info {
            println!("Now playing client parent app info:");
            println!("Name: {}", info.name);
            println!("Icon: {}x{}px", info.icon.width(), info.icon.height());
        } else {
            println!("Failed to get now playing client parent app info.");
        }
    }
}

use media_remote::{
    add_observer, register_for_now_playing_notifications, remove_observer,
    unregister_for_now_playing_notifications, Notification,
};

#[test]
fn test_observer() {
    register_for_now_playing_notifications();
    println!("Registered for now playing notifications.");

    let observer = add_observer(
        Notification::NowPlayingApplicationIsPlayingDidChange,
        move || {
            println!("Now playing status changed.");
        },
    );
    println!("Observer added.");

    remove_observer(observer);
    println!("Observer removed.");

    unregister_for_now_playing_notifications();
    println!("Unregistered for now playing notifications.");
}
