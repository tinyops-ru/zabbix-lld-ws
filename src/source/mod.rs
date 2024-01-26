use crate::types::OperationResult;

pub mod zabbix;
pub mod file;

pub trait UrlSourceProvider {
    fn get_url_sources(&self) -> OperationResult<Vec<UrlSource>>;
}

#[derive(Debug)]
pub struct UrlSource {
    pub zabbix_host: String,
    pub url: String
}



