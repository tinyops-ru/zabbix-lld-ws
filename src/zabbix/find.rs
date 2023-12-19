use anyhow::Context;
use reqwest::blocking::Client;

use crate::config::ZabbixConfig;
use crate::types::OperationResult;
use crate::zabbix::service::ZabbixService;
use crate::ZabbixEntities;

pub fn find_zabbix_objects(zabbix_service: &impl ZabbixService, client: &Client, zabbix_config: &ZabbixConfig,
                       auth_token: &str, item_key_search_mask: &str) ->
                       OperationResult<ZabbixEntities> {

    let items = zabbix_service.find_items(&auth_token, item_key_search_mask).context("unable to find zabbix items")?;

    debug!("received items:");

    let web_scenarios = zabbix_service.find_web_scenarios(&auth_token)
        .context("unable to find web scenarios")?;

    debug!("web scenarios have been obtained");

    let host_ids: Vec<String> = items.iter()
        .map(|item| item.hostid.to_string()).collect();

    let hosts = zabbix_service.find_hosts(&auth_token, host_ids).context("unable to find hosts")?;

    Ok(
        ZabbixEntities {
            items,
            web_scenarios,
            hosts
        }
    )
}