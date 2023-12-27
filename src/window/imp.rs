use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Entry};
use gtk::{Button, ListView, StringList};
use reqwest::blocking::Client;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/juliangcalderon/youtube-downloader/ui/window.ui/")]
pub struct Window {
    #[template_child]
    pub search_entry: TemplateChild<Entry>,
    #[template_child]
    pub search_button: TemplateChild<Button>,
    #[template_child]
    pub download_button: TemplateChild<Button>,
    #[template_child]
    pub results_list: TemplateChild<ListView>,
    pub results: RefCell<Option<StringList>>,
    pub client: RefCell<Option<Client>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_callbacks();
        obj.setup_results();
        obj.setup_client();
        obj.setup_factory();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
