#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let mut rocket = rocket::build().mount("/", routes![index]);

    // mount Fileserver only when --release is set
    if cfg!(not(debug_assertions)) {
        rocket = rocket.mount("/", FileServer::from(relative!("dist")));
    }

    rocket
}
