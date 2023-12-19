use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::zabbix::items::ZabbixItem;

#[derive(Serialize)]
pub struct ItemSearchParams {
    pub sortfield: String,
    pub search: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ItemSearchResponse {
    pub result: Option<Vec<ZabbixItem>>
}

