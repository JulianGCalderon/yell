use html_escape::decode_html_entities;
use serde::Deserialize;

use super::{VideoData, VideoObject};

#[derive(Deserialize)]
pub struct VideoResponse {
    items: Vec<VideoItem>,
}

#[derive(Deserialize)]
struct VideoItem {
    id: VideoId,
    snippet: VideoSnippet,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoId {
    video_id: String,
}

#[derive(Deserialize)]
struct VideoSnippet {
    title: String,
    description: String,
    thumbnails: VideoThumbnails,
}

#[derive(Deserialize)]
struct VideoThumbnails {
    high: VideoThumbnail,
}

#[derive(Deserialize, Debug, Default)]
struct VideoThumbnail {
    url: String,
}

impl From<VideoItem> for VideoData {
    fn from(value: VideoItem) -> Self {
        let title = decode_html_entities(&value.snippet.title).to_string();

        Self {
            title,
            thumbnail: value.snippet.thumbnails.high.url,
            description: value.snippet.description,
            id: value.id.video_id,
        }
    }
}

impl From<VideoResponse> for Vec<VideoObject> {
    fn from(value: VideoResponse) -> Self {
        value
            .items
            .into_iter()
            .map(VideoData::from)
            .map(VideoObject::new)
            .collect()
    }
}
