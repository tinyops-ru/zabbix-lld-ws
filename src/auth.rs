use serde::Deserialize;
use serde::Serialize;

use crate::errors::OperationError;
use crate::http::send_post_request;
use crate::types::StringResult;
use crate::zabbix::JSONRPC;

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
    result: String
}

pub fn login_to_zabbix_api(client: &reqwest::blocking::Client, api_endpoint: &str,
                           username: &str, password: &str) -> StringResult {
    let auth_request = AuthRequest {
        jsonrpc: JSONRPC.to_string(),
        method: "user.login".to_string(),
        params: RequestParams {
            user: username.to_string(), password: password.to_string()
        },
        id: 1,
        auth: None
    };

    match send_post_request(&client, api_endpoint, auth_request) {
        Ok(response) => {
            match serde_json::from_str::<AuthResponse>(&response) {
                Ok(auth_response) => {
                    debug!("auth token: {}", auth_response.result);
                    Ok(String::from(auth_response.result))
                }
                Err(_) => {
                    error!("unsupported auth response");
                    Err(OperationError::Error)
                }
            }
        }
        Err(_) => {
            error!("authentication error");
            Err(OperationError::Error)
        }
    }
}