use std::path::Path;

use regex::Regex;

use crate::auth::auth::login;
use crate::config::config::load_config_from_file;
use crate::hosts::hosts::find_hosts;
use crate::items::items::find_items;
use crate::triggers::triggers::create_trigger;
use crate::webscenarios::webscenarios::{create_web_scenario, find_web_scenarios};

mod config;
mod config_tests;

mod zabbix;
mod auth;

mod items;
mod webscenarios;
mod triggers;
mod hosts;

fn main() {
    let config_file_path = Path::new("wszl.yml");

    match load_config_from_file(config_file_path) {
        Ok(config) => {
            match login(&config.zabbix.api_endpoint,
                        &config.zabbix.username, &config.zabbix.password) {
                Ok(token) => {
                    println!("login success: token '{}'", token);

                    match find_items(&config.zabbix.api_endpoint, &token) {
                        Ok(items) => {
                            println!("ITEMS:");

                            match find_web_scenarios(&config.zabbix.api_endpoint, &token) {
                                Ok(web_scenarios) => {
                                    println!("web scenarios have been obtained");

                                    let url_pattern = Regex::new("^vhost.item\\[(.*)\\]$").unwrap();

                                    let host_ids: Vec<String> = items.iter().map(|item| item.hostid.to_string()).collect();

                                    match find_hosts(&config.zabbix.api_endpoint, &token, host_ids) {
                                        Ok(hosts) => {

                                            for item in items {
                                                println!("---------------------------");
                                                println!("ITEM: {}", item.name);

                                                if url_pattern.is_match(&item.key_) {
                                                    let groups = url_pattern.captures_iter(&item.key_).next().unwrap();
                                                    let url = String::from(&groups[1]);
                                                    println!("- url '{}'", url);

                                                    let scenario_name = format!("Check index page '{}'", url);

                                                    match web_scenarios.iter().find(|entity| entity.name == scenario_name) {
                                                        Some(_) => println!("web scenario has been found for url '{}', skip", url),
                                                        None => {
                                                            println!("web scenario wasn't found for url '{}', creating..", url);

                                                            match hosts.iter().find(|host| host.hostid == item.hostid) {
                                                                Some(host) => {
                                                                    match create_web_scenario(&config.zabbix.api_endpoint, &token, &url, &host.hostid) {
                                                                        Ok(_) => {
                                                                            println!("web scenario has been created for '{}'", url);

                                                                            match create_trigger(&config.zabbix.api_endpoint, &token, &host.host, &url) {
                                                                                Ok(_) => println!("trigger has been created"),
                                                                                Err(_) => println!("error > unable to create trigger for url '{}'", url)
                                                                            }

                                                                        },
                                                                        Err(_) => println!("error > unable to create web scenario for url '{}'", url)
                                                                    }
                                                                }
                                                                None => {
                                                                    println!("error > host wasn't found by id {}", item.hostid)
                                                                }
                                                            }
                                                        }
                                                    }

                                                } else { println!("/!\\ unsupported item format") }
                                            }

                                        }
                                        Err(_) => println!("error > unable to get hosts by ids")
                                    }
                                }
                                Err(_) => println!("unable to get web scenarios")
                            }
                        }
                        Err(_) => println!("error > unable to get zabbix items")
                    }

                },

                Err(_) => println!("error. unable to login")
            }
        }
        Err(_) => println!("error > unable to load config from file")
    }
}
