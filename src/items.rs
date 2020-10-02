pub mod items {
    use crate::zabbix::zabbix::JSONRPC;
    use serde::Serialize;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Serialize)]
    struct ItemSearchRequest {
        jsonrpc: String,
        method: String,
        params: ItemSearchParams,
        auth: String,
        id: i8
    }

    #[derive(Serialize)]
    struct ItemSearchParams {
        key_: String,
        sortfield: String,
        search: HashMap<String, String>
    }

    #[derive(Deserialize)]
    struct ItemSearchResponse {
        result: Vec<ZabbixItem>
    }

    #[derive(Deserialize)]
    pub struct ZabbixItem {
        pub name: String,
        pub key_: String
    }

    pub fn find_items(api_endpoint: &str, auth_token: &str) ->
                                            Result<Vec<ZabbixItem>, Box<dyn std::error::Error>> {
        println!("searching items..");

        let mut search_params = HashMap::new();
        search_params.insert("key_".to_string(), "vhost.item".to_string());
        
        let search_request = ItemSearchRequest {
            jsonrpc: JSONRPC.to_string(),
            method: "item.get".to_string(),
            params: ItemSearchParams {
                key_: "vhost.item".to_string(), 
                sortfield: "name".to_string(),
                search: search_params
            },
            auth: auth_token.to_string(),
            id: 1
        };

        let client = reqwest::blocking::Client::new();

        let request_body = serde_json::to_string(&search_request).unwrap();

        let response = client.post(api_endpoint)
            .body(request_body)
            .header("Content-Type", "application/json")
            .send()?;

        let response_status = response.status();
        let response_text = response.text().unwrap();

        println!("{}", response_text);

        if response_status != reqwest::StatusCode::OK {
            println!("{}", response_text);
            panic!("unexpected server response code {}", response_status)
        }

        let search_response: ItemSearchResponse = serde_json::from_str(&response_text).unwrap();

        Ok(search_response.result)
    }
}
