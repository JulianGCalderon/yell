use std::{fs, path::PathBuf};

use rustube::{Id, Video};

use crate::{video_object::VideoResponse, BoxSendResult};

const MAX_RESULTS: usize = 50;
const REQUEST_TYPE: &str = "video";
const REQUEST_PART: &str = "snippet";
const SEARCH_URL: &str = "https://www.googleapis.com/youtube/v3/search";

#[derive(Default, Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

        Self { client }
    }

    pub async fn query(&self, q: String) -> BoxSendResult<VideoResponse> {
        // let key = dotenv::var("API_KEY").expect("Could not get API key from env. variable");
        //
        // let max_results = MAX_RESULTS.to_string();
        //
        // let response = self
        //     .client
        //     .get(SEARCH_URL)
        //     .query(&[
        //         ("part", REQUEST_PART),
        //         ("type", REQUEST_TYPE),
        //         ("max_results", &max_results),
        //         ("key", &key),
        //         ("q", &q),
        //     ])
        //     .header("key", key)
        //     .send()
        //     .await?
        //     .text()
        //     .await?;

        let _ = (
            REQUEST_PART,
            REQUEST_TYPE,
            MAX_RESULTS,
            SEARCH_URL,
            q,
            &self.client,
        );
        let response = fs::read_to_string("target/demo.json").unwrap();

        Ok(serde_json::from_str::<VideoResponse>(&response)?)
    }

    pub async fn download(&self, id: String, path: PathBuf) -> BoxSendResult<()> {
        let video_id = Id::from_string(id)?;
        let video = Video::from_id(video_id).await?;

        video
            .best_quality()
            .ok_or("Could not get best quality")?
            .download_to(path)
            .await?;

        Ok(())
    }
}
