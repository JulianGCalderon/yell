use crate::config;
use gtk::{gio, glib};

mod imp {
    use adw::prelude::*;
    use adw::subclass::application::AdwApplicationImpl;
    use adw::subclass::prelude::*;
    use gtk::{
        gdk::Display,
        glib::{self, Cast},
        CssProvider,
    };

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

        fn startup(&self) {
            self.parent_startup();

            let provider = CssProvider::new();
            provider.load_from_data(include_str!("../resources/style.css"));

            // Add the provider to the default screen
            gtk::style_context_add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
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
