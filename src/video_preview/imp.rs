use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::glib::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Picture};

use crate::video_object::VideoObject;

#[derive(Properties, gtk::CompositeTemplate, Default)]
#[properties(wrapper_type = super::VideoPreview)]
#[template(resource = "/com/github/juliangcalderon/yell/ui/video-preview.ui")]
pub struct VideoPreview {
    #[template_child]
    pub thumbnail: TemplateChild<Picture>,

    #[property(name="video", get, set, type = VideoObject)]
    pub video: RefCell<VideoObject>,
}

#[glib::object_subclass]
impl ObjectSubclass for VideoPreview {
    const NAME: &'static str = "YellVideoPreview";
    type Type = super::VideoPreview;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for VideoPreview {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup();
    }
}

impl WidgetImpl for VideoPreview {}
impl BoxImpl for VideoPreview {}
