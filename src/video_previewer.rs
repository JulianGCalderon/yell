use adw::prelude::*;
use gtk::gio::File;
use gtk::glib;
use gtk::glib::{closure, subclass::types::ObjectSubclassIsExt, Object};
use gtk::Widget;

use crate::video_object::VideoObject;

mod imp {
    use std::cell::RefCell;

    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::glib::Properties;
    use gtk::{glib, Label, Picture, TextView};

    use crate::video_object::VideoObject;

    #[derive(Properties, gtk::CompositeTemplate, Default)]
    #[properties(wrapper_type = super::VideoPreviewer)]
    #[template(resource = "/com/github/juliangcalderon/yell/ui/video-previewer.ui")]
    pub struct VideoPreviewer {
        #[template_child]
        pub thumbnail: TemplateChild<Picture>,
        #[template_child]
        pub title: TemplateChild<Label>,
        #[template_child]
        pub description: TemplateChild<TextView>,
        #[template_child]
        pub channel_title: TemplateChild<Label>,
        #[template_child]
        pub published_at: TemplateChild<Label>,

        #[property(name="video", get, set, type = VideoObject)]
        pub video: RefCell<VideoObject>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VideoPreviewer {
        const NAME: &'static str = "YellVideoPreviewer";
        type Type = super::VideoPreviewer;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for VideoPreviewer {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup();
        }
    }

    impl WidgetImpl for VideoPreviewer {}
    impl BoxImpl for VideoPreviewer {}
}

glib::wrapper! {
    pub struct VideoPreviewer(ObjectSubclass<imp::VideoPreviewer>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable, gtk::Orientable;
}

impl VideoPreviewer {
    fn setup(&self) {
        let video_expresion = self.property_expression("video");

        let title = self.imp().title.clone();
        video_expresion
            .chain_property::<VideoObject>("title")
            .bind(&title, "label", Widget::NONE);

        let buffer = self.imp().description.buffer();
        video_expresion
            .chain_property::<VideoObject>("description")
            .bind(&buffer, "text", Widget::NONE);

        let picture = self.imp().thumbnail.clone();
        video_expresion
            .chain_property::<VideoObject>("thumbnail")
            .chain_closure::<File>(closure!(|_: Option<Object>, url: &str| {
                File::for_uri(&url)
            }))
            .bind(&picture, "file", Widget::NONE);

        let channel_title = self.imp().channel_title.clone();
        video_expresion
            .chain_property::<VideoObject>("channel-title")
            .bind(&channel_title, "label", Widget::NONE);

        let published_at = self.imp().published_at.clone();
        video_expresion
            .chain_property::<VideoObject>("published-at")
            .bind(&published_at, "label", Widget::NONE);
    }
}
