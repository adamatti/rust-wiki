use log::{debug};

use rocket::{State, Request, Outcome, Response};
use rocket::config::{Config, Environment};
use rocket::request::{LenientForm, FromRequest};
use rocket::http::{Status};
use rocket::response::Redirect;
use rocket_contrib::json::{JsonValue};
use rocket_contrib::templates::Template;

use crate::db::{Repo};
use mongodb::db::{Database};
use crate::{Tiddly, get_env_var_or_default};
use std::io::Cursor;

struct User {
    name: String
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User, (Status, ()), ()> {
        let authorization = request.headers().get_one("Authorization");
        let xfp = request.headers().get_one("x-forwarded-proto");

        // User and pass come from env vars
        let user = get_env_var_or_default("APP_USER","admin");
        let password = get_env_var_or_default("APP_PASS","admin");

        // FIXME implement a way to have multiple users
        let root = User {name: "root".to_string() };

        let part_to_encode = format!("{}:{}",user,password);
        let string_to_match = format!("Basic {}",base64::encode(&part_to_encode));

        return match [authorization,xfp] {
            [Some(value),Some("https")] if value == string_to_match => Outcome::Success(root),
            [_, None] => Outcome::Failure((Status::Forbidden, ())),
            _ => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}


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
        self.html = Some(markdown::to_html(tiddly.body.unwrap().as_str()));
    }
}

#[get("/<name>/edit")]
fn edit_tiddly(db: State<Database>, name: String, _user:User) -> Template {
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
fn get_by_name(db: State<Database>, name: String, _user:User) -> Template {
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
fn save_tiddly_with_name(db: State<Database>, _name:String, tiddly: LenientForm<Tiddly>, _user:User) -> Template {
    debug!("Got tiddly: {}", tiddly.name);
    let entity = tiddly.into_inner().save(db.inner());
    return get_by_name(db,entity.name, _user);
}

#[post("/", data = "<tiddly>")]
fn save_tiddly(db: State<Database>, tiddly: LenientForm<Tiddly>, _user:User) -> Template {
    debug!("Got tiddly: {}", tiddly.name);
    let entity = tiddly.into_inner().save(db.inner());
    return get_by_name(db,entity.name, _user);
}


#[delete("/<name>")]
fn delete_tiddly(db: State<Database>, name:String, _user:User) -> Redirect {
    Tiddly::delete(name, db.inner());
    Redirect::to("/wiki/home")
}

#[get("/<name>/delete")]
fn get_delete_tiddly(db: State<Database>, name:String, _user:User) -> Redirect {
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

#[catch(403)]
fn forbidden() -> String {
    String::from("Need https")
}

#[catch(401)]
fn unauthorized<'a>() -> Result<Response<'a>,Status> {
    return Response::build()
        .raw_header("WWW-Authenticate","Basic")
        .status(Status::Unauthorized)
        .sized_body(Cursor::new("unauthorized: wrong user and pass"))
        .ok();
}

#[get("/")]
fn home(_user:User) -> Redirect {
    Redirect::to("/wiki/home")
}

#[get("/search?<_q>")]
fn search(_q:String, _user:User) -> Template{
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
        .register(catchers![not_found,unauthorized,forbidden])
        .mount("/wiki",routes![get_by_name,save_tiddly,save_tiddly_with_name, delete_tiddly, get_delete_tiddly, edit_tiddly])
        .mount("/", routes![home,search, health_endpoint])
        .attach(Template::fairing());
}

fn string_to_u16(s: &String) -> u16{
    return s.parse::<u16>().unwrap();
}

