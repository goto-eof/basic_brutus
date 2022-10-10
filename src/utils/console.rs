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
    println!("-u  = username");
    println!("     Ex. bill.gates");
    println!("-uu = username file path");
    println!("     Ex. /directory/usernames.txt");
    println!("-v  = verbose");
    println!("     Ex. true");
    println!("-d  = dictionary file path");
    println!("     Ex. /dictionary/passwords.txt");
    println!("-t  = uri");
    println!("     Ex. https://website.com");
    println!("====================================");
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
    println!("===== Password found :) ======");
    println!("=======  thread id {}  =======", idx);
    println!("message:  {}", message);
    println!("uri:      {}", uri);
    println!("username: {}", username);
    println!("password: {}", password);
    println!("base64:   {}", base64);
    println!("===============================");
}
