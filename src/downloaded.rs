use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

extern crate serde_json;
use crate::structs;
use structs::Nerdcast;

const DOWNLOADED_PATH : &str = "./data/downloaded.json"; //TODO: get from env var

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

fn read_downloaded() -> Vec<Nerdcast> {
    let mut downloaded: Vec<Nerdcast> = vec![];
    let content = read_content_if_exists();
    if !content.is_empty() {
        downloaded = serde_json::from_str(&content).unwrap();
    }
    downloaded
}

fn save_downloaded(downloaded: Vec<Nerdcast>) ->  Vec<Nerdcast>
{
    let content = serde_json::to_string_pretty(&downloaded).expect("Error while serializing downloaded.json");
    let mut file = File::create(DOWNLOADED_PATH).expect("Failed to created downloaded.json file");
    file.write_all(content.as_bytes()).expect("Failed to write downloade.json.file");
    downloaded
}

pub fn check_downloaded(id: i64) -> bool {
    let downloaded = read_downloaded();
    for downloaded_episode in downloaded
    {
        if downloaded_episode.id == id
        {
            return true;
        }
    }
    return false;
}

pub fn add_downloaded(episode: Nerdcast) -> Vec<Nerdcast> {
    let mut downloaded = read_downloaded();
    downloaded.push(episode);

    save_downloaded(downloaded)
}