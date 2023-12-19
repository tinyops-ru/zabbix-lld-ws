use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ZabbixItem {
    pub name: String,
    pub key_: String,
    pub hostid: String
}