#![feature(proc_macro_hygiene, decl_macro)]

mod log_config;
#[cfg(test)] mod tests;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate log;

use std::env;
use log::{debug,info};
use rocket::config::{Config, Environment};
use rocket::request::{Form};
use rocket_contrib::json::{JsonValue};

#[derive(FromForm)]
struct Tiddly {
    name: String
}

#[get("/<name>")]
fn get_by_name(name: String) -> String {
    // FIXME implement get_by_name
    name
}

#[post("/<name>", data = "<tiddly>")]
fn save_tiddly(name:String, tiddly: Form<Tiddly>) -> String {
    debug!("Got tiddly: {}", tiddly.name);
    // FIXME implement save_tiddly
    return name;
}

#[delete("/<name>")]
fn delete_tiddly(name:String) -> String {
    // FIXME implement delete_tiddly
    return name;
}


#[get("/health")]
fn health_endpoint() -> JsonValue {
    return json!({"status": "ok"});
}

fn main() {
    log_config::config();
    let port = get_env_var_or_default("PORT","8080");
    rocket(port.to_owned()).launch();
    info!("Started at port: {}",port);
}

fn rocket(port: String) -> rocket::Rocket {
    let environment = Environment::active().expect("Unable to detect rocket environment");

    let config = Config::build(environment)
        .port(string_to_u16(&port))
        .finalize().expect("Unable to build rocket config");

    return rocket::custom(config)
        .mount("/wiki",routes![get_by_name,save_tiddly,delete_tiddly])
        .mount("/", routes![health_endpoint]);
}

fn string_to_u16(s: &String) -> u16{
    return s.parse::<u16>().unwrap();
}

fn get_env_var_or_default (key: &str, default: & str) -> String {
    return match env::var(key) {
        Ok(val) => val.to_owned(),
        Err(_e) => default.to_owned()
    };
}
