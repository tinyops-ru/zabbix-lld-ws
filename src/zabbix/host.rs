use crate::types::OptionalResult;
use serde_derive::Serialize;
use zabbix_api::client::client::ZabbixApiClient;
use zabbix_api::host::get::GetHostsRequest;

pub fn find_zabbix_host_id(
    zabbix_client: &impl ZabbixApiClient,
    session: &str,
    hostname: &str,
) -> OptionalResult<String> {
    info!("find zabbix host id by hostname '{hostname}'..");
    let request = GetHostsRequest {
        filter: HostFilter {
            host: vec![hostname.to_string()],
        },
    };

    let hosts_found = zabbix_client.get_hosts(&session, &request)?;

    match hosts_found.first() {
        Some(host) => {
            info!(
                "zabbix host was found by name '{hostname}', id {}",
                host.host_id
            );
            Ok(Some(host.host_id.to_string()))
        }
        None => {
            info!("zabbix host wasn't found by name '{hostname}'");
            Ok(None)
        }
    }
}

#[derive(Serialize)]
struct HostFilter {
    pub host: Vec<String>,
}
