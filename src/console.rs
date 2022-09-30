pub mod console {

    pub fn read_line() -> Option<String> {
        let mut result = String::new();
        while std::io::stdin().read_line(&mut result).is_err() {
            println!("I/O error. Retrying...");
        }
        return Some(result.trim().to_owned());
    }
}
