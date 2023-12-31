mod imp;

use gtk::gio::File;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::glib::{self, ObjectExt};

glib::wrapper! {
    pub struct VideoPreview(ObjectSubclass<imp::VideoPreview>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable, gtk::Orientable;
}

impl VideoPreview {
    fn setup(&self) {
        self.setup_default();

        self.connect_video_notify(Self::update);
    }

    fn setup_default(&self) {
        let picture = File::for_path("resources/images/default-image.webp");
        self.imp().thumbnail.set_file(Some(&picture));
    }

    fn update(&self) {
        let video = self.imp().video.borrow().clone();

        let thumbnail: String = video.property("thumbnail");
        let thumbnail = File::for_uri(&thumbnail);

        self.imp().thumbnail.set_file(Some(&thumbnail));
    }
}
