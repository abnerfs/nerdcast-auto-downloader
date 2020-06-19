extern crate reqwest;
mod nerdcasts;
mod downloaded;
mod structs;

use structs::Nerdcast;

fn main() {
    const DOWNLOAD_PATH : &str = "./data/";

    let (nerdcasts, _content) = nerdcasts::get_last_nerdcasts();

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
        nerdcasts::download_file(download_url, DOWNLOAD_PATH, &file_name);
        downloaded::add_downloaded(nerdcast_episode);
    }
}

