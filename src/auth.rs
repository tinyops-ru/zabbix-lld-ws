pub mod auth {
    use std::io;
    use serde::Serialize;
    use serde::Deserialize;
    use std::error::Error;
    use crate::zabbix::zabbix::JSONRPC;

    #[derive(Serialize)]
    struct AuthRequest {
        jsonrpc: String,
        method: String,
        params: RequestParams,
        id: i8,
        auth: Option<String>
    }

    #[derive(Serialize)]
    struct RequestParams {
        user: String,
        password: String
    }

    #[derive(Deserialize)]
    struct AuthResponse {
        jsonrpc: String,
        result: String,
        id: i8
    }

    pub fn login(api_endpoint: &str, username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
        let auth_request = AuthRequest {
            jsonrpc: JSONRPC.to_string(),
            method: "user.login".to_string(),
            params: RequestParams {
                user: username.to_string(), password: password.to_string()
            },
            id: 1,
            auth: None
        };

        let client = reqwest::blocking::Client::new();

        let request_body = serde_json::to_string(&auth_request).unwrap();

        let response = client.post(api_endpoint)
                                       .body(request_body)
                                       .header("Content-Type", "application/json")
                                       .send()?;

        let response_status = response.status();
        let response_text = response.text().unwrap();

        println!("{}", response_text);

        if response_status != reqwest::StatusCode::OK {
            println!("{}", response_text);
            panic!("unexpected server response code {}", response_status)
        }

        let auth_response: AuthResponse = serde_json::from_str(&response_text).unwrap();

        Ok(String::from(auth_response.result))
    }
}
