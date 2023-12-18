use anyhow::Context;
use reqwest::blocking::Client;

use crate::config::ZabbixConfig;
use crate::types::OperationResult;
use crate::zabbix::hosts::find_hosts;
use crate::zabbix::items::find_zabbix_items;
use crate::zabbix::webscenarios::find_web_scenarios;
use crate::ZabbixEntities;

pub fn find_zabbix_objects(client: &Client, zabbix_config: &ZabbixConfig,
                       auth_token: &str, item_key_search_mask: &str) ->
                       OperationResult<ZabbixEntities> {

    let items = find_zabbix_items(&client, &zabbix_config.api.endpoint,
                                  &auth_token, item_key_search_mask).context("unable to find zabbix items")?;

    debug!("received items:");

    let web_scenarios = find_web_scenarios(&client, &zabbix_config.api.endpoint, &auth_token).context("unable to find web scenarios")?;

    debug!("web scenarios have been obtained");

    let host_ids: Vec<String> = items.iter()
        .map(|item| item.hostid.to_string()).collect();

    let hosts = find_hosts(&client, &zabbix_config.api.endpoint, &auth_token, host_ids).context("unable to find hosts")?;

    Ok(
        ZabbixEntities {
            items,
            web_scenarios,
            hosts
        }
    )
}