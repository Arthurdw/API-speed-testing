use actix_web::{App, get, http, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use rand::distributions::Alphanumeric;
use rand::Rng;

#[derive(Serialize)]
struct KeyResponse {
    key: String
}

#[derive(Deserialize)]
struct KeyArgs {
    length: usize,
}

#[get("/key")]
async fn key_service(query: web::Query<KeyArgs>) -> impl Responder {
    let key = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(query.length)
        .map(char::from)
        .collect();

    HttpResponse::Ok()
        .set_header(http::header::CONTENT_TYPE, "application/json")
        .json(KeyResponse { key })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    println!("Running a server on http://{}", addr);

    HttpServer::new(|| {
        App::new()
            .service(key_service)
    }).bind(addr)?.run().await
}
