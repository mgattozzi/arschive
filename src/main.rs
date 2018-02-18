#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::response::NamedFile;
use std::io::Error;

fn main() {
    rocket::ignite()
           .mount("/", routes![hello])
           .launch();
}

#[get("/rust-dev/<year>/<month>")]
fn hello(year: String, month: String) -> Result<NamedFile, Error> {
    NamedFile::open(&format!("rust-dev/{}-{}.txt", year, month))
}

