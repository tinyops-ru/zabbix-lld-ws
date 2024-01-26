#[macro_use]
extern crate log;
extern crate log4rs;

use crate::cli::{get_cli_app, process_cli_commands};

pub mod types;

pub mod cli;

pub mod config;

pub mod logging;

pub mod template;
pub mod command;

pub mod source;

#[cfg(test)]
pub mod tests;


fn main() {
    let matches = get_cli_app();
    process_cli_commands(&matches);
}