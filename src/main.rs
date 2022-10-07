pub mod utils {
    pub mod connection;
    pub mod console;
    pub mod core;
    pub mod file;
    pub mod parser;
}

use std::collections::HashMap;
use std::env;
use std::process;
use utils::console::{print_welcome, read_line};
use utils::core::execute_command;

use utils::parser::BasicBrutusError;
use utils::parser::{is_error, parse};

extern crate openssl_probe;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let args: Vec<String> = env::args().collect();
    let args_str = args.join(" ");

    // TODO validation pre-parsing

    // parsing
    let parsed_command_result: Result<HashMap<String, String>, BasicBrutusError> = match args.len()
    {
        1 => {
            print_welcome();
            let command = read_line().unwrap();
            parse(&command)
        }
        _ => parse(&args_str),
    };

    // validation post-parsing
    match is_error(&parsed_command_result) {
        Some(err) => {
            println!("{}: {}", err.code, err.description);
            process::exit(i32::from(err.code))
        }
        None => (),
    }

    // command execution
    execute_command(parsed_command_result.ok().unwrap());
}
