#[macro_use]
extern crate log;
extern crate log4rs;

use crate::logging::logging::get_logging_config;

mod logging;

fn main() {
    let logging_config = get_logging_config();
    log4rs::init_config(logging_config).unwrap();

    println!("SITE DISCOVERY FLEA");
}
