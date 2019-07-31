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
extern crate newrelic;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::env;
use log::{info};

use crate::db::{connect};
use crate::view::{rocket};
use mongodb::db::{Database};
use newrelic::{App};

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

    let new_app = build_newrelic_app();

    rocket(port.to_owned(),db,new_app).launch();
    info!("Started at port: {}",port);
}

fn build_newrelic_app() -> App {
    let license_key = env::var("NEW_RELIC_LICENSE_KEY").unwrap_or_else(|_| "example-license-key".to_string());
    let app = App::new("rust-wiki", &license_key).expect("Could not create app");
    return app;
}

pub fn get_env_var_or_default (key: &str, default: & str) -> String {
    return match env::var(key) {
        Ok(val) => val.to_owned(),
        Err(_e) => default.to_owned()
    };
}
