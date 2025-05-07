use media_remote::get_raw_info;
use serde_json::to_string_pretty;

fn main() {
    let info = get_raw_info();

    if let Some(info) = info {
        println!("{}", to_string_pretty(&info).unwrap());
    } else {
        println!("Failed to get raw info");
    }
}
