extern crate reqwest;

mod nerdcasts;
mod downloaded;
mod structs;

use std::env;
use structs::Nerdcast;
use std::str::FromStr;

fn get_download_path() -> String
{
    let download_path = 
        env::var("NERDCAST_DOWNLOAD_PATH")
            .unwrap_or("./data/".to_string());

    let mut download_formated = download_path.trim_end().replace("\\", "/");
    if !download_formated.ends_with("/")
    {
        download_formated = format!("{}/", download_formated);
    }

    println!("Download Path: {}", download_formated);
    download_formated
}

fn get_amount_download() -> i8
{
    let amount_download_str = 
        env::var("NERDCAST_DOWNLOAD_AMOUNT")
                .unwrap_or("4".to_string());


    let amount_download = i8::from_str(&amount_download_str.trim_end())
        .unwrap_or(4);

    println!("Download {} new podcasts", amount_download);
    amount_download
}

fn main() {

    let download_path : &str  = &get_download_path();
    let amount_download : i8 = get_amount_download();
    let downloaded_path = downloaded::get_downloaded_path();
    println!("Downloaded.json Path {}", downloaded_path);


    let (nerdcasts, _content) = nerdcasts::get_last_nerdcasts(amount_download);

    let nerdcasts_download : Vec<Nerdcast> = nerdcasts
        .clone()
        .into_iter()
        .filter(| nerdcast_episode | !downloaded::check_downloaded(nerdcast_episode.id))
        .collect();

    println!("{} new nerdcasts loaded", nerdcasts_download.len());

    for nerdcast_episode in nerdcasts_download
    {
        let download_url = &nerdcast_episode.audio_high;
        let file_name = format!("{}.mp3", nerdcast_episode.get_full_name());
        println!("{}", file_name);
        nerdcasts::download_file(download_url, download_path, &file_name);
        downloaded::add_downloaded(nerdcast_episode);
    }
}

