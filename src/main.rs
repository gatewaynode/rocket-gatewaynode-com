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

/// A simple implementation of Rocket for a personal blog.  Database content managed by https://github.com/gatewaynode/nautilus
/// <!-- __tera_context {{ __tera_context }} So useful, so obscure -->

#[derive(Serialize, Deserialize, Debug)]
struct GeneralContentList {
    posts: Vec<Post>,
    regular_links: Vec<Link>,
    thingy_links: Vec<Link>,
    tut_links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostContentList {
    posts: Vec<Post>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkContentList {
    links: Vec<Link>,
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, list_all_posts, a_post, list_all_links, list_rust_posts, list_python_posts, list_bash_posts])
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .launch();
}

#[get("/")]
fn index() -> Template {
    let all_posts_raw = read_all_posts();
    let regular_links_raw = read_links_by_filter_limit(String::from("regular"), 10);
    let thingy_links_raw = read_links_by_filter_limit(String::from("thingy"), 10);
    let tut_links_raw = read_links_by_filter_limit(String::from("tut"), 10);
    let all_posts = GeneralContentList{
        posts: all_posts_raw,
        regular_links: regular_links_raw,
        thingy_links: thingy_links_raw,
        tut_links: tut_links_raw,
    };

    Template::render("front", &all_posts)
}

#[get("/posts")]
fn list_all_posts() -> Template {
    let all_posts = PostContentList {
        posts: read_all_posts(),
    };

    Template::render("all_posts", &all_posts)
}

#[get("/post/<post_id>")]
fn a_post(post_id: i32) -> Template {
    let post = read_post(post_id);

    Template::render("a_post", &post)
}

#[get("/links")]
fn list_all_links() -> Template {
    let all_links = LinkContentList {
        links: read_all_links(),
    };

    Template::render("all_links", &all_links)
}

#[get("/rust")]
fn list_rust_posts() -> Template {
    let rustlings = PostContentList {
        posts: read_posts_by_filter_limit(String::from("rust"), 999),
    };
    Template::render("rustlings", &rustlings)
}

#[get("/python")]
fn list_python_posts() -> Template {
    let snakes = PostContentList {
        posts: read_posts_by_filter_limit(String::from("python"), 999),
    };
    Template::render("snakes", &snakes)
}

#[get("/bash")]
fn list_bash_posts() -> Template {
    let shells = PostContentList {
        posts: read_posts_by_filter_limit(String::from("bash"), 999),
    };
    Template::render("shells", &shells)
}
