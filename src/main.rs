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

use crate::auth::login_to_zabbix_api;
use crate::config::{load_config_from_file, ZabbixApiVersion, ZabbixConfig};
use crate::hosts::{find_hosts, ZabbixHost};
use crate::items::{find_zabbix_items, ZabbixItem};
use crate::logging::get_logging_config;
use crate::template::{get_template_vars, process_template_string};
use crate::triggers::{create_trigger, find_zabbix_trigger};
use crate::types::{EmptyResult, OperationResult};
use crate::webscenarios::{create_web_scenario, find_web_scenarios, ZabbixWebScenario};

mod types;

mod config;

mod zabbix;
mod auth;

mod items;
mod webscenarios;
mod triggers;
mod hosts;
mod logging;
mod http;
pub mod template;

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

                    match create_web_scenarios_and_triggers(&config.zabbix.api.version,
                                    &client, &config.zabbix, &item_key_search_mask) {
                        Ok(_) => exit(OK_EXIT_CODE),
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

fn create_web_scenarios_and_triggers(api_version: &ZabbixApiVersion,
                                     client: &Client, zabbix_config: &ZabbixConfig,
                                     item_key_search_mask: &str) -> EmptyResult {

    let auth_token = login_to_zabbix_api(&api_version, &client,
                                         &zabbix_config.api.endpoint,
                                         &zabbix_config.api.username, &zabbix_config.api.password)
                                                    .context("zabbix api authentication error")?;

    let partial_auth_token = truncate(&auth_token, 4);
    debug!("login success: token '{partial_auth_token}..'");

    let zabbix_objects = find_zabbix_objects(client, zabbix_config, &auth_token, &item_key_search_mask)
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

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

fn find_zabbix_objects(client: &Client, zabbix_config: &ZabbixConfig,
                       auth_token: &str, item_key_search_mask: &str) ->
                                                                OperationResult<ZabbixEntities> {

    let items = find_zabbix_items(&client, &zabbix_config.api.endpoint,
                      &auth_token, item_key_search_mask).context("unable to find zabbix items")?;

    debug!("received items:");

    let web_scenarios = find_web_scenarios(&client, &zabbix_config.api.endpoint, &auth_token).context("unable to find web scenarios")?;

    debug!("web scenarios have been obtained");

    let host_ids: Vec<String> = items.iter()
        .map(|item| item.hostid.to_string()).collect();

    let hosts = find_hosts(&client, &zabbix_config.api.endpoint, &auth_token, host_ids).context("unable to find hosts")?;

    Ok(
        ZabbixEntities {
            items,
            web_scenarios,
            hosts
        }
    )
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

fn create_web_scenario_if_does_not_exists(zabbix_config: &ZabbixConfig, auth_token: &str,
                                          url: &str, client: &Client,
                        zabbix_host: &ZabbixHost, zabbix_objects: &ZabbixEntities) -> EmptyResult {

    let scenario_name = format!("Check index page '{url}'");

    match zabbix_objects.web_scenarios.iter()
        .find(|entity| entity.name == scenario_name) {
        None => {
            match create_web_scenario(
                &client, &zabbix_config.api.endpoint, &auth_token,
                &zabbix_config.scenario, &url, &zabbix_host.host_id) {
                Ok(_) => info!("web scenario has been created for '{url}'"),
                Err(e) => {
                    error!("unable to create web scenario: {}", e);
                    return Err(e)
                }
            }
        }
        Some(_) => info!("web scenario '{scenario_name}' already found, skip.")
    }

    Ok(())
}

fn create_trigger_if_does_not_exists(zabbix_config: &ZabbixConfig, auth_token: &str,
                                     url: &str, client: &Client,
                                     zabbix_host: &ZabbixHost) -> EmptyResult {
    let template_vars = get_template_vars(&zabbix_host.host, &url);
    let trigger_name = process_template_string(
        &zabbix_config.trigger.name, &template_vars);

    match find_zabbix_trigger(&client, &zabbix_config, &auth_token, &trigger_name) {
        Ok(trigger) => {
            if trigger.is_none() {
                match create_trigger(&client,
                                     &zabbix_config.api.endpoint, &auth_token,
                                     &zabbix_config.trigger, &zabbix_host.host, &url) {
                    Ok(_) => info!("trigger '{trigger_name}' has been created"),
                    Err(e) =>
                        error!("unable to create trigger '{trigger_name}': {}", e)
                }

            } else {
                info!("trigger '{trigger_name}' already exists, skip")
            }
        }
        Err(e) =>
            error!("unable to find zabbix trigger by name '{trigger_name}': {}", e)
    }

    Ok(())
}

struct ZabbixEntities {
    items: Vec<ZabbixItem>,
    web_scenarios: Vec<ZabbixWebScenario>,
    hosts: Vec<ZabbixHost>
}
