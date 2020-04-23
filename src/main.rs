#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
extern crate diesel;
extern crate tera;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket_gatewaynode_com::*;
use rocket_gatewaynode_com::models::{Post};
use tera::Context;
use std::collections::HashMap;

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;


fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .launch();
}

#[get("/")]
fn index() -> Template {
    let all_posts: Vec<Post> = read_all_posts();
    let mut serialized = HashMap::new();
    for post in all_posts {
        serialized.insert(format!("post-{}", &post.id), post);
    }
    Template::render("front", &serialized)
}
