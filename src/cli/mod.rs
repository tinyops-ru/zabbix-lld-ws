use std::env;
use std::path::Path;
use std::process::exit;

use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::blocking::Client;
use zabbix_api::client::v6::ZabbixApiV6Client;

use crate::command::generate::items::generate_web_scenarios_and_triggers;
use crate::config::load_config_from_file;
use crate::logging::get_logging_config;
use crate::source::file::FileUrlSourceProvider;
use crate::source::zabbix::ZabbixUrlSourceProvider;

pub const GENERATE_COMMAND: &str = "gen";

pub const SOURCE_ARG: &str = "source";
pub const SOURCE_ARG_DEFAULT_VALUE: &str = "zabbix";
pub const SOURCE_ARG_FILE_VALUE: &str = "file";

pub const FILE_ARG: &str = "file";
pub const FILE_ARG_DEFAULT_VALUE: &str = "urls.txt";
pub const FILE_SHORT_ARG: &str = "f";
pub const SOURCE_SHORT_ARG: &str = "s";
pub const ITEM_KEY_SEARCH_MASK_ARG: &str = "item-key-starts-with";
pub const ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE: &str = "vhost.item";

pub const WORK_DIR_ARG: &str = "work-dir";
pub const WORK_DIR_SHORT_ARG: &str = "d";

pub const LOG_LEVEL_ARG: &str = "log-level";
pub const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

pub const OK_EXIT_CODE: i32 = 0;
pub const ERROR_EXIT_CODE: i32 = 1;

pub fn get_cli_app() -> ArgMatches<'static> {
    let matches = App::new("WSZL tool")
        .version("0.8.0")
        .author("Eugene Lebedev <duke.tougu@gmail.com>")
        .about("Add Web scenarios support for Zabbix Low Level Discovery")
        .arg(
            Arg::with_name(WORK_DIR_ARG)
                .long(WORK_DIR_ARG)
                .short(WORK_DIR_SHORT_ARG)
                .help("set working directory")
                .takes_value(true)
        )
        .arg(
            Arg::with_name(LOG_LEVEL_ARG)
                .long(LOG_LEVEL_ARG)
                .help("set logging level. possible values: debug, info, error, warn, trace")
                .case_insensitive(true)
                .takes_value(true).required(false)
                .default_value(LOG_LEVEL_DEFAULT_VALUE)
        )
        .subcommand(SubCommand::with_name(GENERATE_COMMAND)
            .about("generate web scenarios and triggers for zabbix items")
            .arg(
                Arg::with_name(SOURCE_ARG)
                    .long(SOURCE_ARG)
                    .short(SOURCE_SHORT_ARG)
                    .help("set urls source: zabbix, text-file")
                    .default_value(SOURCE_ARG_DEFAULT_VALUE)
                    .possible_values(&[SOURCE_ARG_DEFAULT_VALUE, SOURCE_ARG_FILE_VALUE])
                    .takes_value(true)
                    .required(false)
            )
            .arg(
                Arg::with_name(FILE_ARG)
                    .long(FILE_ARG)
                    .short(FILE_SHORT_ARG)
                    .requires(SOURCE_ARG)
                    .help("urls file name. Expected file format (per row): zabbix-host|url")
                    .default_value(FILE_ARG_DEFAULT_VALUE)
                    .takes_value(true)
                    .required(false)
            )
            .arg(
                Arg::with_name(ITEM_KEY_SEARCH_MASK_ARG)
                    .long(ITEM_KEY_SEARCH_MASK_ARG)
                    .short(ITEM_KEY_SEARCH_MASK_ARG)
                    .help("set search mask for items")
                    .default_value(ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE)
                    .takes_value(true)
                    .required(false)
            )
        )
        .get_matches();

    let working_directory: &Path = if matches.is_present(WORK_DIR_ARG) {
        let work_dir_value = matches.value_of(WORK_DIR_ARG).unwrap();
        Path::new(work_dir_value)

    } else { Path::new("/etc/zabbix") };

    env::set_current_dir(working_directory).expect("unable to set working directory");

    let logging_level: &str = if matches.is_present(LOG_LEVEL_ARG) {
        matches.value_of(LOG_LEVEL_ARG).unwrap()
    } else { LOG_LEVEL_DEFAULT_VALUE };

    let logging_config = get_logging_config(logging_level);
    log4rs::init_config(logging_config).unwrap();

    matches
}

pub fn process_cli_commands(matches: &ArgMatches) {
    let mut matched_command = false;

    match matches.subcommand_matches(GENERATE_COMMAND) {
        Some(_) => {
            matched_command = true;
            let config_file_path = Path::new("wszl.yml");

            match load_config_from_file(config_file_path) {
                Ok(config) => {
                    let http_client = Client::new();

                    let zabbix_client = ZabbixApiV6Client::new(
                        http_client, &config.zabbix.api.endpoint);

                    let item_key_search_mask: &str = if matches.is_present(ITEM_KEY_SEARCH_MASK_ARG) {
                        matches.value_of(ITEM_KEY_SEARCH_MASK_ARG).unwrap()
                    } else { ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE };

                    let url_source_type: &str = if matches.is_present(SOURCE_ARG) {
                        matches.value_of(SOURCE_ARG).unwrap()
                    } else { SOURCE_ARG_DEFAULT_VALUE };

                    let filename: &str = if matches.is_present(FILE_ARG) {
                        matches.value_of(FILE_ARG).unwrap()
                    } else { FILE_ARG_DEFAULT_VALUE };

                    info!("collecting urls from source '{url_source_type}'..");

                    if url_source_type == SOURCE_ARG_DEFAULT_VALUE {
                        let url_provider = ZabbixUrlSourceProvider::new(
                            &config.zabbix, zabbix_client.clone(), item_key_search_mask
                        );

                        match generate_web_scenarios_and_triggers(
                            &zabbix_client, &config.zabbix.api.username, &config.zabbix.api.password, url_provider,
                            &config.zabbix.scenario, &config.zabbix.trigger
                        ) {
                            Ok(_) => exit(OK_EXIT_CODE),
                            Err(_) => exit(ERROR_EXIT_CODE)
                        }

                    } else if url_source_type == SOURCE_ARG_FILE_VALUE {
                        let url_provider = FileUrlSourceProvider::new(filename);

                        match generate_web_scenarios_and_triggers(
                            &zabbix_client, &config.zabbix.api.username, &config.zabbix.api.password, url_provider,
                            &config.zabbix.scenario, &config.zabbix.trigger
                        ) {
                            Ok(_) => exit(OK_EXIT_CODE),
                            Err(_) => exit(ERROR_EXIT_CODE)
                        }

                    } else {
                        error!("unsupported data source type '{url_source_type}'");
                        exit(ERROR_EXIT_CODE)
                    }

                }
                Err(e) => {
                    error!("config load error: {}", e);
                    error!("{}", e.root_cause());
                    exit(ERROR_EXIT_CODE);
                }
            }
        }
        None => {}
    }

    if !matched_command {
        matches.usage();
    }
}