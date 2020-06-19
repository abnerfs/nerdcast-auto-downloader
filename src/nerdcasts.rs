extern crate reqwest;

use std::io;
use std::fs::File;


use crate::structs;
use structs::Nerdcast;



impl Nerdcast {
    pub fn get_full_name(&self) -> String {
        format!("{}_{}_{}", self.product_name.to_lowercase().replace(" ", "_"), self.episode, self.slug)
    }
}

pub fn get_last_nerdcasts(amount: i8) -> (Vec<Nerdcast>, String )
{
    let amount_download = if amount == 0 { 4 }  else { amount };

    let url = format!("https://jovemnerd.com.br/wp-json/jovemnerd/v1/nerdcasts?per_page={}&page=1", amount_download);

    let body : String = reqwest::blocking::get(&url)
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
            .expect("Error while getting file name")
    };

    let fname = if file_name.is_empty() { get_file_name() } else { file_name };
    let full_path = format!("{}{}", path, fname);
    println!("File will be located at {}", full_path);

    let mut file = File::create(full_path).expect("Failed to create file");

    io::copy(&mut response, &mut file).expect("Failed to download file");
}

