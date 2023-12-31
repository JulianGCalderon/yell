mod deserialize;
mod imp;

pub use deserialize::VideoResponse;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct VideoObject(ObjectSubclass<imp::VideoObject>);
}

impl VideoObject {
    pub fn new(data: VideoData) -> Self {
        Object::builder()
            .property("title", data.title)
            .property("id", data.id)
            .property("thumbnail", data.thumbnail)
            .property("description", data.description)
            .build()
    }
}

impl Default for VideoObject {
    fn default() -> Self {
        Object::builder().build()
    }
}

#[derive(Default, Debug)]
pub struct VideoData {
    pub title: String,
    pub thumbnail: String,
    pub description: String,
    pub id: String,
}
