pub mod utils {
    pub mod connection;
    pub mod console;
    pub mod core;
    pub mod file;
    pub mod logger;
    pub mod parser;
}

use std::collections::HashMap;
use std::env;
use std::process;
use utils::console::{print_welcome, read_line};
use utils::core::execute_command;

use utils::parser::BasicBrutusError;
use utils::parser::{is_error, parse};

use dotenv;

fn main() {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let args_str = args.join(" ");

    let original_command: String;

    // parsing
    let parsed_command_result: Result<HashMap<String, String>, BasicBrutusError> = match args.len()
    {
        1 => {
            print_welcome();
            let command = read_line().unwrap();
            original_command = command.to_string();
            parse(&command)
        }
        _ => {
            original_command = args_str.to_string();
            parse(&args_str)
        }
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
    execute_command(parsed_command_result.ok().unwrap(), original_command);
}
