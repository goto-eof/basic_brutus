use async_std::task;

pub fn http_req(    uri: &str,
    auth: &str,
    username: &str,
    password: &str) -> Result<BruteResponse, String> {
        use reqwest::header::USER_AGENT;
    let hosts: Vec<String> = vec![];
  let res =   task::block_on(async move {
    
        match reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
            .unwrap()
            .get(uri)
            .header(USER_AGENT, "Basic Brutus")
        .header("Authorization", format!("Base {}", auth))

            .send()
            .await
        {
            Ok(x) => x,
            Err(x) => panic!(),
        }
    });
    if res.status().is_success() {
        return Ok(BruteResponse::new(
            "yahoo!".to_string(),
            username.to_string(),
            password.to_string(),
            uri.to_string(),
            auth.to_string(),
        ));
    }
    return Err("Nada".to_string());
}


pub fn http_req_old(
    uri: &str,
    auth: &str,
    username: &str,
    password: &str,
) -> Result<BruteResponse, String> {
    use reqwest::header::USER_AGENT;
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(uri)
        .header(USER_AGENT, "Basic Brutus")
        .header("Authorization", format!("Base {}", auth))
        .send()
        .unwrap();
    if res.status().is_success() {
        return Ok(BruteResponse::new(
            "yahoo!".to_string(),
            username.to_string(),
            password.to_string(),
            uri.to_string(),
            auth.to_string(),
        ));
    }
    return Err("Nada".to_string());
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
