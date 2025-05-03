use serde_json::{to_string_pretty, Value};
use std::process::Command;

fn main() {
    let output = Command::new("osascript")
        .arg("-l")
        .arg("JavaScript")
        .arg("src/nowPlaying.jxa")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let data: Value = serde_json::from_slice(&output.stdout).expect("Failed to parse JSON");
        println!(
            "{}",
            to_string_pretty(&data).expect("Failed to convert to pretty JSON")
        );
    } else {
        panic!("Failed to execute command");
    }
}
