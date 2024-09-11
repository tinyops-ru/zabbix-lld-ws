use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};
use zabbix_api::host::ZabbixHostTag;

/// Zabbix API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/item/object#item
#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ZabbixItemConfig {
    pub name_template: String,
    pub key_template: String,
    pub interface_id: String,
    pub delay: String,

    /// Item type:
    /// 0 - Zabbix agent
    /// 7 - Zabbix agent (agent)
    pub r#type: u8,

    pub value_type: u8,

    #[serde(default = "get_empty_tag_vec")]
    pub tags: Vec<ZabbixHostTag>
}

impl Display for ZabbixItemConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name-template: '{}', key-template: '{}', interface-id: '{}', delay: '{}', type: {}, value-type: {}",
               self.name_template, self.key_template, self.interface_id, self.delay, self.r#type, self.value_type)
    }
}

fn get_empty_tag_vec() -> Vec<ZabbixHostTag> {
    Vec::new()
}