#![feature(proc_macro_hygiene, decl_macro)]

mod log_config;
mod db;

#[cfg(test)]
mod tests;

extern crate bson;
#[macro_use(bson, doc)]
extern crate mongodb;
extern crate log;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::env;
use log::{debug,info};

use rocket::{State, Request};
use rocket::config::{Config, Environment};
use rocket::request::{LenientForm};
use rocket_contrib::json::{JsonValue};

use crate::db::{connect, Repo};
use mongodb::db::{Database};
use rocket::logger::LoggingLevel;

#[derive(FromForm,Serialize, Deserialize,Debug)]
pub struct Tiddly {
    //_id: String,
    name: String,
    content: Option<String>
}

#[get("/<name>")]
fn get_by_name(db: State<Database>, name: String) -> String {
    return match Tiddly::find_one(name, db.inner()){
        Some(tiddly) => format!("Found: {}", tiddly.name),
        None => "Not found".to_string()
    };
}

#[post("/<_name>", data = "<tiddly>")]
fn save_tiddly_with_name(db: State<Database>, _name:String, tiddly: LenientForm<Tiddly>) -> String {
    debug!("Got tiddly: {}", tiddly.name);
    let entity = tiddly.into_inner().save(db.inner());
    return entity.name;
}

#[post("/", data = "<tiddly>")]
fn save_tiddly(db: State<Database>, tiddly: LenientForm<Tiddly>) -> String {
    debug!("Got tiddly: {}", tiddly.name);
    let entity = tiddly.into_inner().save(db.inner());
    return entity.name;
}


#[delete("/<name>")]
fn delete_tiddly(db: State<Database>, name:String) -> String {
    Tiddly::delete(name, db.inner());
    return get_by_name(db,"home".to_string());
}

#[get("/health")]
fn health_endpoint() -> JsonValue {
    return json!({"status": "ok"});
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

fn main() {
    log_config::config_log();
    let port = get_env_var_or_default("PORT","8080");
    let db:Database = connect();
    rocket(port.to_owned(),db).launch();
    info!("Started at port: {}",port);
}

fn rocket(port: String, db: Database) -> rocket::Rocket {
    let environment = Environment::active().expect("Unable to detect rocket environment");

    let config = Config::build(environment)
        //.log_level(LoggingLevel::Debug) // FIXME remove it
        .port(string_to_u16(&port))
        .finalize().expect("Unable to build rocket config");

    return rocket::custom(config)
        .manage(db)
        .register(catchers![not_found])
        .mount("/wiki",routes![get_by_name,save_tiddly,save_tiddly_with_name, delete_tiddly])
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
