extern crate log4rs;

use log::{warn,LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root,Config};
use log4rs::Handle;
use std::path::Path;

fn config_log_hardcoded() -> Handle {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    return log4rs::init_config(config).unwrap();
}

pub fn config_log(){
    let file_name ="log4rs.yml";

    if !Path::new(file_name).exists() {
        warn!("{} file not found", file_name);
        config_log_hardcoded();
        return;
    }

    log4rs::init_file(file_name, Default::default()).unwrap();
}
