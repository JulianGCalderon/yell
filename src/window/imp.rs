use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/juliangcalderon/youtube-downloader/ui/window.ui/")]
pub struct Window {
    #[template_child]
    pub button: TemplateChild<Button>,
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

        self.button.connect_clicked(move |button| {
            button.set_label("Hello World!");
        });
    }
}

impl WidgetImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl WindowImpl for Window {}
