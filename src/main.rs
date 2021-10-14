#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;
use std::path::Path;
use std::process::exit;

use clap::{App, Arg, SubCommand};
use regex::Regex;
use reqwest::blocking::Client;

use crate::auth::auth::login_to_zabbix_api;
use crate::config::config::{load_config_from_file, ZabbixConfig};
use crate::errors::errors::OperationError;
use crate::hosts::hosts::{find_hosts, ZabbixHost};
use crate::items::items::{find_zabbix_items, ZabbixItem};
use crate::logging::logging::get_logging_config;
use crate::triggers::triggers::create_trigger;
use crate::types::types::{EmptyResult, OperationResult};
use crate::webscenarios::webscenarios::{create_web_scenario, find_web_scenarios, ZabbixWebScenario};

mod types;

mod config;
mod config_tests;

mod zabbix;
mod auth;

mod items;
mod webscenarios;
mod triggers;
mod hosts;
mod logging;
mod errors;
mod http;

const GENERATE_COMMAND: &str = "gen";
const ITEM_KEY_SEARCH_MASK_ARG: &str = "item-key-starts-with";
const ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE: &str = "vhost.item";

const WORK_DIR_ARGUMENT: &str = "work-dir";

const LOG_LEVEL_ARGUMENT: &str = "log-level";
const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let matches = App::new("WSZL tool")
        .version("0.6.0")
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
                    .help("set search mask for items.")
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
                    let client = reqwest::blocking::Client::new();

                    let item_key_search_mask: &str = if matches.is_present(ITEM_KEY_SEARCH_MASK_ARG) {
                        matches.value_of(ITEM_KEY_SEARCH_MASK_ARG).unwrap()
                    } else { ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE };

                    match create_web_scenarios_and_triggers(&client, &config.zabbix, &item_key_search_mask) {
                        Ok(_) => info!("web scenarios and triggers have been created"),
                        Err(_) => exit(ERROR_EXIT_CODE)
                    }
                }
                Err(_) => error!("unable to load config from file")
            }
        }
        None => {}
    }

    if !matched_command {
        matches.usage();
    }
}

fn create_web_scenarios_and_triggers(client: &Client, zabbix_config: &ZabbixConfig,
                                     item_key_search_mask: &str) -> EmptyResult {
    match login_to_zabbix_api(&client, &zabbix_config.api.endpoint,
                              &zabbix_config.api.username, &zabbix_config.api.password) {
        Ok(auth_token) => {
            debug!("login success: token '{}'", auth_token);

            match find_zabbix_objects(client, zabbix_config, &auth_token, &item_key_search_mask) {
                Ok(zabbix_objects) => {
                    let pattern_start = "^".to_string() + item_key_search_mask;
                    let pattern = pattern_start + "\\[(.*)\\]$";

                    let url_pattern = Regex::new(&pattern).unwrap();

                    let mut has_errors = false;

                    for item in &zabbix_objects.items {
                        debug!("item '{}'", item.name);

                        match create_scenario_and_trigger_for_item(zabbix_config, &auth_token,
                                        client, &url_pattern, &zabbix_objects, item) {
                            Ok(_) => {}
                            Err(_) => has_errors = true
                        }
                    }

                    if has_errors {
                        Err(OperationError::Error)

                    } else {
                        Ok(())
                    }
                }
                Err(_) => {
                    error!("unable to get zabbix objects");
                    Err(OperationError::Error)
                }
            }

        },
        Err(_) => {
            error!("unable to login");
            Err(OperationError::Error)
        }
    }
}

fn find_zabbix_objects(client: &Client, zabbix_config: &ZabbixConfig,
                       auth_token: &str, item_key_search_mask: &str) ->
                                                                OperationResult<ZabbixObjects> {
    match find_zabbix_items(&client, &zabbix_config.api.endpoint,
                            &auth_token, item_key_search_mask) {
        Ok(items) => {
            debug!("received items:");

            match find_web_scenarios(&client, &zabbix_config.api.endpoint, &auth_token) {
                Ok(web_scenarios) => {
                    debug!("web scenarios have been obtained");

                    let host_ids: Vec<String> = items.iter()
                                    .map(|item| item.hostid.to_string()).collect();

                    match find_hosts(&client, &zabbix_config.api.endpoint, &auth_token, host_ids) {
                        Ok(hosts) => {

                            Ok(
                                ZabbixObjects {
                                    items,
                                    web_scenarios,
                                    hosts
                                }
                            )

                        }
                        Err(_) => {
                            error!("unable to get zabbix hosts by ids");
                            Err(OperationError::Error)
                        }
                    }
                }
                Err(_) => {
                    error!("unable to get zabbix web scenarios");
                    Err(OperationError::Error)
                }
            }
        }
        Err(_) => {
            error!("unable to get zabbix items");
            Err(OperationError::Error)
        }
    }
}

fn create_scenario_and_trigger_for_item(zabbix_config: &ZabbixConfig,
                                        auth_token: &str, client: &Client,
                                        url_pattern: &Regex, zabbix_objects: &ZabbixObjects,
                                        zabbix_item: &ZabbixItem) -> EmptyResult {
    let mut has_errors = false;

    debug!("---------------------------");
    debug!("item: {}", zabbix_item.name);

    if url_pattern.is_match(&zabbix_item.key_) {
        let groups = url_pattern.captures_iter(&zabbix_item.key_).next().unwrap();
        let url = String::from(&groups[1]);
        debug!("- url '{}'", url);

        let scenario_name = format!("Check index page '{}'", url);

        match zabbix_objects.web_scenarios.iter().find(|entity| entity.name == scenario_name) {
            Some(_) => debug!("web scenario has been found for url '{}', skip", url),
            None => {
                debug!("web scenario wasn't found for url '{}', creating..", url);

                match zabbix_objects.hosts.iter().find(|host| host.hostid == zabbix_item.hostid) {
                    Some(host) => {
                        match create_web_scenario(&client, &zabbix_config.api.endpoint, &auth_token, &zabbix_config.scenario, &url, &host.hostid) {
                            Ok(_) => {
                                info!("web scenario has been created for '{}'", url);

                                match create_trigger(&client, &zabbix_config.api.endpoint, &auth_token, &host.host, &url) {
                                    Ok(_) => info!("trigger has been created"),
                                    Err(_) => {
                                        error!("unable to create trigger for url '{}'", url);
                                        has_errors = true;
                                    }
                                }
                            },
                            Err(_) => {
                                error!("unable to create web scenario for url '{}'", url);
                                has_errors = true;
                            }
                        }
                    }
                    None => {
                        error!("host wasn't found by id {}", zabbix_item.hostid);
                        has_errors = true;
                    }
                }
            }
        }

    } else {
        error!("unsupported item format");
        has_errors = true;
    }

    if has_errors {
        Err(OperationError::Error)

    } else {
        Ok(())
    }
}

struct ZabbixObjects {
    items: Vec<ZabbixItem>,
    web_scenarios: Vec<ZabbixWebScenario>,
    hosts: Vec<ZabbixHost>
}
