mod imp;

use std::path::PathBuf;

use gtk::gio::ListStore;
use gtk::glib::spawn_future_local;

use crate::client::Client;
use crate::video_object::VideoObject;
use crate::{config, RUNTIME};
use glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{
    gio, glib, Application, BuilderListItemFactory, BuilderScope, FileChooserAction,
    FileChooserDialog, MessageDialog, ResponseType, SingleSelection,
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

#[gtk::template_callbacks]
impl ApplicationWindow {
    #[template_callback]
    fn handle_search(&self) {
        let query = self.imp().search_entry.buffer().text().to_string();
        if query.is_empty() {
            return;
        }

        spawn_future_local(clone!(@weak self as this => async move {
            this.start_search(query).await;
        }));
    }

    async fn start_search(&self, query: String) {
        self.imp().search_button.set_sensitive(false);
        self.query_videos(query).await;
        self.imp().search_button.set_sensitive(true);
    }

    async fn query_videos(&self, query: String) {
        let client = self.client();
        let (sender, receiver) = async_channel::bounded(1);

        RUNTIME.spawn(async move {
            let res = client.query(query).await;
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

    #[template_callback]
    fn handle_download(&self) {
        let selected = match self.get_selected() {
            Some(selected) => selected,
            None => return,
        };

        let selected_id = selected.property::<String>("id");

        let file_chooser_dialog =
            Builder::from_resource(&format!("{}ui/file-chooser-dialog.ui", config::APP_IDPATH))
                .objects()
                .remove(0)
                .downcast::<FileChooserDialog>()
                .expect("Should be a FileChooserDialog");

        file_chooser_dialog.set_action(FileChooserAction::Save);
        file_chooser_dialog.set_current_name(&format!("{}.mp4", selected_id));
        file_chooser_dialog.set_transient_for(Some(self));

        file_chooser_dialog.connect_response(clone!(@weak self as this => move |chooser, event| {
            match event {
                ResponseType::Cancel => chooser.destroy(),
                ResponseType::Accept => {
                    let path = match chooser.file().and_then(|file| file.path()) {
                        Some(path) => path,
                        None => return,
                    };

                    let selected_id = selected_id.clone();

                    spawn_future_local(clone!( @weak this => async move {
                        this.start_download(selected_id, path).await;
                    }));

                    chooser.destroy();
                }
                _ => {}
            }
        }));

        file_chooser_dialog.present();
    }

    async fn start_download(&self, id: String, path: PathBuf) {
        let download_dialog =
            Builder::from_resource(&format!("{}ui/download-dialog.ui", config::APP_IDPATH))
                .objects()
                .remove(0)
                .downcast::<MessageDialog>()
                .expect("Should be a MessageDialog");

        download_dialog.set_transient_for(Some(self));
        download_dialog.show();

        let client = self.client();
        let (sender, receiver) = async_channel::bounded(1);

        RUNTIME.spawn(async move {
            let res = client.download(id, path).await;
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

        let finished_dialog =
            Builder::from_resource(&format!("{}ui/finished-dialog.ui", config::APP_IDPATH))
                .objects()
                .remove(0)
                .downcast::<MessageDialog>()
                .expect("Should be a MessageDialog");

        finished_dialog.set_transient_for(Some(self));
        finished_dialog.connect_response(|self_, _| {
            self_.destroy();
        });

        download_dialog.destroy();
        finished_dialog.show();
    }
}
