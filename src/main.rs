#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate rocket;
extern crate rocket_contrib;

use rocket::response::NamedFile;
use rocket_contrib::Template;

use std::io::{ BufReader, Error, Read };
use std::path::{ Path, PathBuf };


fn main() {
    rocket::ignite()
           .mount("/", routes![
                // Navbar handlers
                index,
                mailing_list,
                contact,
                // Info page handlers
                mailing_list_item,
                // Handlers to get the site working
                static_files,
                // External API handlers
                mailing_list_raw,
           ])
           .attach(Template::fairing())
           .launch();
}
// Navbar calls
#[get("/")]
fn index() -> Option<Template> {
    Some(Template::render("index", Nav::new(0)))
}

#[get("/mailing-list")]
fn mailing_list() -> Option<Template> {
    Some(Template::render("mailing-list", Nav::new(1)))
}

#[get("/contact")]
fn contact() -> Option<Template> {
    Some(Template::render("contact", Nav::new(2)))
}

// Calls for non navbar pages but still contain data to look at
#[get("/mailing-list/<year>/<month>")]
fn mailing_list_item(year: String, month: String) -> Result<Template, Error> {
    let mut buffer = String::new();
    let file = NamedFile::open(&format!("rust-dev/{}-{}.txt", year, month))?;
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut buffer)?;
    let mailing_list = MailingList::new(buffer);
    Ok(Template::render("mailing-list-item", mailing_list))
}

// Calls for the site to grab resources and function
#[get("/static/<file..>", rank=1)]
fn static_files(file: PathBuf) -> Option<NamedFile>{
    NamedFile::open(Path::new("static").join(file)).ok()
}

// API calls for external users to grab the data they want without any css or formatting.
#[get("/rust-dev/<year>/<month>")]
fn mailing_list_raw(year: String, month: String) -> Result<NamedFile, Error> {
    NamedFile::open(&format!("rust-dev/{}-{}.txt", year, month))
}

#[derive(Deserialize, Serialize)]
pub struct Nav {
    url: i32,
}

impl Nav {
    pub fn new(url: i32) -> Self {
        Self { url }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MailingList {
    data: String,
    url: i32,
}

impl MailingList {
    pub fn new(data: String) -> Self {
        Self { data, url: -1 }
    }
}
