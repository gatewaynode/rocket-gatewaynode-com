#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
extern crate diesel;
extern crate tera;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket_gatewaynode_com::*;
use rocket_gatewaynode_com::models::{Post};

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
    struct PostList { posts: Vec<Post>, }

    let all_posts_raw = read_all_posts();
    let all_posts = PostList{ posts: all_posts_raw };

    Template::render("front", &all_posts)
}
