use chrono::{Timelike, Utc};

pub fn log(message: &str) {
    println!("{} {}", time(), message);
}

fn time() -> String {
    let time = Utc::now();
    format!("[{}:{}:{}]", time.hour(), time.minute(), time.second())
}
