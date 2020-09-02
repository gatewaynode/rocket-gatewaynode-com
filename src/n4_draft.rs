use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::time::{UNIX_EPOCH};

use markdown;
use serde_derive::{Serialize, Deserialize};
use chrono;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MDContent {
    //created: f64,
    created: NaiveDateTime,
    title: String,
    body: String,
}

pub fn read_dir(dir: &str) -> Vec<MDContent> {
    let paths = fs::read_dir(dir).unwrap();
    
    let mut contents: Vec<MDContent> = Vec::new();

    for item in paths {
        let this_path = &item.unwrap().path();
        if !&this_path.is_dir() {
            if this_path.extension().unwrap() == "md" {
                let new_content = MDContent {
                    created: read_file_creation_time(&this_path),
                    title: String::from(this_path.file_stem().unwrap().to_string_lossy()),
                    body: read_markdown_from_path(&this_path),
                };
                contents.push(new_content);
            } else if this_path.extension().unwrap() == "html" {
                let new_content = MDContent {
                    created: read_file_creation_time(&this_path),
                    title: String::from(this_path.file_stem().unwrap().to_string_lossy()),
                    body: read_html_from_path(&this_path),
                };
                contents.push(new_content);
            } else if this_path.extension().unwrap() == "json" {
                let new_content = MDContent {
                    created: read_file_creation_time(&this_path),
                    title: String::from(this_path.file_stem().unwrap().to_string_lossy()),
                    body: read_json_from_path(&this_path),
                };
                contents.push(new_content);
            }
        }
    }
    contents.sort_unstable_by_key(|x| x.created);
    contents
}

pub fn read_file_creation_time(path: &std::path::Path) -> NaiveDateTime {
    let metadata = fs::metadata(path).expect("Not found");

    // if let Ok(time) = metadata.created() {
    //     time.duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
    // } else {
    //     panic!("Created not supported on this platform");
    // }
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