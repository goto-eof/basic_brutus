use chrono::{Timelike, Utc};
use colored::Colorize;

pub fn log(message: &str) {
    println!("{} {}", time(), message);
}

pub fn error(message: &str) {
    let colored_string = format!("[ERROR] {}", message).red();
    println!("{} {}", time(), colored_string);
}

pub fn warn(message: &str) {
    let colored_string = &format!("[WARNING] {}", message).yellow();
    println!("{} {}", time(), colored_string);
}

pub fn info(message: &str) {
    let colored_string = &format!("[INFO] {}", message).blue();
    println!("{} {}", time(), colored_string);
}

fn time() -> String {
    let time = Utc::now();
    format!("[{}:{}:{}]", time.hour(), time.minute(), time.second())
}
