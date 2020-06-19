extern crate reqwest;

#[path = "structs.rs"]
mod structs;

use structs::Nerdcast;

use std::io;
use std::fs::File;

impl Nerdcast {
    pub fn get_full_name(&self) -> String {
        format!("{} {} - {}", self.product_name, self.episode, self.title)
    }
}

pub fn get_last_nerdcasts() -> (Vec<Nerdcast>, String )
{
    let url = "https://jovemnerd.com.br/wp-json/jovemnerd/v1/nerdcasts?per_page=4&page=1";
    let body : String = reqwest::blocking::get(url)
    .unwrap()
    .text()
    .unwrap();

    let retorno_nerdcast = serde_json::from_str::<Vec<Nerdcast>>(&body).expect("Error while getting latest nerdcasts");
    (retorno_nerdcast, body)
}


pub fn download_file(url: &str, path: &str, file_name: &str) {
    let mut response = reqwest::blocking::get(url).expect("Failed to send download request");

    let get_file_name = | | {
        response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .expect("Error while gettinf file name")
    };

    let fname = if file_name.is_empty() { get_file_name() } else { file_name };
    let full_path = format!("{}{}", path, fname);
    println!("File will be located at {}", full_path);

    let mut file = File::create(full_path).expect("Failed to create file");

    io::copy(&mut response, &mut file).expect("Failed to download file");
}

