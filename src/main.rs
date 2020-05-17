#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;
extern crate diesel;
extern crate tera;
// extern crate rocket_cors;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::response::Redirect;
use rocket::http::Method;
use rocket::{get, routes};
use rocket_gatewaynode_com::*;
use rocket_gatewaynode_com::models::{Post, Link};
// use rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_cors;
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
    let allowed_origins = AllowedOrigins::some_exact(&["https://commento.io", "http://localhost:8000","http://127.0.0.1:8000"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS");

    rocket::ignite()
        .attach(Template::fairing())
        .attach(cors)
        .mount("/", routes![
            index,
            favicon,
            list_all_posts,
            a_post,
            list_all_links,
            list_rust_posts,
            list_python_posts,
            list_bash_posts,
            generate_sitemap,
            let_the_robots_free
        ])
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .launch();
}

#[get("/")]
fn index() -> Template {
    let all_posts_raw = read_some_posts(10);
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

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::permanent("/static/images/favicon.ico")
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

#[get("/sitemap.xml")]
fn generate_sitemap() -> Template {
    let all_posts = PostContentList{
        posts: read_all_posts(),
    };
    Template::render("sitemap", &all_posts)
}

#[get("/robots.txt")]
fn let_the_robots_free() -> String {
    String::from("User-agents: *
Allow: *

Sitemap: https://gatewaynode.com/sitemap.xml")
}
