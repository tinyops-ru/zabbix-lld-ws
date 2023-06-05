use anyhow::anyhow;
use anyhow::Context;
use serde::Serialize;

use crate::types::OperationResult;
use crate::zabbix::UNSUPPORTED_RESPONSE_MESSAGE;

const CONTENT_TYPE_HEADER: &str = "Content-Type";
const CONTENT_TYPE_JSON: &str = "application/json";

pub fn send_post_request<T: Serialize>(client: &reqwest::blocking::Client,
                                       url: &str, request: T) -> OperationResult<String> {
    debug!("send post request to '{url}'");

    let request_body = serde_json::to_string(&request)
        .context(UNSUPPORTED_RESPONSE_MESSAGE)?;

    let response = client.post(url)
        .body(request_body)
        .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON)
        .send().context("zabbix api communication error")?;

    let response_status = response.status();
    let response_text = response.text().context("unable to decode server response")?;

    debug!("---[HTTP RESPONSE]----");
    debug!("{}", response_text);
    debug!("---[/HTTP RESPONSE]----");

    if response_status == reqwest::StatusCode::OK {
        Ok(response_text)

    } else {
        error!("unexpected server response code {}", response_status);
        Err(anyhow!("unexpected server response"))
    }
}