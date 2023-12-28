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
            .property("video-id", data.video_id)
            .build()
    }
}

#[derive(Default, Debug)]
pub struct VideoData {
    pub title: String,
    pub video_id: String,
}
