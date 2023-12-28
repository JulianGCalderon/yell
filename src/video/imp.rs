use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::VideoData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::VideoObject)]
pub struct VideoObject {
    #[property(name = "title", get, set, type = String, member = title)]
    #[property(name = "video-id", get, set, type = String, member = video_id)]
    pub data: RefCell<VideoData>,
}

#[glib::object_subclass]
impl ObjectSubclass for VideoObject {
    const NAME: &'static str = "YellVideoObject";
    type Type = super::VideoObject;
}

#[glib::derived_properties]
impl ObjectImpl for VideoObject {}
