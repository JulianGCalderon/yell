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
    #[property(name = "id", get, set, type = String, member = id)]
    #[property(name = "thumbnail", get, set, type = String, member = thumbnail)]
    #[property(name = "description", get, set, type = String, member = description)]
    #[property(name = "published-at", get, set, type = String, member = published_at)]
    #[property(name = "channel-title", get, set, type = String, member = channel_title)]
    pub data: RefCell<VideoData>,
}

#[glib::object_subclass]
impl ObjectSubclass for VideoObject {
    const NAME: &'static str = "YellVideoObject";
    type Type = super::VideoObject;
}

#[glib::derived_properties]
impl ObjectImpl for VideoObject {}
