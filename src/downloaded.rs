use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

extern crate serde_json;

const DOWNLOADED_PATH : &str = "./data/downloaded.json";

fn read_content_if_exists() -> String {
    if !Path::new(DOWNLOADED_PATH).exists() {
        return "".to_string();
    }

    let file = File::open(DOWNLOADED_PATH).expect("Failed to open downloaded.json file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap_or(0);
    contents
}

fn read_downloaded() -> Vec<i64> {
    let mut downloaded: Vec<i64> = vec![];
    let content = read_content_if_exists();
    if !content.is_empty() {
        downloaded = serde_json::from_str(&content).unwrap();
    }
    downloaded
}

fn save_downloaded(downloaded: Vec<i64>) ->  Vec<i64>
{
    let content = serde_json::to_string(&downloaded).expect("Error while serializing downloaded.json");
    let mut file = File::create(DOWNLOADED_PATH).expect("Failed to created downloaded.json file");
    file.write_all(content.as_bytes()).expect("Failed to write downloade.json.file");
    downloaded
}

pub fn check_downloaded(id: i64) -> bool {
    let downloaded = read_downloaded();
    for id_downloaded in downloaded
    {
        if id_downloaded == id
        {
            return true;
        }
    }
    return false;
}

pub fn add_downloaded(id: i64) -> Vec<i64> {
    let mut downloaded = read_downloaded();
    downloaded.push(id);

    save_downloaded(downloaded)
}