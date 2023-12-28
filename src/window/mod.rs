mod imp;

use gtk::gio::ListStore;
use reqwest::blocking::Client;
use rustube::blocking::Video;
use rustube::Id;
use std::error::Error;
use std::fs;

use crate::video::{VideoData, VideoObject, VideoResponse};
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, BuilderListItemFactory, BuilderScope, SingleSelection};

const MAX_RESULTS: usize = 50;
const REQUEST_TYPE: &str = "video";
const REQUEST_PART: &str = "snippet";
const SEARCH_URL: &str = "https://www.googleapis.com/youtube/v3/search";

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn query_videos(&self) {
        let buffer = self.imp().search_entry.buffer();
        let query = buffer.text().to_string();
        if query.is_empty() {
            return;
        }

        if let Err(err) = self.send_query(query) {
            eprintln!("Could not query youtube, with error: {:?}", err);
        }
    }

    fn send_query(&self, query: String) -> Result<(), Box<dyn Error>> {
        let key = dotenv::var("API_KEY").expect("Could not get API key from env. variable");

        let max_results = MAX_RESULTS.to_string();

        let response = self
            .client()
            .get(SEARCH_URL)
            .query(&[
                ("part", REQUEST_PART),
                ("type", REQUEST_TYPE),
                ("max_results", &max_results),
                ("key", &key),
                ("q", &query),
            ])
            .header("key", key)
            .send()?
            .text()?;

        // let _ = query;
        // let response = fs::read_to_string("demo_response.json").unwrap();

        let videos = serde_json::from_str::<VideoResponse>(&response)?.into();

        self.set_results(videos);

        Ok(())
    }

    fn set_results(&self, videos: Vec<VideoObject>) {
        self.results().remove_all();
        self.results().extend_from_slice(&videos);
    }

    fn download_selected(&self) {
        let selection_model = self
            .imp()
            .results_list
            .model()
            .expect("Could not get selection model");

        let selected_bitset = selection_model.selection();

        if selected_bitset.is_empty() {
            return;
        }

        let selected_index = selected_bitset.nth(0);

        let selected = self
            .results()
            .item(selected_index)
            .expect("Selected item should exist")
            .downcast::<VideoObject>()
            .expect("Item should be VideoObject");

        let video_id = selected.property("video_id");

        if let Err(err) = self.download_video(video_id) {
            eprintln!("Could not download video, with error: {:?}", err);
        }
    }

    fn download_video(&self, video_id: String) -> Result<(), Box<dyn Error>> {
        let video_id = Id::from_string(video_id)?;
        let video = Video::from_id(video_id)?;

        video
            .best_quality()
            .ok_or("Could not get best quality")?
            .blocking_download()?;

        Ok(())
    }

    fn setup_callbacks(&self) {
        self.imp()
            .search_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.query_videos();
            }));

        self.imp()
            .search_entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.query_videos();
            }));

        self.imp()
            .download_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.download_selected();
            }));
    }

    fn setup_results(&self) {
        let model = ListStore::new::<VideoObject>();
        self.imp().results.replace(Some(model));

        let selection_model = SingleSelection::new(Some(self.results()));
        self.imp().results_list.set_model(Some(&selection_model));
    }

    fn results(&self) -> ListStore {
        self.imp()
            .results
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn setup_client(&self) {
        self.imp().client.replace(Some(Client::new()));
    }

    fn client(&self) -> Client {
        self.imp()
            .client
            .borrow()
            .clone()
            .expect("Could not get client")
    }

    fn setup_factory(&self) {
        let factory = BuilderListItemFactory::from_resource(
            None::<&BuilderScope>,
            "/juliangcalderon/yell/ui/video.ui",
        );

        self.imp().results_list.set_factory(Some(&factory));
    }
}
