use media_remote::get_now_playing_application_is_playing;

#[test]
fn test_get_now_playing_application_is_playing() {
    let result = get_now_playing_application_is_playing();

    assert!(
        result.is_some(),
        "Expected Some(true) or Some(false), but got None (possible timeout)"
    );

    println!("Now playing status: {}", result.unwrap());
}

use media_remote::get_now_playing_application_pid;

#[test]
fn test_get_now_playing_application_pid() {
    match get_now_playing_application_pid() {
        Some(ptr) => println!("Now playing application PID: {:?}", ptr),
        None => println!("No application found or timed out."),
    }
}

use media_remote::get_now_playing_client_bundle_identifier;

#[test]
fn test_get_now_playing_client_bundle_identifier() {
    match get_now_playing_client_bundle_identifier() {
        Some(id) => println!("Bundle identifier: {}", id),
        None => println!("No bundle found or timed out."),
    }
}

use media_remote::get_now_playing_client_parent_app_bundle_identifier;

#[test]
fn test_get_now_playing_client_parent_app_bundle_identifier() {
    match get_now_playing_client_parent_app_bundle_identifier() {
        Some(id) => println!("Client parent app bundle identifier: {}", id),
        None => println!("No parent app found or timed out."),
    }
}

use media_remote::get_now_playing_client;

#[test]
fn test_get_now_playing_client() {
    match get_now_playing_client() {
        Some(ptr) => println!("Now playing client: {:?}", ptr),
        None => println!("No client found or timed out."),
    }
}

use media_remote::get_now_playing_info;

#[test]
fn test_get_now_playing_info() {
    match get_now_playing_info() {
        Some(dict) => {
            println!("Now playing info: ");
            for (key, value) in dict.iter() {
                println!("{}: {}", key, value);
            }
        }
        None => println!("No info found or timed out."),
    }
}
