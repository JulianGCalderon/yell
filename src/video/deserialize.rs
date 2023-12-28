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
}

impl From<VideoItem> for VideoData {
    fn from(value: VideoItem) -> Self {
        Self {
            title: value.snippet.title,
            video_id: value.id.video_id,
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
