use std::cell::RefCell;

use adw::subclass::prelude::*;
use adw::Leaflet;
use glib::subclass::InitializingObject;
use gtk::gio::ListStore;
use gtk::glib;
use gtk::{Button, Entry, ListView};

use crate::client::Client;
use crate::video_preview::VideoPreview;

// #[template(resource = "/juliangcalderon/yell/ui/window.ui/")]
// Object holding the state
#[derive(gtk::CompositeTemplate, Default)]
#[template(resource = "/com/github/juliangcalderon/yell/ui/application-window.ui")]
pub struct ApplicationWindow {
    #[template_child]
    pub search_entry: TemplateChild<Entry>,
    #[template_child]
    pub search_button: TemplateChild<Button>,
    #[template_child]
    pub download_button: TemplateChild<Button>,
    #[template_child]
    pub results_list: TemplateChild<ListView>,
    #[template_child]
    pub preview: TemplateChild<VideoPreview>,
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,

    pub results: RefCell<Option<ListStore>>,
    pub client: RefCell<Client>,
}

#[glib::object_subclass]
impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "YellApplicationWindow";
    type Type = super::ApplicationWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ApplicationWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.setup();
    }
}

impl WidgetImpl for ApplicationWindow {}
impl WindowImpl for ApplicationWindow {}
impl ApplicationWindowImpl for ApplicationWindow {}
impl AdwApplicationWindowImpl for ApplicationWindow {}
