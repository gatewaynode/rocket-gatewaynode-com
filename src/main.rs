#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
extern crate diesel;
extern crate tera;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket_gatewaynode_com::*;
use rocket_gatewaynode_com::models::{Post, Link};

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
    #[derive(Serialize, Deserialize, Debug)]
    struct PostList {
        posts: Vec<Post>,
        regular_links: Vec<Link>,
        thingy_links: Vec<Link>,
        tut_links: Vec<Link>,
    }

    let all_posts_raw = read_all_posts();
    let regular_links_raw = read_links_by_filter_limit(String::from("regular"), 10);
    let thingy_links_raw = read_links_by_filter_limit(String::from("thingy"), 10);
    let tut_links_raw = read_links_by_filter_limit(String::from("tut"), 10);
    let all_posts = PostList{
        posts: all_posts_raw,
        regular_links: regular_links_raw,
        thingy_links: thingy_links_raw,
        tut_links: tut_links_raw,
    };

    Template::render("front", &all_posts)
}
