pub mod items {
    use std::collections::HashMap;

    use serde::Deserialize;
    use serde::Serialize;

    use crate::errors::errors::OperationError;
    use crate::http::http::send_post_request;
    use crate::types::types::OperationResult;
    use crate::zabbix::zabbix;
    use crate::zabbix::zabbix::ZabbixRequest;

    #[derive(Serialize)]
    struct ItemSearchParams {
        sortfield: String,
        search: HashMap<String, String>,
    }

    #[derive(Deserialize)]
    struct ItemSearchResponse {
        result: Vec<ZabbixItem>
    }

    #[derive(Deserialize)]
    pub struct ZabbixItem {
        pub name: String,
        pub key_: String,
        pub hostid: String
    }

    pub fn find_zabbix_items(client: &reqwest::blocking::Client,
                             api_endpoint: &str, auth_token: &str) ->
                                                                OperationResult<Vec<ZabbixItem>> {
        println!("searching items..");

        let mut search_params = HashMap::new();
        search_params.insert("key_".to_string(), "vhost.item[".to_string());

        let params = ItemSearchParams {
            sortfield: "name".to_string(),
            search: search_params
        };

        let request: ZabbixRequest<ItemSearchParams> = ZabbixRequest::new(
            "item.get", params, auth_token
        );

        match send_post_request(client, api_endpoint, request) {
            Ok(response) => {
                let search_response: ItemSearchResponse = serde_json::from_str(&response)
                                                .expect(zabbix::UNSUPPORTED_RESPONSE_MESSAGE);

                Ok(search_response.result)
            }
            Err(_) => {
                error!("unable to find zabbix items");
                Err(OperationError::Error)
            }
        }
    }
}
