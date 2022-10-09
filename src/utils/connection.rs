use async_std::task;
use reqwest::{Response, StatusCode};

use super::logger::log;

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
) -> Result<BruteResponse, String> {
    let mut res = task::block_on(async move {
        let auth_base = format!("Base {}", auth);
        request(uri, &auth_base).await
    });

    while res.is_err() {
        log(&format!(
            "[KO] -> Error. Retrying username:[{}], password[{}]...",
            username, password
        ));

        *failed_and_restored_requests = (*failed_and_restored_requests) + 1;
        let auth_base = format!("Base {}", auth);
        res = task::block_on(async move { request(uri, &auth_base.as_str()).await });
    }

    if res.unwrap().status().is_success() {
        return Ok(BruteResponse::new(
            "Let's login now!".to_string(),
            username.to_string(),
            password.to_string(),
            uri.to_string(),
            auth.to_string(),
        ));
    }
    return Err("Error".to_string());
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
}

impl BruteResponse {
    fn new(
        message: String,
        username: String,
        password: String,
        uri: String,
        base64: String,
    ) -> Self {
        Self {
            message,
            username,
            password,
            uri,
            base64,
        }
    }
}
