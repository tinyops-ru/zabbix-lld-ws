use serde_derive::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WebScenarioConfig {
    pub key_starts_with: String,
    pub name_template: String,
    pub response_timeout: String,
    pub expect_status_code: String,
    pub attempts: u8,
    pub update_interval: String
}

impl Display for WebScenarioConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "key-starts-with: '{}', name-template: '{}', response-timeout: '{}', \
            expect-status-code: '{}, attempts: {}, update-interval: '{}'",
            self.key_starts_with, self.name_template, self.response_timeout,
            self.expect_status_code, self.attempts, self.update_interval
        )
    }
}