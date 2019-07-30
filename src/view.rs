use log::{debug};

use rocket::{State, Request};
use rocket::config::{Config, Environment};
use rocket::request::{LenientForm};
use rocket_contrib::json::{JsonValue};

use crate::db::{Repo};
use mongodb::db::{Database};
use crate::Tiddly;
use rocket_contrib::templates::Template;
use rocket::response::Redirect;

#[derive(Serialize)]
struct TemplateContext {
    tiddler_name : String,
    body: Option<String>,
    html: Option<String>
}

impl TemplateContext {
    fn new(name: String) -> TemplateContext {
        TemplateContext{
            tiddler_name: name,
            body: Some("".to_string()),
            html: Some("".to_string())
        }
    }

    fn set_tiddly(&mut self, tiddly: Tiddly){
        self.body = tiddly.body.to_owned();
        // FIXME process asciidoc here
        self.html = tiddly.body.to_owned();
    }
}

#[get("/<name>/edit")]
fn edit_tiddly(db: State<Database>, name: String) -> Template {
    let mut context = TemplateContext::new(name.to_owned());

    return match Tiddly::find_one(name, db.inner()){
        Some(t) => {
            context.set_tiddly(t);
            return Template::render("edit",context)
        },
        None => Template::render("edit",context)
    };
}

#[get("/<name>")]
fn get_by_name(db: State<Database>, name: String) -> Template {
    let mut context = TemplateContext::new(name.to_owned());

    return match Tiddly::find_one(name, db.inner()){
        Some(t) => {
            context.set_tiddly(t);
            return Template::render("view",context)
        },
        None => Template::render("not_found",context)
    };
}

#[post("/<_name>", data = "<tiddly>")]
fn save_tiddly_with_name(db: State<Database>, _name:String, tiddly: LenientForm<Tiddly>) -> Template {
    debug!("Got tiddly: {}", tiddly.name);
    let entity = tiddly.into_inner().save(db.inner());
    return get_by_name(db,entity.name);
}

#[post("/", data = "<tiddly>")]
fn save_tiddly(db: State<Database>, tiddly: LenientForm<Tiddly>) -> Template {
    debug!("Got tiddly: {}", tiddly.name);
    let entity = tiddly.into_inner().save(db.inner());
    return get_by_name(db,entity.name);
}


#[delete("/<name>")]
fn delete_tiddly(db: State<Database>, name:String) -> Redirect {
    Tiddly::delete(name, db.inner());
    Redirect::to("/wiki/home")
}

#[get("/<name>/delete")]
fn get_delete_tiddly(db: State<Database>, name:String) -> Redirect {
    Tiddly::delete(name, db.inner());
    Redirect::to("/wiki/home")
}


#[get("/health")]
fn health_endpoint() -> JsonValue {
    return json!({"status": "ok"});
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/")]
fn home() -> Redirect {
    Redirect::to("/wiki/home")
}

#[get("/search?<_q>")]
fn search(_q:String) -> Template{
    // FIXME implement search
    let context = TemplateContext::new("Search..".to_string());
    Template::render("search",context)
}

pub fn rocket(port: String, db: Database) -> rocket::Rocket {
    let environment = Environment::active().expect("Unable to detect rocket environment");

    let config = Config::build(environment)
        //.log_level(LoggingLevel::Debug) // FIXME remove it
        .port(string_to_u16(&port))
        .finalize().expect("Unable to build rocket config");

    return rocket::custom(config)
        .manage(db)
        .register(catchers![not_found])
        .mount("/wiki",routes![get_by_name,save_tiddly,save_tiddly_with_name, delete_tiddly, get_delete_tiddly, edit_tiddly])
        .mount("/", routes![home,search, health_endpoint])
        .attach(Template::fairing());
}

fn string_to_u16(s: &String) -> u16{
    return s.parse::<u16>().unwrap();
}

