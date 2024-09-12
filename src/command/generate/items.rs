use anyhow::Context;
use zabbix_api::client::ZabbixApiClient;
use zabbix_api::item::create::CreateItemRequest;
use zabbix_api::item::get::GetItemsRequestByKey;
use zabbix_api::trigger::create::CreateTriggerRequest;
use zabbix_api::trigger::get::GetTriggerByDescriptionRequest;
use zabbix_api::webscenario::create::CreateWebScenarioRequest;
use zabbix_api::webscenario::get::GetWebScenarioByNameRequest;
use zabbix_api::webscenario::ZabbixWebScenarioStep;

use crate::config::item::ZabbixItemConfig;
use crate::config::trigger::ZabbixTriggerConfig;
use crate::config::ws::WebScenarioConfig;
use crate::source::UrlSourceProvider;
use crate::template::{get_template_vars, process_template_string};
use crate::types::EmptyResult;
use crate::zabbix::host::find_zabbix_host_id;

pub fn generate_web_scenarios_and_triggers(
    zabbix_client: &impl ZabbixApiClient, zabbix_login: &str, zabbix_password: &str,
    url_source_provider: impl UrlSourceProvider, target_hostname: &str,
    web_scenario_config: &WebScenarioConfig,
    item_config: &ZabbixItemConfig, trigger_config: &ZabbixTriggerConfig) -> EmptyResult {
    info!("generate web scenarios and triggers..");

    let url_sources = url_source_provider.get_url_sources()?;

    debug!("url sources: {:?}", url_sources);

    let session = zabbix_client.get_auth_session(&zabbix_login, &zabbix_password)?;

    let mut host_id: String = String::new();

    if !target_hostname.is_empty() {
        if let Some(id) = find_zabbix_host_id(zabbix_client, &session, &target_hostname)? {
            host_id = id;
        }
    };

    for url_source in url_sources {
        debug!("url source: {:?}", url_source);

        let mut zabbix_host: String = url_source.zabbix_host.to_string();

        if target_hostname.is_empty() {
            if let Some(id) = find_zabbix_host_id(zabbix_client, &session, &url_source.zabbix_host)? {
                host_id = id;
            }
        } else {
            zabbix_host = target_hostname.to_string();
        }

        debug!("target host id '{host_id}'");

        if !host_id.is_empty() {
            let item_key = item_config.key_template.replace("{}", &url_source.url);

            let request = GetItemsRequestByKey::new(&item_key);

            let items_found = zabbix_client.get_items(&session, &request)?;

            if items_found.is_empty() {
                let name = item_config.name_template.replace("{}", &url_source.url);

                let request = CreateItemRequest {
                    name,
                    key_: item_key,
                    host_id: host_id.to_string(),
                    r#type: item_config.r#type,
                    value_type: item_config.value_type,
                    interface_id: item_config.interface_id.to_string(),
                    tags: item_config.tags.clone(),
                    delay: item_config.delay.to_string(),
                };

                debug!("create item request: {:?}", request);

                zabbix_client.create_item(&session, &request)?;

            } else {
                info!("item with key '{item_key}' already exists, skip")
            }

            let template_vars = get_template_vars(&zabbix_host, &url_source.url);

            let scenario_name = process_template_string(
                &web_scenario_config.name_template, &template_vars);

            let request = GetWebScenarioByNameRequest::new(&scenario_name);

            let web_scenarios = zabbix_client.get_webscenarios(&session, &request)?;

            if web_scenarios.is_empty() {
                let step = ZabbixWebScenarioStep {
                    name: process_template_string(&web_scenario_config.name_template, &template_vars),
                    url: url_source.url.to_string(),
                    status_codes: web_scenario_config.expect_status_code.to_string(),
                    no: "1".to_string(),
                };

                let request = CreateWebScenarioRequest {
                    name: scenario_name.to_string(),
                    host_id: host_id.to_string(),
                    steps: vec![step],
                };

                zabbix_client.create_webscenario(&session, &request).context("unable to create web-scenario")?;

                info!("web scenario '{scenario_name}' has been created")

            } else { info!("web-scenario '{scenario_name}' already exists, skip"); }

            let trigger_description = process_template_string(&trigger_config.name, &template_vars);

            let request = GetTriggerByDescriptionRequest::new(&trigger_description);

            let triggers = zabbix_client.get_triggers(&session, &request)?;

            if triggers.is_empty() {
                info!("trigger '{trigger_description}' wasn't found, creating..");

                let request = CreateTriggerRequest {
                    description: trigger_description.to_string(),
                    expression: process_template_string(
                        &trigger_config.problem_expression, &template_vars),
                    priority: trigger_config.priority,
                    recovery_mode: trigger_config.recovery_mode,
                    recovery_expression: process_template_string(
                        &trigger_config.recovery_expression, &template_vars),
                    url: process_template_string(&trigger_config.url, &template_vars),
                    event_name: process_template_string(&trigger_config.event_name, &template_vars),
                    dependencies: vec![],
                    tags: vec![],
                };

                zabbix_client.create_trigger(&session, &request)?;

                info!("trigger '{trigger_description}' has been created")

            } else { info!("trigger '{trigger_description}' already exists, skip") }

        } else {
            warn!("zabbix host '{}' wasn't found, skip", zabbix_host)
        }

    }

    Ok(())
}