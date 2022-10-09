use async_std::task;
use reqwest::Response;

pub fn http_req(
    uri: &str,
    auth: &str,
    username: &str,
    password: &str,
) -> Result<BruteResponse, String> {

    let mut res = task::block_on(async move {
       request(uri, auth).await  
    });


    while res.is_err(){
        println!("[KO] -> Error. Retrying username:[{}], password[{}]...", username, password);
         res = task::block_on(async move {
            request(uri, auth).await  
         });
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

async fn request(uri: &str, auth: &str)  -> Result<Response, reqwest::Error> {
    use reqwest::header::USER_AGENT;
    reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
            .unwrap()
            .get(uri)
            .header(USER_AGENT, "Basic Brutus")
            .header("Authorization", format!("Base {}", auth))
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
