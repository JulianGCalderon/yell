use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::gio::ListStore;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Entry};
use gtk::{Button, ListView};

use crate::client::Client;

// #[template(resource = "/juliangcalderon/yell/ui/window.ui/")]
// Object holding the state
#[derive(CompositeTemplate, Default)]
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

    pub results: RefCell<Option<ListStore>>,
    pub client: RefCell<Client>,
}

#[glib::object_subclass]
impl ObjectSubclass for ApplicationWindow {
    const NAME: &'static str = "YellApplicationWindow";
    type Type = super::ApplicationWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ApplicationWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.setup_callbacks();
        obj.setup_list();
    }
}

impl WidgetImpl for ApplicationWindow {}
impl WindowImpl for ApplicationWindow {}
impl ApplicationWindowImpl for ApplicationWindow {}
