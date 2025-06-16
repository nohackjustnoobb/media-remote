use std::{
    io::Write,
    process::{Command, Stdio},
};

use serde_json::Value;

static SCRIPT: &[u8] = include_bytes!("nowPlaying.jxa");

pub fn get_raw_info() -> Option<Value> {
    let mut child = Command::new("osascript")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-l")
        .arg("JavaScript")
        .spawn()
        .expect("Failed to spawn command");

    child
        .stdin
        .as_mut()
        .expect("Failed to open stdin")
        .write_all(SCRIPT)
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");
    if !output.status.success() {
        return None;
    }

    serde_json::from_slice(&output.stdout).ok()
}
