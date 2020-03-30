#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

extern crate rocket_contrib;

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

#[derive(Serialize)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .launch();
}

#[get("/")]
fn index() -> Template {
    let context = Post {
        id: 0,
        title: String::from("This"),
        body: String::from("works"),
    };
    Template::render("front", &context)
}
