mod imp;

use gtk::gio::File;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::glib::{self, closure, Object};
use gtk::prelude::{GObjectPropertyExpressionExt, TextViewExt};
use gtk::Widget;

use crate::video_object::VideoObject;

glib::wrapper! {
    pub struct VideoPreview(ObjectSubclass<imp::VideoPreview>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable, gtk::Orientable;
}

impl VideoPreview {
    fn setup(&self) {
        let title = self.imp().title.clone();

        self.property_expression("video")
            .chain_property::<VideoObject>("title")
            .bind(&title, "label", Widget::NONE);

        let buffer = self.imp().description.buffer();

        self.property_expression("video")
            .chain_property::<VideoObject>("description")
            .bind(&buffer, "text", Widget::NONE);

        let picture = self.imp().thumbnail.clone();

        self.property_expression("video")
            .chain_property::<VideoObject>("thumbnail")
            .chain_closure::<File>(closure!(|_: Option<Object>, url: &str| {
                if url.is_empty() {
                    File::for_path("resources/images/default-image.webp")
                } else {
                    File::for_uri(&url)
                }
            }))
            .bind(&picture, "file", Widget::NONE);

        let channel_title = self.imp().channel_title.clone();

        self.property_expression("video")
            .chain_property::<VideoObject>("channel-title")
            .bind(&channel_title, "label", Widget::NONE);

        let published_at = self.imp().published_at.clone();

        self.property_expression("video")
            .chain_property::<VideoObject>("published-at")
            .bind(&published_at, "label", Widget::NONE);
    }
}
