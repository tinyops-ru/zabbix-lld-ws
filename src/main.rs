#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;
use std::path::Path;
use std::process::exit;

use clap::{App, Arg, SubCommand};
use reqwest::blocking::Client;

use crate::command::generate::items::generate_web_scenarios_and_triggers_for_items;
use crate::config::load_config_from_file;
use crate::logging::get_logging_config;
use crate::zabbix::service::DefaultZabbixService;

mod types;

mod config;

mod logging;
mod http;
pub mod template;
pub mod zabbix;
pub mod command;

#[cfg(test)]
pub mod tests;

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

                    let zabbix_service = DefaultZabbixService::new(
                        &config.zabbix.api.endpoint, &config.zabbix.api.version, &client);

                    let item_key_search_mask: &str = if matches.is_present(ITEM_KEY_SEARCH_MASK_ARG) {
                        matches.value_of(ITEM_KEY_SEARCH_MASK_ARG).unwrap()
                    } else { ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE };

                    match generate_web_scenarios_and_triggers_for_items(&zabbix_service,
                                    &config.zabbix, &item_key_search_mask) {
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