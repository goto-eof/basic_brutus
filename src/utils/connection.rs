use std::collections::HashMap;

use super::logger::error;
use async_std::task;
use reqwest::{Response, StatusCode};

pub fn is_basic_protected(uri: &str) -> Option<()> {
    let empty = "";
    let res = task::block_on(async move { request(uri, empty).await });
    match res {
        Ok(data) => match data.status() {
            StatusCode::UNAUTHORIZED => Some(()),
            _ => None,
        },
        Err(_) => Some(()),
    }
}

pub async fn http_req(
    uri: &str,
    auth: &str,
    username: &str,
    password: &str,
    failed_and_restored_requests: &mut i32,
    max_failed_requests: i32,
    parsed_command: &HashMap<String, String>,
) -> Result<BruteResponse, BruteFailedMatchResponse> {
    let mut res = task::block_on(async move {
        let auth_base = format!("Base {}", auth);
        request(uri, &auth_base).await
    });

    let mut count_failed_requests = 0;

    while res.is_err() && (max_failed_requests == -1 || count_failed_requests < max_failed_requests)
    {
        if parsed_command.get("max_failed_requests").is_none() {
            error(&format!(
                "[KO] -> Error. Retrying username:[{}], password[{}]...",
                username, password
            ));
        }

        count_failed_requests = count_failed_requests + 1;

        *failed_and_restored_requests = (*failed_and_restored_requests) + 1;
        let auth_base = format!("Base {}", auth);
        res = task::block_on(async move { request(uri, &auth_base.as_str()).await });
    }

    if res.is_err() {
        let message = format!(
            "Attempts: {}, Error {}",
            count_failed_requests,
            res.err().unwrap()
        );
        let response = BruteFailedMatchResponse::new(count_failed_requests, message);
        return Err(response);
    }

    if res.unwrap().status().is_success() {
        return Ok(BruteResponse::new(
            "Let's login now!".to_string(),
            username.to_string(),
            password.to_string(),
            uri.to_string(),
            auth.to_string(),
            count_failed_requests,
        ));
    };

    let message = "Username and password does not match".to_string();
    let response = BruteFailedMatchResponse::new(count_failed_requests, message);
    return Err(response);
}

async fn request(uri: &str, auth: &str) -> Result<Response, reqwest::Error> {
    // use reqwest::header::USER_AGENT;
    reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()
        .unwrap()
        .head(uri)
        // .header(USER_AGENT, "Basic Brutus")
        .header("Authorization", auth)
        .send()
        .await
}

pub struct BruteResponse {
    pub message: String,
    pub username: String,
    pub password: String,
    pub uri: String,
    pub base64: String,
    pub attempts: i32,
}

impl BruteResponse {
    fn new(
        message: String,
        username: String,
        password: String,
        uri: String,
        base64: String,
        attempts: i32,
    ) -> Self {
        Self {
            message,
            username,
            password,
            uri,
            base64,
            attempts,
        }
    }
}

#[derive(Debug)]
pub struct BruteFailedMatchResponse {
    pub attempts: i32,
    pub message: String,
}

impl BruteFailedMatchResponse {
    fn new(attempts: i32, message: String) -> Self {
        Self { attempts, message }
    }
}
