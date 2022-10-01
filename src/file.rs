use std::fs::File;
use std::io::prelude::*;

pub fn file_to_vec(filename: &str) -> std::io::Result<Vec<String>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let split: Vec<String> = contents
        .replace("\r\n", "\n")
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    Ok(split)
}
