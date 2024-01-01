use std::fs;
use std::path::PathBuf;

use adw::prelude::*;
use gtk::glib;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::glib::{clone, spawn_future_local};
use gtk::ResponseType;
use rustube::{Callback, CallbackArguments};
use tokio::sync::mpsc::channel;

use gtk::Builder;
use gtk::{Dialog, FileChooserAction, FileChooserDialog, MessageDialog, ProgressBar};

use crate::{config, RUNTIME};

use super::ApplicationWindow;

const PROGRESS_CHANNEL_CAPACITY: usize = 100;

#[gtk::template_callbacks]
impl ApplicationWindow {
    #[template_callback]
    fn handle_search(&self) {
        let query = self.imp().search_entry.buffer().text().to_string();
        if query.is_empty() {
            return;
        }

        spawn_future_local(clone!(@weak self as this => async move {
            this.imp().search_button.set_sensitive(false);
            this.query_videos(query).await;
            this.imp().search_button.set_sensitive(true);
        }));
    }

    async fn query_videos(&self, query: String) {
        let client = self.client();
        let (sender, receiver) = async_channel::bounded(1);

        RUNTIME.spawn(async move {
            let res = client.query(query).await;
            sender
                .send(res)
                .await
                .expect("Receiver should never be closed");
        });

        match receiver.recv().await {
            Ok(Ok(videos)) => {
                self.set_results(videos.into());
            }
            Ok(Err(_)) => {
                let error_dialog = Builder::from_resource(&format!(
                    "{}ui/search-error-dialog.ui",
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
            _ => panic!("Sender should never be closed"),
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
        let download_builder =
            Builder::from_resource(&format!("{}ui/downloading-dialog.ui", config::APP_IDPATH));

        let download_dialog = download_builder
            .objects()
            .remove(0)
            .downcast::<Dialog>()
            .expect("Should be a MessageDialog");

        download_dialog.set_transient_for(Some(self));
        download_dialog.show();

        let progress_bar: ProgressBar = download_builder
            .object("progress_bar")
            .expect("Should exist a progress bar");

        let (progress_sender, mut progress_receiver) = channel(PROGRESS_CHANNEL_CAPACITY);
        let (complete_sender, mut complete_receiver) = channel(1);
        let (abort_sender, mut abort_receiver) = channel(1);

        download_dialog.connect_response(move |self_, _| {
            abort_sender.blocking_send(()).ok();
            self_.destroy();
        });

        let callback = Callback::new();
        let callback = callback.connect_on_progress_sender(progress_sender, true);

        let client = self.client();
        let path_clone = path.clone();
        RUNTIME.spawn(async move {
            let res = client.download(id, path_clone, callback).await;
            complete_sender.send(res).await.ok()
        });

        loop {
            tokio::select! {
                Some(progress) = progress_receiver.recv() => {
                    let CallbackArguments { current_chunk, content_length } = progress;
                    let Some(content_length) = content_length else { continue };
                    let fraction = current_chunk as f64 / content_length as f64;

                    progress_bar.set_fraction(fraction);
                }
                Some(complete) = complete_receiver.recv() => {
                    download_dialog.destroy();
                    match complete {
                        Ok(_) => self.open_download_finish_dialog(),
                        Err(err) => {
                            eprintln!("Could not finish download, with error: {:?}", err);
                            self.open_download_error_dialog();
                        },
                    }
                    return;
                }
                Some(()) = abort_receiver.recv() => {
                    if let Err(err) = fs::remove_file(path) {
                        eprintln!("Could delete leftover file, with error: {:?}", err);
                    }
                    return;
                }
            };
        }
    }

    pub(super) fn handle_select(&self) {
        let Some(selected) = self.get_selected() else {
            return;
        };

        self.imp().leaflet.set_can_unfold(true);
        self.imp().preview.set_video(selected);
    }
}
