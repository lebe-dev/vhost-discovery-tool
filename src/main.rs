#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;
use std::process::exit;

use crate::logging::logging::get_logging_config;

mod logging;

const VERSION: &str = "1.0.0";

const VERTICAL_LINE: &str = "-----------------------------------";

const WITHOUT_ARGUMENTS: usize = 1;
const ONE_ARGUMENT: usize = 2;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let logging_config = get_logging_config();
    log4rs::init_config(logging_config).unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() == ONE_ARGUMENT {
        let first_argument = &args[1].to_lowercase();

        if is_version_command(first_argument) { show_version() }
        else if is_help_command(first_argument) { show_usage() }
        else {
            show_usage();
            exit(ERROR_EXIT_CODE);
        }

    } else if args.len() == WITHOUT_ARGUMENTS {
        info!("collect urls..");

    } else {
        show_usage();
        exit(ERROR_EXIT_CODE);
    }
}

fn is_version_command(arg: &str) -> bool {
    arg == "-v" || arg == "--version"
}

fn is_help_command(arg: &str) -> bool {
    arg == "-h" || arg == "--help"
}

fn show_usage() {
    println!("{}", VERTICAL_LINE);
    println!(" SITE DISCOVERY FLEA v{}", VERSION);
    println!("{}", VERTICAL_LINE);
    println!("Discover site configs for nginx and apache. Then generate urls and show output in Zabbix Low Level Discovery format");
    println!();
    println!("usage:");
    println!("<without arguments> - discover and generate site urls");
    println!("-v - show version");
}

fn show_version() { println!("{}", VERSION) }