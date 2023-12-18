use crate::config::ZabbixApiVersion;
use crate::types::OperationResult;

pub fn login_to_zabbix_api(api_version: &ZabbixApiVersion,
                           client: &reqwest::blocking::Client, api_endpoint: &str,
                           username: &str, password: &str) -> OperationResult<String> {

    unimplemented!()
}

