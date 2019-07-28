use super::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};

fn build_client() -> Client{
    return Client::new(rocket("8080".to_string())).unwrap();
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

    let query = format!("name={}","tmp2");

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

    assert_eq!(response.status(),Status::Ok);
}
