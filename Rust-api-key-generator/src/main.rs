#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rand::Rng;
use rocket::response::content::Json;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~.";

#[get("/key?<length>")]
fn generate_key(length: u128) -> Json<String> {
    let mut rng = rand::thread_rng();

    let password: String = (0..length).map(|_| {
        let idx = rng.gen_range(0, CHARSET.len());
        CHARSET[idx] as char
    }).collect();

    Json(format!("{{'key': '{}'}}", password))
}

fn main() {
    rocket::ignite().mount("/api/generator/", routes![generate_key]).launch();
}
