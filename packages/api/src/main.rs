// #[macro_use] extern crate rocket;

// // Try visiting:
// // http://127.0.0.1:8000/downloads
// #[get("/downloads")]
// fn downloads() -> &'static str {
//     "downloads"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![downloads])
// }

use rocketapi::server::init_server;
use rocket::error::Error;
use std::process::exit;

#[rocket::main]
async fn main() -> Result<(), Error> {
    // start the server
    match init_server().await {
        Ok(server) => server.launch().await,
        Err(e) => {
            println!("{}", e.to_string());
            exit(1)
        }   
    }
}