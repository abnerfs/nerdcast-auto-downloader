use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::fs;

extern crate serde_json;
use crate::structs;
use structs::Nerdcast;

pub fn get_downloaded_path() -> String
{
    let mut dir = env::current_exe().expect("Error while reading current path");
    dir.pop();
    dir.push("data");
    let parent_path = dir.to_str().unwrap().to_string();
    if !Path::new(&parent_path).exists()
    {
        fs::create_dir(parent_path).expect("Error while trying to create data path");
    }

    dir.push("downloaded.json");
    let path_return = dir.to_str().unwrap().to_string();
    path_return
}


fn read_content_if_exists() -> String {
    let downloaded_path = get_downloaded_path();
    if !Path::new(&downloaded_path).exists() {
        return "".to_string();
    }

    let file = File::open(downloaded_path).expect("Failed to open downloaded.json file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap_or(0);
    contents
}

fn read_downloaded() -> Vec<Nerdcast> {
    let mut downloaded: Vec<Nerdcast> = vec![];
    let content = read_content_if_exists();
    if !content.is_empty() {
        downloaded = serde_json::from_str(&content).expect("Error while parsing downloaded.json");
    }
    downloaded
}

fn save_downloaded(downloaded: Vec<Nerdcast>) ->  Vec<Nerdcast>
{
    let downloaded_path = get_downloaded_path();

    let content = serde_json::to_string_pretty(&downloaded).expect("Error while serializing downloaded.json");
    let mut file = File::create(downloaded_path).expect("Failed to created downloaded.json file");
    file.write_all(content.as_bytes()).expect("Failed to write downloaded.json.file");
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