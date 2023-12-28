mod imp;

use gtk::gio::ListStore;
use gtk::glib::spawn_future_local;

use crate::client::Client;
use crate::video_object::VideoObject;
use crate::{config, RUNTIME};
use glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{
    gio, glib, Application, BuilderListItemFactory, BuilderScope, MessageDialog, SingleSelection,
};
use gtk::{prelude::*, Builder};

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

    async fn query_videos(&self) {
        let buffer = self.imp().search_entry.buffer().text().to_string();
        if buffer.is_empty() {
            return;
        }

        let client = self.client();
        let (sender, receiver) = async_channel::bounded(1);

        RUNTIME.spawn(async move {
            let res = client.query(buffer).await;
            if let Err(err) = sender.send(res).await {
                eprintln!("Could not send to channel, with error: {:?}", err);
            }
        });

        match receiver.recv().await {
            Ok(Ok(videos)) => {
                self.set_results(videos.into());
            }
            Ok(Err(err)) => {
                eprintln!("Could not query videos, with error: {:?}", err);
            }
            Err(err) => {
                eprintln!("Could not receive from channel, with error: {:?}", err);
            }
        }
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

    async fn download_selected(&self) {
        let selected = match self.get_selected() {
            Some(selected) => selected,
            None => return,
        };

        let id = selected.property("id");

        let client = self.client();
        let (sender, receiver) = async_channel::bounded(1);

        RUNTIME.spawn(async move {
            let res = client.download(id).await;
            if let Err(err) = sender.send(res).await {
                eprintln!("Could not send to channel, with error: {:?}", err);
            }
        });

        match receiver.recv().await {
            Ok(Ok(())) => {}
            Ok(Err(err)) => {
                eprintln!("Could not download video, with error: {:?}", err);
            }
            Err(err) => {
                eprintln!("Could not receive from channel, with error: {:?}", err);
            }
        }
    }

    fn setup_callbacks(&self) {
        self.imp()
            .search_button
            .connect_clicked(clone!(@weak self as self_ => move |_| {
                spawn_future_local(async move {
                    self_.imp().search_button.set_sensitive(false);
                    self_.query_videos().await;
                    self_.imp().search_button.set_sensitive(true);
                });
            }));

        self.imp()
            .search_entry
            .connect_activate(clone!(@weak self as self_ => move |_| {
                spawn_future_local(async move {
                    self_.imp().search_button.set_sensitive(false);
                    self_.query_videos().await;
                    self_.imp().search_button.set_sensitive(true);
                });
            }));

        self.imp()
            .download_button
            .connect_clicked(clone!(@weak self as self_ => move |_| {
                spawn_future_local(async move {
                    let download_dialog = Builder::from_resource(&format!("{}ui/download-dialog.ui", config::APP_IDPATH))
                        .objects().remove(0)
                        .downcast::<MessageDialog>()
                        .expect("Should be a MessageDialog");

                    download_dialog.set_transient_for(Some(&self_));
                    download_dialog.show();

                    self_.download_selected().await;

                    let finished_dialog = Builder::from_resource(&format!("{}ui/finished-dialog.ui", config::APP_IDPATH))
                        .objects().remove(0)
                        .downcast::<MessageDialog>()
                        .expect("Should be a MessageDialog");

                    finished_dialog.set_transient_for(Some(&self_));
                    finished_dialog.connect_response(|self_, _| {
                        self_.destroy();
                    });

                    download_dialog.destroy();
                    finished_dialog.show();
                });
            }));
    }

    fn setup_list(&self) {
        let model = ListStore::new::<VideoObject>();
        self.imp().results.replace(Some(model));

        let selection_model = SingleSelection::new(Some(self.results()));
        self.imp().results_list.set_model(Some(&selection_model));

        let factory = BuilderListItemFactory::from_resource(
            None::<&BuilderScope>,
            &format!("{}ui/video-item.ui", config::APP_IDPATH),
        );

        self.imp().results_list.set_factory(Some(&factory));
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
}
