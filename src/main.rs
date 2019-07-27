extern crate log;
extern crate log4rs;

use log::{info};

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Hello, world!");
}
