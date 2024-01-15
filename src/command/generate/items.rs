use anyhow::{anyhow, Context};
use regex::Regex;

use crate::config::ZabbixConfig;
use crate::template::{get_template_vars, process_template_string};
use crate::types::EmptyResult;
use crate::zabbix::service::ZabbixService;

pub fn generate_web_scenarios_and_triggers_for_items(zabbix_service: &impl ZabbixService,
                                                     zabbix_config: &ZabbixConfig,
                                                     item_key_search_mask: &str) -> EmptyResult {

    let auth_token = zabbix_service.get_session(&zabbix_config.api.username,
                                        &zabbix_config.api.password).context("zabbix auth error")?;

    let items = zabbix_service.find_items(
        &auth_token, item_key_search_mask).context("unable to find zabbix items")?;

    debug!("received items:");

    let web_scenarios = zabbix_service.find_web_scenarios(
        &auth_token, &zabbix_config.scenario.key_starts_with).context("unable to find web scenarios")?;

    debug!("web scenarios have been obtained");

    let host_ids: Vec<String> = items.iter()
        .map(|item| item.hostid.to_string()).collect();

    let hosts = zabbix_service.find_hosts(&auth_token, host_ids)
                                                            .context("unable to find hosts")?;

    let pattern_start = "^".to_string() + item_key_search_mask;
    let pattern = pattern_start + "\\[(.*)\\]$";

    let url_pattern = Regex::new(&pattern).context("invalid regular expressions")?;

    let mut has_errors = false;

    for item in &items {
        debug!("item '{}'", item.name);

        if url_pattern.is_match(&item.key_) {
            let groups = url_pattern.captures_iter(&item.key_).next()
                .context("unable to get regexp group")?;
            let url = String::from(&groups[1]);
            debug!("- url '{url}'");

            match hosts.iter()
                .find(|host| host.host_id == item.hostid) {
                Some(host) => {

                    // TODO: able to customize
                    let scenario_name = format!("Check index page '{url}'");

                    match web_scenarios.iter()
                        .find(|entity| entity.name == scenario_name) {
                        None => {
                            match zabbix_service.create_web_scenario(
                                &auth_token, &url, &host.host_id, &zabbix_config.scenario) {
                                Ok(_) => info!("web scenario has been created for '{url}'"),
                                Err(e) => {
                                    error!("unable to create web scenario: {}", e);
                                    return Err(e)
                                }
                            }
                        }
                        Some(_) => info!("web scenario '{scenario_name}' already found, skip.")
                    }

                    let template_vars = get_template_vars(&host.host, &url);
                    let trigger_name = process_template_string(
                        &zabbix_config.trigger.name, &template_vars);

                    match zabbix_service.find_trigger(&auth_token, &trigger_name) {
                        Ok(trigger_found) => {
                            match trigger_found {
                                None => {
                                    match zabbix_service.create_trigger(&auth_token, &zabbix_config.trigger, &host.host, &url) {
                                        Ok(_) => info!("trigger '{trigger_name}' has been created"),
                                        Err(e) => {
                                            error!("unable to create zabbix trigger: {}", e);
                                            has_errors = true;
                                        }
                                    }
                                }
                                Some(_) => info!("trigger '{trigger_name}' already exists, skip")
                            }

                        }
                        Err(e) => error!("unable to find zabbix trigger by name '{trigger_name}': {}", e)
                    }
                }
                None => {
                    error!("host wasn't found by id {}", item.hostid);
                    has_errors = true;
                }
            }

        } else {
            error!("unsupported item format");
            has_errors = true;
        }
    }

    if !has_errors {
        Ok(())

    } else {
        Err(anyhow!("unable to create web scenarios and triggers"))
    }
}