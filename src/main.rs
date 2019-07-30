#![feature(proc_macro_hygiene, decl_macro)]

mod log_config;
mod db;
mod view;

#[cfg(test)]
mod tests;

extern crate bson;
#[macro_use(bson, doc)]
extern crate mongodb;
extern crate log;
extern crate markdown;
extern crate base64;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::env;
use log::{info};

use crate::db::{connect};
use crate::view::{rocket};
use mongodb::db::{Database};

#[derive(FromForm,Serialize, Deserialize,Debug)]
pub struct Tiddly {
    //_id: String,
    name: String,
    body: Option<String>,
    //tags: [String]
}

fn main() {
    log_config::config_log();
    let port = get_env_var_or_default("PORT","8081");
    let db:Database = connect();
    rocket(port.to_owned(),db).launch();
    info!("Started at port: {}",port);
}

pub fn get_env_var_or_default (key: &str, default: & str) -> String {
    return match env::var(key) {
        Ok(val) => val.to_owned(),
        Err(_e) => default.to_owned()
    };
}
