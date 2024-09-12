use crate::command::generate::items::generate_web_scenarios_and_triggers;
use crate::config::file::load_config_from_file;
use crate::logging::get_logging_config;
use crate::source::file::FileUrlSourceProvider;
use crate::source::zabbix::ZabbixUrlSourceProvider;
use clap::{Arg, ArgMatches, Command};
use reqwest::blocking::Client;
use std::env;
use std::path::Path;
use std::process::exit;
use zabbix_api::client::v6::ZabbixApiV6Client;

pub const GENERATE_COMMAND: &str = "gen";

pub const SOURCE_ARG: &str = "source";
pub const SOURCE_ARG_DEFAULT_VALUE: &str = "zabbix";
pub const SOURCE_ARG_FILE_VALUE: &str = "file";

pub const FILE_ARG: &str = "file";
pub const FILE_ARG_DEFAULT_VALUE: &str = "urls.txt";
pub const FILE_SHORT_ARG: &str = "f";
pub const SOURCE_SHORT_ARG: &str = "s";
pub const ITEM_KEY_SEARCH_MASK_ARG: &str = "item-key-starts-with";
pub const ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE: &str = "nginx.vhost.item";

pub const WORK_DIR_ARG: &str = "work-dir";
pub const WORK_DIR_SHORT_ARG: &str = "d";
pub const WORK_DIR_DEFAULT_VALUE: &str = ".";

pub const LOG_LEVEL_ARG: &str = "log-level";
pub const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

pub const OK_EXIT_CODE: i32 = 0;
pub const ERROR_EXIT_CODE: i32 = 1;

pub fn get_cli_app() -> ArgMatches {
    let matches = Command::new("WSZL tool")
        .version("1.0.0")
        .author("Eugene Lebedev <duke.tougu@gmail.com>")
        .about("Add Web scenarios support for Zabbix Low Level Discovery")
        .arg_required_else_help(true)
        .arg(
            Arg::new(WORK_DIR_ARG)
                .long(WORK_DIR_ARG)
                .short('d')
                .help("set working directory")
                .required(false)
                .default_value(WORK_DIR_DEFAULT_VALUE)
        )
        .arg(
            Arg::new(LOG_LEVEL_ARG)
                .long(LOG_LEVEL_ARG)
                .help("set logging level. possible values: debug, info, error, warn, trace")
                .ignore_case(true)
                .required(false)
                .default_value(LOG_LEVEL_DEFAULT_VALUE)
        )
        .subcommand(Command::new(GENERATE_COMMAND)
            .about("generate web scenarios and triggers for zabbix items")
            .arg(
                Arg::new(SOURCE_ARG)
                    .long(SOURCE_ARG)
                    .short('s')
                    .help("set urls source: zabbix, text-file")
                    .default_value(SOURCE_ARG_DEFAULT_VALUE)
                    .required(false)
            )
            .arg(
                Arg::new(FILE_ARG)
                    .long(FILE_ARG)
                    .short('f')
                    .requires(SOURCE_ARG)
                    .help("urls file name. Expected file format (per row): zabbix-host|url")
                    .default_value(FILE_ARG_DEFAULT_VALUE)
                    .required(false)
            )
            .arg(
                Arg::new(ITEM_KEY_SEARCH_MASK_ARG)
                    .long(ITEM_KEY_SEARCH_MASK_ARG)
                    .help("set search mask for items")
                    .default_value(ITEM_KEY_SEARCH_MASK_DEFAULT_VALUE)
                    .required(false)
            )
        )
        .get_matches();

    init_working_dir(&matches);
    init_logging(&matches);

    matches
}

pub fn init_working_dir(matches: &ArgMatches) {
    let working_directory: &Path = get_argument_path_value(
        &matches, WORK_DIR_ARG, WORK_DIR_DEFAULT_VALUE);

    debug!("working directory '{}'", &working_directory.display());

    env::set_current_dir(&working_directory).expect("couldn't set working directory");
}

fn init_logging(matches: &ArgMatches) {
    let log_level = match matches.get_one::<String>(LOG_LEVEL_ARG) {
        Some(value) => {value}
        None => LOG_LEVEL_DEFAULT_VALUE
    };

    let logging_config = get_logging_config(log_level);
    log4rs::init_config(logging_config).expect("logging init error");
}

pub fn process_cli_commands(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("gen", matches)) => {
            let config_file_path = Path::new("wszl.yml");

            match load_config_from_file(config_file_path) {
                Ok(config) => {
                    let http_client = Client::new();

                    let zabbix_client = ZabbixApiV6Client::new(
                        http_client, &config.zabbix.api.endpoint);

                    let item_key_search_mask = matches.get_one::<String>(ITEM_KEY_SEARCH_MASK_ARG).unwrap();
                    debug!("item key search mask '{item_key_search_mask}'");
                    let url_source_type = matches.get_one::<String>(SOURCE_ARG).unwrap();
                    debug!("url source type '{url_source_type}'");
                    let filename = matches.get_one::<String>(FILE_ARG).unwrap();
                    debug!("filename '{filename}'");

                    info!("collecting urls from source '{url_source_type}'..");

                    if url_source_type == SOURCE_ARG_DEFAULT_VALUE {
                        let url_provider = ZabbixUrlSourceProvider::new(
                            &config.zabbix, zabbix_client.clone(), item_key_search_mask
                        );

                        match generate_web_scenarios_and_triggers(
                            &zabbix_client, &config.zabbix.api.username, &config.zabbix.api.password,
                            url_provider, &config.zabbix.target_hostname,
                            &config.zabbix.scenario, &config.zabbix.item, &config.zabbix.trigger
                        ) {
                            Ok(_) => exit(OK_EXIT_CODE),
                            Err(e) => {
                                eprintln!("generation error: {}", e);
                                error!("{}", e.root_cause());
                                exit(ERROR_EXIT_CODE)
                            }
                        }

                    } else if url_source_type == SOURCE_ARG_FILE_VALUE {
                        let url_provider = FileUrlSourceProvider::new(filename);

                        match generate_web_scenarios_and_triggers(
                            &zabbix_client, &config.zabbix.api.username, &config.zabbix.api.password,
                            url_provider, &config.zabbix.target_hostname,
                            &config.zabbix.scenario, &config.zabbix.item, &config.zabbix.trigger
                        ) {
                            Ok(_) => exit(OK_EXIT_CODE),
                            Err(e) => {
                                eprintln!("generation error: {}", e);
                                error!("{}", e.root_cause());
                                exit(ERROR_EXIT_CODE)
                            }
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
        _ => println!("use -h to get help")
    }
}

fn get_argument_path_value<'a>(matches: &'a ArgMatches, long_argument: &str,
                               default_path: &'a str) -> &'a Path {
    let mut path: &Path = Path::new(default_path);

    match matches.get_one::<String>(long_argument) {
        Some(value) => path = Path::new(value),
        None => {}
    }

    path
}