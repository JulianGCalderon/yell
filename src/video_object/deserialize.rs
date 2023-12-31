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

impl From<VideoItem> for VideoData {
    fn from(video: VideoItem) -> Self {
        let title = decode_html_entities(&video.snippet.title).to_string();

        Self {
            title,
            thumbnail: video.snippet.thumbnails.high.url,
            description: video.snippet.description,
            id: video.id.video_id,
            channel_title: video.snippet.channel_title,
            published_at: video.snippet.published_at,
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
