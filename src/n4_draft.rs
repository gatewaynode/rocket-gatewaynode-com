use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::time::{UNIX_EPOCH};
use std::collections::HashMap;

use markdown;
use serde_derive::{Serialize, Deserialize};
use chrono;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PageContent {
    markdown: MDContent,
    // html: HTMLContent,
    json: JSONContent,
    css: CSSContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MDContent {
    created: NaiveDateTime,
    title: String,
    body: String,
}

impl Default for MDContent {
    fn default() -> Self {
        MDContent { 
            created: NaiveDateTime::from_timestamp(0, 0),
            title: String::from("None"),
            body: String::from("None"),
        }
    }
}

// #[derive(Serialize, Deserialize, Debug, Default)]
// pub struct HTMLContent {
//     created: NaiveDateTime,
//     title: String,
//     body: String,
// }

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JSONContent {
    payload: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CSSContent {
    payload: String,
}

pub fn read_md_dir(dir: &str) -> Vec<MDContent> {
    let paths = fs::read_dir(dir).unwrap();
    
    let mut contents: Vec<MDContent> = Vec::new();

    for item in paths {
        let this_path = &item.unwrap().path();
        if this_path.extension().unwrap() == "md" {
            let new_content = MDContent {
                created: read_file_creation_time(&this_path),
                title: String::from(this_path.file_stem().unwrap().to_string_lossy()),
                body: read_markdown_from_path(&this_path),
            };
            contents.push(new_content);
        }
    }
    contents.sort_unstable_by_key(|x| x.created);
    contents
}


pub fn read_full_dir_sorted(dir: &str) -> Vec<PageContent> {
    let paths = fs::read_dir(dir).unwrap();
    
    let mut pages: HashMap<String, PageContent> = HashMap::new();

    for item in paths {
        let this_path = &item.unwrap().path(); // Just easier this way
        if !&this_path.is_dir() {
            let file_stem: String = String::from(this_path.file_stem().unwrap().to_string_lossy());
            if !pages.contains_key(&file_stem) {
                pages.insert(file_stem.clone(), PageContent::default());
            } 
            if this_path.extension().unwrap() == "md" {
                let new_content = MDContent {
                    created: read_file_creation_time(&this_path),
                    title: file_stem.clone(),
                    body: read_markdown_from_path(&this_path),
                };
                pages.get_mut(&file_stem).unwrap().markdown = new_content;
            } else if this_path.extension().unwrap() == "json" {
                let new_content = JSONContent {
                    payload: read_json_from_path(&this_path),
                };
                pages.get_mut(&file_stem).unwrap().json = new_content;
            } else if this_path.extension().unwrap() == "css" {
                let new_content = CSSContent {
                    payload: read_css_from_path(&this_path),
                };
                pages.get_mut(&file_stem).unwrap().css = new_content;
            }
        }
    }
    // Convert to Vec for sorting
    let mut contents: Vec<PageContent> = Vec::new();
    for (_key, value) in pages.drain() {
        contents.push(value);
    };

    contents.sort_unstable_by_key(|x| x.markdown.created);
    contents
}

pub fn read_single_page(this_path: &std::path::Path) -> PageContent {
    let mut page_content: PageContent = PageContent::default();
    let file_stem: String = String::from(this_path.file_stem().unwrap().to_string_lossy());

    // Load markdown first
    if this_path.extension().unwrap() == "md" {
        page_content.markdown = MDContent {
            created: read_file_creation_time(&this_path),
            title: file_stem,
            body: read_markdown_from_path(&this_path),
        };
    } else if check_path_alternatives(&this_path, "json") {
        let replaced_path_ext: String = this_path.to_string_lossy().replace(".md", ".json");
        let new_path: &std::path::Path = Path::new(&replaced_path_ext);
        page_content.json = JSONContent {
            payload: read_json_from_path(&new_path),
        };
    } else if check_path_alternatives(&this_path, "css") {
        let replaced_path_ext: String = this_path.to_string_lossy().replace(".md", ".css");
        let new_path: &std::path::Path = Path::new(&replaced_path_ext);
        page_content.css = CSSContent {
            payload: read_css_from_path(&new_path),
        };
    }
    page_content
}

fn check_path_alternatives(this_path: &std::path::Path, extension: &str) -> bool {
    let path_check_str: String = this_path.to_string_lossy().replace(".md", extension);
    let new_path: &std::path::Path = Path::new(&path_check_str);
    new_path.exists()
}

pub fn read_file_creation_time(path: &std::path::Path) -> NaiveDateTime {
    let metadata = fs::metadata(path).expect("Not found");

    let _ = match metadata.created() {
        Err(why) => panic!("Couldn't get file metadata: {}", why),
        Ok(_time) => {
            let _temp_time = _time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
            return NaiveDateTime::from_timestamp(_temp_time, 0)
        }
    };
}

pub fn read_markdown_from_path(path: &std::path::Path) -> String {
    let mut content = String::new();
    let mut _file = match fs::File::open(&path) {
        Err(why) => panic!("Couldn't open file: {}", why),
        Ok(mut _file) => {
            match _file.read_to_string(&mut content) {
                Err(why) => panic!("Couldn't read file: {}", why),
                Ok(_) => {
                    return markdown::to_html(&content)
                }
            }
        },
    };
}

pub fn read_html_from_path(path: &std::path::Path) -> String {
    let mut content = String::new();
    let mut _file = match fs::File::open(&path) {
        Err(why) => panic!("Couldn't open file: {}", why),
        Ok(mut _file) => {
            match _file.read_to_string(&mut content) {
                Err(why) => panic!("Couldn't read file: {}", why),
                Ok(_) => {
                    return content
                }
            }
        },
    };
}
pub fn read_json_from_path(path: &std::path::Path) -> String {
    let mut content = String::new();
    let mut _file = match fs::File::open(&path) {
        Err(why) => panic!("Couldn't open file: {}", why),
        Ok(mut _file) => {
            match _file.read_to_string(&mut content) {
                Err(why) => panic!("Couldn't read file: {}", why),
                Ok(_) => {
                    return content
                }
            }
        },
    };
}

pub fn read_css_from_path(path: &std::path::Path) -> String {
    let mut content = String::new();
    let mut _file = match fs::File::open(&path) {
        Err(why) => panic!("Couldn't open file: {}", why),
        Ok(mut _file) => {
            match _file.read_to_string(&mut content) {
                Err(why) => panic!("Couldn't read file: {}", why),
                Ok(_) => {
                    return content
                }
            }
        },
    };
}