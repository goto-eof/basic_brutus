mod connection;
mod console;
mod file;
mod parser;

use connection::{http_req, BruteResponse};
use console::read_line;
use file::file_to_vec;
use parser::parse;
use rayon::prelude::*;
use std::collections::HashMap;
use std::process;

fn main() {
    println!("====================================");
    println!("=========== Basic Brutus ==========");
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
        let filename = unwrappedd.get("dictionary").unwrap();

        let vec = file_to_vec(&filename).unwrap();

        let chunked_items: Vec<Vec<String>> = vec
            .chunks(vec.len() / 7)
            .into_iter()
            .map(|chunk| chunk.to_vec())
            .collect();

        let result = chunked_items
            .par_iter()
            .enumerate()
            .map(|(i, chunck)| process_dictionary(i, username, uri, chunck))
            .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 });

        match result.unwrap() {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}

fn process_dictionary(
    idx: usize,
    username: &str,
    uri: &str,
    arc: &Vec<String>,
) -> Result<BruteResponse, String> {
    for password in arc.to_vec() {
        let auth = base64::encode(format!("{}:{}", &username, &password));
        let status = http_req(uri, &auth, username, &password);
        if status.is_ok() {
            let result = status.unwrap();
            println!("==============================");
            println!("==============================");
            println!("======= Password found =======");
            println!("=======  thread id {}  =======", idx);
            println!("message:  {}", result.message);
            println!("uri:      {}", result.uri);
            println!("username: {}", result.username);
            println!("password: {}", result.password);
            println!("base64:   {}", result.base64);
            println!("===============================");
            process::exit(0x0100);
        } else {
            println!("thread {} | {}:{} ", idx, &username, &password);
        }
    }
    Err("No password found".to_string())
}
