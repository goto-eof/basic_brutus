pub fn read_line() -> Option<String> {
    let mut result = String::new();
    while std::io::stdin().read_line(&mut result).is_err() {
        println!("I/O error. Retrying...");
    }
    return Some(result.trim().to_owned());
}

pub fn print_welcome() {
    println!("====================================");
    println!("=========== Basic Brutus ==========");
    print_help();
}

pub fn print_help() {
    println!("====================================");
    println!("Help:");
    println!("====================================");
    println!("--help = help");
    println!("====================================");
    println!("-u = username");
    println!("-d = dictionary path.");
    println!("     Ex. /dictionary/filename.txt");
    println!("-t = uri. Ex. https://website.com");
    println!("====================================");
    println!("all parameters are mandatory");
    println!("====================================");
}
pub fn print_error(err_message: String) {
    println!("{}", err_message);
}

pub fn print_result(
    idx: usize,
    message: &str,
    uri: &str,
    username: &str,
    password: &str,
    base64: &str,
) {
    println!("==============================");
    println!("==============================");
    println!("======= Password found =======");
    println!("=======  thread id {}  =======", idx);
    println!("message:  {}", message);
    println!("uri:      {}", uri);
    println!("username: {}", username);
    println!("password: {}", password);
    println!("base64:   {}", base64);
    println!("===============================");
}
