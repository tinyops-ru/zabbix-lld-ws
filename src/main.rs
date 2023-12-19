#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;
use std::path::Path;
use std::process::exit;

use anyhow::anyhow;
use anyhow::Context;
use clap::{App, Arg, SubCommand};
use regex::Regex;
use reqwest::blocking::Client;

use crate::config::{load_config_from_file, ZabbixConfig};
use crate::logging::get_logging_config;
use crate::types::EmptyResult;
use crate::zabbix::find::find_zabbix_objects;
use crate::zabbix::hosts::ZabbixHost;
use crate::zabbix::items::ZabbixItem;
use crate::zabbix::service::{DefaultZabbixService, ZabbixService};
use crate::zabbix::triggers::create::create_trigger_if_does_not_exists;
use crate::zabbix::webscenarios::create::create_web_scenario_if_does_not_exists;
use crate::zabbix::webscenarios::ZabbixWebScenario;

mod types;

mod config;

mod logging;
mod http;
pub mod template;
pub mod zabbix;

const GENERATE_COMMAND: &str = "gen";
const ITEM_KEY_SEARCH_MASK_ARG: &str = "item-key-starts-with";
const ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE: &str = "vhost.item";

const WORK_DIR_ARGUMENT: &str = "work-dir";

const LOG_LEVEL_ARGUMENT: &str = "log-level";
const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

const OK_EXIT_CODE: i32 = 0;
const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let matches = App::new("WSZL tool")
        .version("0.8.0")
        .author("Eugene Lebedev <duke.tougu@gmail.com>")
        .about("Add Web scenarios support for Zabbix Low Level Discovery")
        .arg(
            Arg::with_name(WORK_DIR_ARGUMENT)
                .short("d")
                .help("set working directory")
                .long(WORK_DIR_ARGUMENT).takes_value(true)
        )
        .arg(
            Arg::with_name(LOG_LEVEL_ARGUMENT)
                .help("set logging level. possible values: debug, info, error, warn, trace")
                .long(LOG_LEVEL_ARGUMENT)
                .case_insensitive(true)
                .takes_value(true).required(false)
                .default_value(LOG_LEVEL_DEFAULT_VALUE)
        )
        .subcommand(SubCommand::with_name(GENERATE_COMMAND)
            .about("generate web scenarios and triggers for zabbix items")
            .arg(
                Arg::with_name(ITEM_KEY_SEARCH_MASK_ARG)
                    .short(ITEM_KEY_SEARCH_MASK_ARG)
                    .help("set search mask for items")
                    .default_value(ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE)
                    .long(ITEM_KEY_SEARCH_MASK_ARG).takes_value(true)
                    .required(false)
            )
        )
        .get_matches();

    let working_directory: &Path = if matches.is_present(WORK_DIR_ARGUMENT) {
        let work_dir_value = matches.value_of(WORK_DIR_ARGUMENT).unwrap();
        Path::new(work_dir_value)

    } else { Path::new("/etc/zabbix") };

    env::set_current_dir(working_directory).expect("unable to set working directory");

    let logging_level: &str = if matches.is_present(LOG_LEVEL_ARGUMENT) {
        matches.value_of(LOG_LEVEL_ARGUMENT).unwrap()
    } else { LOG_LEVEL_DEFAULT_VALUE };

    let logging_config = get_logging_config(logging_level);
    log4rs::init_config(logging_config).unwrap();

    let mut matched_command = false;

    match matches.subcommand_matches(GENERATE_COMMAND) {
        Some(_) => {
            matched_command = true;
            let config_file_path = Path::new("wszl.yml");

            match load_config_from_file(config_file_path) {
                Ok(config) => {
                    let client = Client::new();

                    let zabbix_service = DefaultZabbixService::new(&config.zabbix.api.endpoint, &config.zabbix.api.version, &client);

                    let item_key_search_mask: &str = if matches.is_present(ITEM_KEY_SEARCH_MASK_ARG) {
                        matches.value_of(ITEM_KEY_SEARCH_MASK_ARG).unwrap()
                    } else { ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE };

                    match create_web_scenarios_and_triggers(&zabbix_service,
                                    &client, &config.zabbix, &item_key_search_mask) {
                        Ok(_) => exit(OK_EXIT_CODE),
                        Err(_) => exit(ERROR_EXIT_CODE)
                    }
                }
                Err(e) => error!("config load error: {}", e)
            }
        }
        None => {}
    }

    if !matched_command {
        matches.usage();
    }
}

fn create_web_scenarios_and_triggers(zabbix_service: &impl ZabbixService,
                                     client: &Client, zabbix_config: &ZabbixConfig,
                                     item_key_search_mask: &str) -> EmptyResult {

    let auth_token = zabbix_service.get_session(&zabbix_config.api.username, &zabbix_config.api.password)
                                          .context("zabbix api authentication error")?;

    let zabbix_objects = find_zabbix_objects(zabbix_service, client, zabbix_config, &auth_token, &item_key_search_mask)
        .context("unable to find zabbix objects")?;

    let pattern_start = "^".to_string() + item_key_search_mask;
    let pattern = pattern_start + "\\[(.*)\\]$";

    let url_pattern = Regex::new(&pattern).context("invalid regular expressions")?;

    let mut has_errors = false;

    for item in &zabbix_objects.items {
        debug!("item '{}'", item.name);

        match create_scenario_and_trigger_for_item(zabbix_config, &auth_token,
                                                   client, &url_pattern, &zabbix_objects, item) {
            Ok(_) => {}
            Err(e) => {
                error!("{}", e);
                has_errors = true
            }
        }
    }

    if has_errors {
        Err(anyhow!("unable to create web scenarios and trigger"))

    } else {
        Ok(())
    }
}

fn create_scenario_and_trigger_for_item(zabbix_config: &ZabbixConfig,
                                        auth_token: &str, client: &Client,
                                        url_pattern: &Regex, zabbix_objects: &ZabbixEntities,
                                        zabbix_item: &ZabbixItem) -> EmptyResult {
    let mut has_errors = false;

    debug!("---------------------------");
    debug!("item: {}", zabbix_item.name);

    if url_pattern.is_match(&zabbix_item.key_) {
        let groups = url_pattern.captures_iter(&zabbix_item.key_).next()
                                                        .context("unable to get regexp group")?;
        let url = String::from(&groups[1]);
        debug!("- url '{url}'");

        match zabbix_objects.hosts.iter()
            .find(|host| host.host_id == zabbix_item.hostid) {
            Some(host) => {

                match create_web_scenario_if_does_not_exists(&zabbix_config, &auth_token, &url, &client, &host, &zabbix_objects) {
                    Ok(_) => {
                        match create_trigger_if_does_not_exists(&zabbix_config, &auth_token, &url, &client, &host) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }

            }
            None => {
                error!("host wasn't found by id {}", zabbix_item.hostid);
                has_errors = true;
            }
        }

    } else {
        error!("unsupported item format");
        has_errors = true;
    }

    if has_errors {
        Err(anyhow!("unable to create web scenario and trigger for item"))

    } else {
        Ok(())
    }
}

pub struct ZabbixEntities {
    items: Vec<ZabbixItem>,
    web_scenarios: Vec<ZabbixWebScenario>,
    hosts: Vec<ZabbixHost>
}
