use std::fs;

use anyhow::Context;

use crate::source::{UrlSource, UrlSourceProvider};
use crate::types::OperationResult;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub struct FileUrlSourceProvider {
    filename: String
}

impl FileUrlSourceProvider {
    pub fn new(filename: &str) -> FileUrlSourceProvider {
        FileUrlSourceProvider {
            filename: filename.to_string(),
        }
    }
}

impl UrlSourceProvider for FileUrlSourceProvider {
    fn get_url_sources(&self) -> OperationResult<Vec<UrlSource>> {
        info!("extracting url sources from file '{}'..", self.filename);

        let content = fs::read_to_string(&self.filename)
                                .context("unable to read url source file")?;

        let rows = content.split(LINE_ENDING).collect::<Vec<&str>>();

        let mut results: Vec<UrlSource> = vec![];

        for row in rows {

            let row_parts = row.split("|").collect::<Vec<&str>>();

            if row_parts.len() == 2 && !row.starts_with("#") {
                let hostname = row_parts.first().unwrap();
                let url = row_parts.last().unwrap();

                let url_source = UrlSource {
                    zabbix_host: hostname.to_string(),
                    url: url.to_string(),
                };

                debug!("add url source: {:?}", url_source);

                results.push(url_source)

            } else {
                debug!("row doesn't match pattern: '{row}' (skip)");
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use crate::source::file::FileUrlSourceProvider;
    use crate::source::UrlSourceProvider;
    use crate::tests::init_logging;

    #[test]
    fn url_sources_should_be_returned() {
        init_logging();

        let provider = FileUrlSourceProvider::new("tests/urls.txt");
        match provider.get_url_sources() {
            Ok(results) => {
                assert_eq!(2, results.len());

                assert!(
                    results.iter().find(|us|
                        us.zabbix_host == "websrv10-182" &&
                        us.url == "https://demo.company.com").is_some()
                );

                assert!(
                    results.iter().find(|us|
                        us.zabbix_host == "Websrv121" &&
                            us.url == "https://app12.stage.company.com").is_some()
                );
            }
            Err(e) => error!("unexpected error '{}': {}", e, e.root_cause())
        }
    }
}