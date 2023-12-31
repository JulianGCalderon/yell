mod callbacks;
mod imp;

use gtk::glib::clone;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::{gio::ListStore, MessageDialog};
use gtk::{prelude::*, Builder};

use crate::client::Client;
use crate::config;
use crate::video_object::VideoObject;
use glib::Object;
use gtk::{gio, glib, Application, BuilderListItemFactory, BuilderScope, SingleSelection};

glib::wrapper! {
    pub struct ApplicationWindow(ObjectSubclass<imp::ApplicationWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ApplicationWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn setup(&self) {
        self.setup_list();
    }

    fn setup_list(&self) {
        let model = ListStore::new::<VideoObject>();
        self.imp().results.replace(Some(model));

        let selection_model = SingleSelection::new(Some(self.results()));
        selection_model.connect_selected_item_notify(clone!(@weak self as this => move |_| {
            this.handle_select();
        }));

        self.imp().results_list.set_model(Some(&selection_model));

        let factory = BuilderListItemFactory::from_resource(
            None::<&BuilderScope>,
            &format!("{}ui/video-item.ui", config::APP_IDPATH),
        );

        self.imp().results_list.set_factory(Some(&factory));
    }

    fn set_results(&self, videos: Vec<VideoObject>) {
        self.results().remove_all();
        self.results().extend_from_slice(&videos);
    }

    fn get_selected(&self) -> Option<VideoObject> {
        let selection_model = self
            .imp()
            .results_list
            .model()
            .expect("Could not get selection model");

        let selected_index = selection_model.selection().nth(0);

        self.results()
            .item(selected_index)?
            .downcast::<VideoObject>()
            .ok()
    }

    fn results(&self) -> ListStore {
        self.imp()
            .results
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn client(&self) -> Client {
        self.imp().client.borrow().clone()
    }

    fn open_download_finish_dialog(&self) {
        let finished_dialog = Builder::from_resource(&format!(
            "{}ui/download-finish-dialog.ui",
            config::APP_IDPATH
        ))
        .objects()
        .remove(0)
        .downcast::<MessageDialog>()
        .expect("Should be a MessageDialog");

        finished_dialog.set_transient_for(Some(self));
        finished_dialog.connect_response(|self_, _| {
            self_.destroy();
        });

        finished_dialog.show();
    }

    fn open_download_error_dialog(&self) {
        let error_dialog = Builder::from_resource(&format!(
            "{}ui/download-error-dialog.ui",
            config::APP_IDPATH
        ))
        .objects()
        .remove(0)
        .downcast::<MessageDialog>()
        .expect("Should be a MessageDialog");

        error_dialog.set_transient_for(Some(self));
        error_dialog.connect_response(|self_, _| {
            self_.destroy();
        });

        error_dialog.show();
    }
}
