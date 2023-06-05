use std::collections::HashMap;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

use crate::config::ZabbixApiVersion;
use crate::http::send_post_request;
use crate::types::OperationResult;
use crate::zabbix::JSONRPC;

#[derive(Serialize)]
struct AuthRequest {
    jsonrpc: String,
    method: String,
    params: HashMap<String, String>,
    id: i8,
    auth: Option<String>
}

#[derive(Deserialize)]
struct AuthResponse {
    result: String
}

const AUTH_REQUEST_METHOD: &str = "user.login";

pub fn login_to_zabbix_api(api_version: &ZabbixApiVersion,
                           client: &reqwest::blocking::Client, api_endpoint: &str,
                           username: &str, password: &str) -> OperationResult<String> {

    let auth_request = AuthRequest {
        jsonrpc: JSONRPC.to_string(),
        method: AUTH_REQUEST_METHOD.to_string(),
        params: get_params(&api_version, username, password),
        id: 1,
        auth: None
    };

    let response = send_post_request(&client, api_endpoint, auth_request)
        .context("api communication error")?;

    let auth_response = serde_json::from_str::<AuthResponse>(&response)
        .context("authentication error")?;

    Ok(String::from(auth_response.result))
}

fn get_params(api_version: &ZabbixApiVersion,
                 username: &str, password: &str) -> HashMap<String, String> {

    let username_field = match api_version {
        ZabbixApiVersion::V6 => "username",
        ZabbixApiVersion::V5 => "user"
    };

    HashMap::from([
        (username_field.to_string(), username.to_string()),
        ("password".to_string(), password.to_string()),
    ])
}