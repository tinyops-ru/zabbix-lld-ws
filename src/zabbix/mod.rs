use serde::Deserialize;
use serde::Serialize;

pub mod triggers;
pub mod items;
pub mod auth;
pub mod hosts;
pub mod webscenarios;
pub mod find;
pub mod service;

pub const JSONRPC: &str = "2.0";

pub const UNSUPPORTED_RESPONSE_MESSAGE: &str = "unsupported zabbix api response";
pub const ZABBIX_API_COMMUNICATION_ERROR: &str = "zabbix api communication error";

#[derive(Serialize)]
pub struct ZabbixRequest<P: Serialize> {
    pub jsonrpc: String,
    pub method: String,
    pub params: P,
    pub auth: String,
    pub id: i8
}

impl<P: Serialize> ZabbixRequest<P> {
    pub fn new(method: &str, params: P, auth_token: &str) -> ZabbixRequest<P> {
        ZabbixRequest {
            jsonrpc: JSONRPC.to_string(),
            method: method.to_string(),
            params,
            auth: auth_token.to_string(),
            id: 1
        }
    }
}

#[derive(Deserialize)]
pub struct ZabbixError {
    pub code: i32,
    pub message: String,
    pub data: String
}

pub fn log_zabbix_error(zabbix_error: &Option<ZabbixError>) {
    match zabbix_error {
        Some(error) => {
            error!("error {}", error.code);
            error!("- message: '{}'", error.message);
            error!("- data: '{}'", error.data);
        }
        None => {}
    }
}
