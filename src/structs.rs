extern crate serde_derive;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Nerdcast {
    pub id: i64,
    pub url: String,
    pub published_at: String,
    pub pub_date: String,
    pub modified_at: String,
    pub duration: i64,
    pub title: String,
    pub slug: String,
    pub episode: String,
    pub product: String,
    pub product_name: String,
    pub friendly_post_type: String,
    pub friendly_post_type_slug: String,
    pub friendly_post_time: String,
    pub friendly_post_date: String,
    pub subject: String,
    pub image: String,
    pub image_alt: ::serde_json::Value,
    pub thumbnails: Thumbnails,
    pub audio_high: String,
    pub audio_medium: String,
    pub audio_low: String,
    pub audio_zip: String,
    pub insertions: Vec<Insertion>,
    pub description: String,
    #[serde(rename = "jump-to-time")]
    pub jump_to_time: JumpToTime,
    pub guests: String,
    pub post_type_class: String,
}



#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Thumbnails {
    #[serde(rename = "img-4x3-355x266")]
    pub img4_x3355_x266: String,
    #[serde(rename = "img-16x9-1210x544")]
    pub img16_x91210_x544: String,
    #[serde(rename = "img-16x9-760x428")]
    pub img16_x9760_x428: String,
    #[serde(rename = "img-4x6-448x644")]
    pub img4_x6448_x644: String,
    #[serde(rename = "img-1x1-3000x3000")]
    pub img1_x13000_x3000: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Insertion {
    pub id: i64,
    pub title: String,
    pub image: String,
    pub link: String,
    #[serde(rename = "button-title")]
    pub button_title: String,
    #[serde(rename = "start-time")]
    pub start_time: i64,
    #[serde(rename = "end-time")]
    pub end_time: i64,
    pub sound: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JumpToTime {
    pub test: String,
    #[serde(rename = "start-time")]
    pub start_time: i64,
    #[serde(rename = "end-time")]
    pub end_time: i64,
}
