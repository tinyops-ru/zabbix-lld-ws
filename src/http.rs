pub mod http {
    use serde::Serialize;

    use crate::errors::errors::OperationError;
    use crate::types::types::StringResult;

    const CONTENT_TYPE_HEADER: &str = "Content-Type";
    const CONTENT_TYPE_JSON: &str = "application/json";

    pub fn send_post_request<T: Serialize>(client: &reqwest::blocking::Client,
                                url: &str, request: T) -> StringResult {
        debug!("send post request to '{}'", url);

        let request_body = serde_json::to_string(&request).unwrap();

        match client.post(url)
            .body(request_body)
            .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON)
            .send() {
            Ok(response) => {
                let response_status = response.status();
                let response_text = response.text().unwrap();

                debug!("---[HTTP RESPONSE]----");
                debug!("{}", response_text);
                debug!("---[/HTTP RESPONSE]----");

                if response_status == reqwest::StatusCode::OK {
                    Ok(response_text)

                } else {
                    error!("unexpected server response code {}", response_status);
                    Err(OperationError::Error)
                }
            }
            Err(e) => {
                error!("unable to create trigger: '{}'", e);
                Err(OperationError::Error)
            }
        }
    }
}
