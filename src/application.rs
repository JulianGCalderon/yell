use crate::config;
use gtk::{gio, glib};

mod imp {
    use adw::prelude::*;
    use adw::subclass::application::AdwApplicationImpl;
    use adw::subclass::prelude::*;
    use gtk::glib::{self, Cast};

    use crate::application_window::ApplicationWindow;

    #[derive(Debug, Default)]
    pub struct Application {}

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "YellApplication";
        type Type = super::Application;
        type ParentType = adw::Application;
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
    impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Application {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", config::APP_ID)
            .property("resource-base-path", config::APP_IDPATH)
            .build()
    }
}
