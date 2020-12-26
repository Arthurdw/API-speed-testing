#[macro_use]
extern crate tower_web;

use rand::distributions::Alphanumeric;
use rand::Rng;
use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
struct KeyResource;

#[derive(Debug, Extract)]
struct KeyArgs {
    length: usize
}

#[derive(Debug, Response)]
struct KeyResponse {
    key: String
}

impl_web! {
    impl KeyResource {
        #[get("/key")]
        #[content_type("application/json")]
        fn get_key(&self, query_string: KeyArgs) -> Result<KeyResponse, ()> {
            let key = rand::thread_rng()
                .sample_iter(Alphanumeric)
                .take(query_string.length)
                .map(char::from)
                .collect();

            Ok(KeyResponse { key })
        }
    }
}


fn main() {
    let addr = "127.0.0.1:8000".parse().expect("Invalid address!");
    println!("Running a server on http://{}", addr);

    ServiceBuilder::new()
        .resource(KeyResource)
        .run(&addr)
        .unwrap()
}
