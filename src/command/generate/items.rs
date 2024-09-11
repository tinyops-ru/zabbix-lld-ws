use serde_derive::Serialize;
use zabbix_api::client::ZabbixApiClient;
use zabbix_api::host::get::GetHostsRequest;
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

pub fn generate_web_scenarios_and_triggers(
    zabbix_client: &impl ZabbixApiClient, zabbix_login: &str, zabbix_password: &str,
    url_source_provider: impl UrlSourceProvider, target_hostname: &str,
    web_scenario_config: &WebScenarioConfig,
    item_config: &ZabbixItemConfig, trigger_config: &ZabbixTriggerConfig) -> EmptyResult {

    let url_sources = url_source_provider.get_url_sources()?;

    debug!("url sources: {:?}", url_sources);

    let session = zabbix_client.get_auth_session(&zabbix_login, &zabbix_password)?;

    #[derive(Serialize)]
    struct HostFilter {
        pub host: Vec<String>
    }

    for url_source in url_sources {

        let zabbix_host: String;

        if target_hostname.is_empty() {
            url_source.zabbix_host.to_string()

        } else {
            target_hostname.to_string()
        }

        let request = GetHostsRequest {
            filter: HostFilter {
                host: vec![zabbix_host],
            },
        };

        debug!("looking for host '{zabbix_host}'..");

        let hosts_found = zabbix_client.get_hosts(&session, &request)?;

        match hosts_found.first() {
            Some(host) => {
                info!("zabbix host '{zabbix_host}' has been found");

                let item_key = item_config.key_template.replace("{}", &url_source.url);

                #[derive(Serialize)]
                struct ItemSearch {
                    pub key_: String
                }

                let request = GetItemsRequestByKey::new(&item_key);

                let items_found = zabbix_client.get_items(&session, &request)?;

                if items_found.is_empty() {
                    let name = item_config.name_template.replace("{}", &url_source.url);

                    let request = CreateItemRequest {
                        name,
                        key_: item_key,
                        host_id: host.host_id.to_string(),
                        r#type: item_config.r#type,
                        value_type: item_config.value_type,
                        interface_id: item_config.interface_id.to_string(),
                        tags: item_config.tags.clone(),
                        delay: item_config.delay.to_string(),
                    };

                    zabbix_client.create_item(&session, &request)?;

                } else {
                    info!("item with key '{item_key}' already exists, skip")
                }

                let template_vars = get_template_vars(&host.host, &url_source.url);

                // TODO: make configurable
                let scenario_name = format!("Check index page '{}'", &url_source.url);

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
                        host_id: host.host_id.to_string(),
                        steps: vec![step],
                    };

                    zabbix_client.create_webscenario(&session, &request)?;

                    info!("web scenario '{scenario_name}' has been created")

                } else { info!("web-scenario '{scenario_name}' already exists, skip"); }

                let trigger_description = process_template_string(&trigger_config.name, &template_vars);
                let trigger_expression = process_template_string(&trigger_config.value, &template_vars);

                let request = GetTriggerByDescriptionRequest::new(&trigger_description);

                let triggers = zabbix_client.get_triggers(&session, &request)?;

                if triggers.is_empty() {
                    info!("trigger '{trigger_description}' wasn't found, creating..");

                    let request = CreateTriggerRequest {
                        description: trigger_description.to_string(),
                        expression: trigger_expression.to_string(),
                        dependencies: vec![],
                        tags: vec![],
                    };

                    zabbix_client.create_trigger(&session, &request)?;

                    info!("trigger '{trigger_description}' has been created")

                } else { info!("trigger '{trigger_description}' already exists, skip") }

            }
            None => warn!("zabbix host '{}' wasn't found, skip", url_source.zabbix_host)
        }
    }

    Ok(())
}