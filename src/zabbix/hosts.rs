use serde::Deserialize;
use serde::Serialize;

use crate::zabbix::ZabbixError;

#[derive(Serialize)]
pub struct SearchRequestParams {
    #[serde(rename = "hostids")]
    pub host_ids: Vec<String>
}

#[derive(Deserialize)]
pub struct SearchResponse {
    pub result: Option<Vec<ZabbixHost>>,
    pub error: Option<ZabbixError>
}

#[derive(Deserialize)]
pub struct ZabbixHost {
    #[serde(rename = "hostid")]
    pub host_id: String,
    pub host: String
}