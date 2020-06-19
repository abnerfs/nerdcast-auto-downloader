extern crate reqwest;
mod nerdcasts;

fn main() {
    const DOWNLOAD_PATH : &str = "D:\\Abner\\hello-rust\\downloads\\";

    let (nerdcasts, _content) = nerdcasts::get_last_nerdcasts();
    println!("{} nerdcasts loaded", nerdcasts.len());

    for nerdcast_episode in nerdcasts
    {
        let download_url = &nerdcast_episode.audio_high;
        let file_name = format!("{}.mp3", nerdcast_episode.get_full_name());
        nerdcasts::download_file(download_url, DOWNLOAD_PATH, &file_name);
    }
}

