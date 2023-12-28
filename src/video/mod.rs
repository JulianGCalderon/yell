mod imp;

use std::error::Error;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct VideoObject(ObjectSubclass<imp::VideoObject>);
}

impl VideoObject {
    pub fn new(data: VideoData) -> Self {
        let obj: Self = Object::builder().build();

        *obj.imp().data.borrow_mut() = data;

        obj
    }
}

#[derive(Default, Debug)]
pub struct VideoData {
    pub title: String,
}

impl TryFrom<serde_json::Value> for VideoData {
    type Error = Box<dyn Error>;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let title = value
            .get("snippet")
            .ok_or("There should be a snippet")?
            .get("title")
            .ok_or("There should be a title")?
            .as_str()
            .ok_or("Title should be a snippet")?
            .to_string();

        Ok(Self { title })
    }
}
