#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![index])
        .mount("/", FileServer::from(relative!("dist")))
}
