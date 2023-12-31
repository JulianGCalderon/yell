use gtk::glib::Object;
use html_escape::decode_html_entities;
use serde::Deserialize;

use super::VideoObject;

#[derive(Deserialize)]
pub struct VideoResponse {
    items: Vec<VideoItem>,
}

#[derive(Deserialize)]
pub struct VideoItem {
    id: VideoId,
    snippet: VideoSnippet,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoId {
    video_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoSnippet {
    title: String,
    description: String,
    thumbnails: VideoThumbnails,
    published_at: String,
    channel_title: String,
}

#[derive(Deserialize)]
struct VideoThumbnails {
    high: VideoThumbnail,
}

#[derive(Deserialize, Debug, Default)]
struct VideoThumbnail {
    url: String,
}

impl From<VideoItem> for VideoObject {
    fn from(video: VideoItem) -> Self {
        let title = decode_html_entities(&video.snippet.title).to_string();

        Object::builder()
            .property("title", title)
            .property("thumbnail", video.snippet.thumbnails.high.url)
            .property("description", video.snippet.description)
            .property("id", video.id.video_id)
            .property("channel-title", video.snippet.channel_title)
            .property("published-at", video.snippet.published_at)
            .build()
    }
}

impl From<VideoResponse> for Vec<VideoObject> {
    fn from(value: VideoResponse) -> Self {
        value.items.into_iter().map(VideoObject::from).collect()
    }
}
