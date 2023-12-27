mod imp;

use glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application, BuilderListItemFactory, BuilderScope, SingleSelection};
use gtk::{prelude::*, StringList};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_callbacks(&self) {
        self.imp()
            .search_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.query_videos();
            }));

        self.imp()
            .search_entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.query_videos();
            }));

        self.imp()
            .download_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.download_selected();
            }));
    }

    fn query_videos(&self) {
        let buffer = self.imp().search_entry.buffer();
        let query = buffer.text().to_string();
        if query.is_empty() {
            return;
        }

        unimplemented!("Querying {}", query);
    }

    fn download_selected(&self) {
        let selection_model = self
            .imp()
            .results_list
            .model()
            .expect("Could not get selection model");

        let selected_bitset = selection_model.selection();

        if selected_bitset.is_empty() {
            return;
        }

        let selected_index = selected_bitset.nth(0);

        let selected = self
            .results()
            .string(selected_index)
            .expect("Selected item should exist");

        unimplemented!("Downloading {}", selected);
    }

    fn results(&self) -> StringList {
        self.imp()
            .results
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn setup_results(&self) {
        let model = StringList::new(&[]);

        self.imp().results.replace(Some(model));

        let selection_model = SingleSelection::new(Some(self.results()));
        self.imp().results_list.set_model(Some(&selection_model));
    }

    fn setup_factory(&self) {
        let factory = BuilderListItemFactory::from_resource(
            None::<&BuilderScope>,
            "/juliangcalderon/youtube-downloader/ui/result.ui",
        );

        self.imp().results_list.set_factory(Some(&factory));
    }
}
