use std::process::Command;

use serde_json::Value;

pub fn get_raw_info() -> Option<Value> {
    let output = Command::new("osascript")
        .arg("-l")
        .arg("JavaScript")
        .arg("src/low_level/nowPlaying.jxa")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        return None;
    }

    serde_json::from_slice(&output.stdout).ok()
}
