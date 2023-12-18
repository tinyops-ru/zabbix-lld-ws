use serde::Deserialize;

pub mod create;
pub mod find;

#[derive(Deserialize, Clone, Debug)]
pub struct ZabbixTrigger {
    pub name: String
}







