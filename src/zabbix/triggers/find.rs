use serde_derive::Deserialize;

use crate::zabbix::triggers::ZabbixTrigger;

#[derive(Deserialize)]
pub struct TriggerSearchResponse {
    pub result: Vec<ZabbixTrigger>
}