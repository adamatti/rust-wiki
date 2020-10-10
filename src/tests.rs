use super::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};
use crate::db::connect;
use mongodb::sync::{Database};

fn build_client() -> Client{
    // FIXME find how to use global var
    let db:Database = connect();
    let rocket = rocket("8080".to_string(),db);

    return Client::new(rocket).unwrap();
}

#[test]
fn test_health_check(){
    let client = build_client();

    let response = client.get("/health")
        .dispatch();

    assert_eq!(response.status(),Status::Ok);
}

#[test]
fn test_get_tmp(){
    let client = build_client();

    let response = client.get("/wiki/tmp")
        .dispatch();

    assert_eq!(response.status(),Status::Ok);
}

#[test]
fn test_save_tmp(){
    let client = build_client();

    let query = format!("name={}&field_to_ignore=true&body={}","tmp2","some content2");

    let response = client.post("/wiki/tmp")
        .header(ContentType::Form)
        .body(&query)
        .dispatch();

    assert_eq!(response.status(),Status::Ok);
}

#[test]
fn test_delete_tmp(){
    let client = build_client();

    let response = client.delete("/wiki/tmp")
        .dispatch();

    // SeeOther == redirect
    assert_eq!(response.status(),Status::SeeOther);
}
