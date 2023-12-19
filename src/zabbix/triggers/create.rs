use serde_derive::{Deserialize, Serialize};

use crate::zabbix::ZabbixError;

#[derive(Serialize)]
pub struct TriggerCreateRequestParams {
    pub description: String,
    pub expression: String,
    pub priority: String,
    pub url: String
}

#[derive(Deserialize)]
pub struct CreateTriggerResponse {
    pub error: Option<ZabbixError>
}