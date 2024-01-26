use serde_derive::Serialize;
use zabbix_api::client::ZabbixApiClient;
use zabbix_api::host::get::GetHostsRequest;
use zabbix_api::item::create::CreateItemRequest;
use zabbix_api::item::get::GetItemsRequest;
use zabbix_api::trigger::create::CreateTriggerRequest;
use zabbix_api::trigger::get::GetTriggerByDescriptionRequest;
use zabbix_api::webscenario::create::CreateWebScenarioRequest;
use zabbix_api::webscenario::get::GetWebScenarioByNameRequest;
use zabbix_api::webscenario::ZabbixWebScenarioStep;
use zabbix_api::ZABBIX_EXTEND_PROPERTY_VALUE;

use crate::config::{WebScenarioConfig, ZabbixTriggerConfig};
use crate::source::UrlSourceProvider;
use crate::template::{get_template_vars, process_template_string};
use crate::types::EmptyResult;

pub fn generate_web_scenarios_and_triggers(zabbix_client: &impl ZabbixApiClient, zabbix_login: &str, zabbix_password: &str,
                               url_source_provider: impl UrlSourceProvider,
                               web_scenario_config: &WebScenarioConfig,
                               trigger_config: &ZabbixTriggerConfig) -> EmptyResult {

    let url_sources = url_source_provider.get_url_sources()?;

    let session = zabbix_client.get_auth_session(&zabbix_login, &zabbix_password)?;

    #[derive(Serialize)]
    struct HostFilter {
        pub host: Vec<String>
    }

    for url_source in url_sources {
        let request = GetHostsRequest {
            filter: HostFilter {
                host: vec![url_source.zabbix_host.to_string()],
            },
        };

        let hosts_found = zabbix_client.get_hosts(&session, &request)?;

        match hosts_found.first() {
            Some(host) => {
                info!("zabbix host '{}' has been found", &url_source.zabbix_host);

                let item_key = format!("vhost.item[{}]", &url_source.url);

                #[derive(Serialize)]
                struct ItemSearch {
                    pub key_: String
                }

                let request = GetItemsRequest {
                    output: ZABBIX_EXTEND_PROPERTY_VALUE.to_string(),
                    with_triggers: false,
                    host_ids: host.host_id.to_string(),
                    search: ItemSearch {
                        key_: item_key.to_string(),
                    },
                    sort_field: "name".to_string(),
                };

                let items_found = zabbix_client.get_items(&session, &request)?;

                if items_found.is_empty() {
                    // TODO: make configurable via wszl.yml
                    let request = CreateItemRequest {
                        name: format!("Vhost '{}' item", url_source.url),
                        key_: item_key,
                        host_id: host.host_id.to_string(),
                        r#type: 7,
                        value_type: 0,
                        interface_id: "0".to_string(),
                        tags: vec![],
                        delay: "5m".to_string(),
                    };

                    zabbix_client.create_item(&session, &request)?;

                } else {
                    info!("item with key '{item_key}' already exists, skip")
                }

                let template_vars = get_template_vars(&host.host_id, &url_source.url);

                // TODO: make configurable
                let scenario_name = format!("Check index page '{}'", &url_source.url);

                let request = GetWebScenarioByNameRequest::new(&scenario_name);

                let web_scenarios = zabbix_client.get_webscenarios(&session, &request)?;

                if web_scenarios.is_empty() {
                    let step = ZabbixWebScenarioStep {
                        name: process_template_string(&web_scenario_config.name, &template_vars),
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