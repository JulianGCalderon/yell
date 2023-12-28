use gtk::glib::{self, Cast};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::application_window::ApplicationWindow;

#[derive(Debug, Default)]
pub struct Application {}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "YellApplication";
    type Type = super::Application;
    type ParentType = gtk::Application;
}

impl ObjectImpl for Application {}

impl ApplicationImpl for Application {
    fn activate(&self) {
        self.parent_activate();

        let application_window = ApplicationWindow::new(self.obj().upcast_ref());
        application_window.present();
    }
}

impl GtkApplicationImpl for Application {}
