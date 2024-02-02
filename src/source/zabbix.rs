use anyhow::Context;
use regex::Regex;
use serde_derive::Serialize;
use zabbix_api::client::ZabbixApiClient;
use zabbix_api::item::get::GetItemsRequestByKey;

use crate::config::ZabbixConfig;
use crate::source::{UrlSource, UrlSourceProvider};
use crate::types::OperationResult;

pub struct ZabbixUrlSourceProvider<T: ZabbixApiClient> {
    pub zabbix_config: ZabbixConfig,
    pub zabbix_client: T,
    pub item_key_search_mask: String
}

impl <T: ZabbixApiClient> ZabbixUrlSourceProvider<T> {
    pub fn new(zabbix_config: &ZabbixConfig, zabbix_service: T,
               item_key_search_mask: &str) -> ZabbixUrlSourceProvider<T> {
        ZabbixUrlSourceProvider {
            zabbix_config: zabbix_config.clone(),
            zabbix_client: zabbix_service,
            item_key_search_mask: item_key_search_mask.to_string()
        }
    }
}

impl <T: ZabbixApiClient> UrlSourceProvider for ZabbixUrlSourceProvider<T> {
    fn get_url_sources(&self) -> OperationResult<Vec<UrlSource>> {
        info!("getting url sources from zabbix server '{}'..", &self.zabbix_config.api.endpoint);

        let auth_token = &self.zabbix_client.get_auth_session(
            &self.zabbix_config.api.username, &self.zabbix_config.api.password).context("zabbix auth error")?;

        let request = GetItemsRequestByKey::new(&self.item_key_search_mask);

        debug!("request: {:?}", request);

        let items = &self.zabbix_client.get_items(
            &auth_token, &request).context("unable to find zabbix items")?;

        debug!("items received: {:?}", items);

        let host_ids: Vec<String> = items.iter()
            .map(|item| item.host_id.to_string()).collect();

        #[derive(Serialize)]
        struct Params {
            pub hostids: Vec<String>
        }

        let params = Params {
            hostids: host_ids.clone(),
        };

        debug!("search hosts by ids: {:?}", host_ids);

        let hosts = &self.zabbix_client.get_hosts(&auth_token, &params)
            .context("unable to find hosts")?;

        debug!("hosts received: {:?}", hosts);

        let mut results: Vec<UrlSource> = vec![];

        let pattern_start = "^".to_string() + &self.item_key_search_mask;
        let pattern = pattern_start + "\\[(.*)\\]$";

        let url_pattern = Regex::new(&pattern).context("invalid regular expressions")?;

        for item in items {
            debug!("item '{}'", item.name);

            if url_pattern.is_match(&item.key_) {
                let groups = url_pattern.captures_iter(&item.key_).next()
                    .context("unable to get pattern group")?;
                let url = String::from(&groups[1]);
                debug!("- url '{url}'");

                if let Some(host) = hosts.iter().find(|host| host.host_id == item.host_id) {
                    let url_source = UrlSource {
                        zabbix_host: host.host.to_string(),
                        url,
                    };

                    debug!("add url source: {:?}", url_source);

                    results.push(url_source)
                }
            }
        }

        Ok(results)
    }
}