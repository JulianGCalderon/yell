mod imp;

use gtk::gio::ListStore;
use reqwest::blocking::Client;
use std::error::Error;
use std::fs;

use crate::video::{VideoData, VideoObject};
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, BuilderListItemFactory, BuilderScope, SingleSelection};

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
        // let key = dotenv::var("API_KEY").expect("Could not get API key from env. variable");
        //
        // let response = self
        //     .client()
        //     .get("https://www.googleapis.com/youtube/v3/search")
        //     .query(&[("part", "snippet"), ("key", &key), ("q", &query)])
        //     .header("key", key)
        //     .send()?
        //     .text()?;

        let _ = query;
        let response_text = fs::read_to_string("demo_response.json").unwrap();

        let mut response_json = serde_json::from_str::<serde_json::Value>(&response_text)?;

        let videos_json = response_json
            .get_mut("items")
            .ok_or("Response should have 'items' object")?
            .as_array_mut()
            .ok_or("'items' should be an array")?;

        let videos = videos_json
            .iter_mut()
            .map(|video| video.take())
            .flat_map(VideoData::try_from)
            .map(VideoObject::new)
            .collect::<Vec<VideoObject>>();

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

        unimplemented!("Downloading {:?}", selected);
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
