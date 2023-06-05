use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

use crate::http::send_post_request;
use crate::types::OperationResult;
use crate::zabbix::{UNSUPPORTED_RESPONSE_MESSAGE, ZABBIX_API_COMMUNICATION_ERROR, ZabbixError, ZabbixRequest};

#[derive(Serialize)]
struct ItemSearchParams {
    sortfield: String,
    search: HashMap<String, String>,
}

#[derive(Deserialize)]
struct ItemSearchResponse {
    result: Option<Vec<ZabbixItem>>,
    error: Option<ZabbixError>
}

#[derive(Deserialize, Debug)]
pub struct ZabbixItem {
    pub name: String,
    pub key_: String,
    pub hostid: String
}

pub fn find_zabbix_items(client: &reqwest::blocking::Client,
                         api_endpoint: &str,
                         auth_token: &str, item_key_search_mask: &str) ->
                         OperationResult<Vec<ZabbixItem>> {
    info!("searching zabbix items..");

    let mut search_params = HashMap::new();
    search_params.insert("key_".to_string(), item_key_search_mask.to_string());

    let params = ItemSearchParams {
        sortfield: "name".to_string(),
        search: search_params
    };

    let request: ZabbixRequest<ItemSearchParams> = ZabbixRequest::new(
        "item.get", params, auth_token
    );

    let response = send_post_request(client, api_endpoint, request)
        .context(ZABBIX_API_COMMUNICATION_ERROR)?;

    let search_response: ItemSearchResponse = serde_json::from_str(&response)
        .context(UNSUPPORTED_RESPONSE_MESSAGE)?;

    if let Some(items) = search_response.result {
        debug!("zabbix items: {:?}", items);
        Ok(items)

    } else {
        Err(anyhow!("unable to get zabbix items"))
    }
}