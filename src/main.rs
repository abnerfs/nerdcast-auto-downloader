extern crate reqwest;
mod nerdcasts;
mod downloaded;

fn main() {
    const DOWNLOAD_PATH : &str = "./data/";

    let (nerdcasts, _content) = nerdcasts::get_last_nerdcasts();
    println!("{} nerdcasts loaded", nerdcasts.len());

    for nerdcast_episode in nerdcasts
    {
        if downloaded::check_downloaded(nerdcast_episode.id)
        {
            continue;
        }

        let download_url = &nerdcast_episode.audio_high;
        let file_name = format!("{}.mp3", nerdcast_episode.get_full_name());
        println!("{}", file_name);
        nerdcasts::download_file(download_url, DOWNLOAD_PATH, &file_name);
        downloaded::add_downloaded(nerdcast_episode.id);
    }
}

