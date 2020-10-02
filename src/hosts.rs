pub mod hosts {
    use serde::Serialize;
    use serde::Deserialize;
    use crate::zabbix::zabbix::{JSONRPC, CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON};

    #[derive(Serialize)]
    struct SearchRequest {
        jsonrpc: String,
        method: String,
        params: SearchRequestParams,
        auth: String,
        id: u8
    }

    #[derive(Serialize)]
    struct SearchRequestParams {
        hostids: Vec<String>
    }

    #[derive(Deserialize)]
    struct SearchResponse {
        result: Vec<ZabbixHost>
    }

    #[derive(Deserialize)]
    pub struct ZabbixHost {
        pub hostid: String,
        pub host: String
    }

    pub fn find_hosts(api_endpoint: &str, api_token: &str, ids: Vec<String>) ->
                                             Result<Vec<ZabbixHost>, Box<dyn std::error::Error>> {
        println!("find hosts by ids..");
        
        let request = SearchRequest {
            jsonrpc: JSONRPC.to_string(),
            method: "host.get".to_string(),
            params: SearchRequestParams { hostids: ids },
            auth: api_token.to_string(),
            id: 1
        };

        let client = reqwest::blocking::Client::new();

        let request_body = serde_json::to_string(&request).unwrap();

        let response = client.post(api_endpoint)
            .body(request_body)
            .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON)
            .send()?;

        let response_status = response.status();
        let response_text = response.text().unwrap();

        println!("{}", response_text);

        if response_status != reqwest::StatusCode::OK {
            println!("{}", response_text);
            panic!("unexpected server response code {}", response_status)
        }

        let search_response: SearchResponse = serde_json::from_str(&response_text).unwrap();

        Ok(search_response.result)
    }
}
