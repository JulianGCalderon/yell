use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio::ListStore, glib};

use crate::{client::Client, video_object::VideoObject};

mod imp {
    use std::cell::RefCell;

    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::{gio::ListStore, glib, Button, Entry, ListBox};

    use crate::client::Client;

    #[derive(gtk::CompositeTemplate, Default)]
    #[template(resource = "/com/github/juliangcalderon/yell/ui/video-searcher.ui")]
    pub struct VideoSearcher {
        #[template_child]
        pub search_entry: TemplateChild<Entry>,
        #[template_child]
        pub search_button: TemplateChild<Button>,
        #[template_child]
        pub download_button: TemplateChild<Button>,
        #[template_child]
        pub results_list: TemplateChild<ListBox>,

        pub client: RefCell<Client>,
        pub results: RefCell<Option<ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VideoSearcher {
        const NAME: &'static str = "YellVideoSearcher";
        type Type = super::VideoSearcher;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VideoSearcher {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup();
        }
    }

    impl WidgetImpl for VideoSearcher {}
    impl BoxImpl for VideoSearcher {}
}

glib::wrapper! {
    pub struct VideoSearcher(ObjectSubclass<imp::VideoSearcher>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable, gtk::Orientable;
}

impl VideoSearcher {
    fn setup(&self) {
        let model = ListStore::new::<VideoObject>();
        self.imp().results.replace(Some(model));

        self.imp()
            .results_list
            .bind_model(Some(&self.results()), |video| {
                let video: &VideoObject = video.downcast_ref().expect("Should be a video object");
                let title = video.title();
                let label = gtk::Label::new(Some(&title));

                label.upcast()
            });
    }

    fn _set_results(&self, videos: Vec<VideoObject>) {
        self.results().remove_all();
        self.results().extend_from_slice(&videos);
    }

    fn results(&self) -> ListStore {
        self.imp()
            .results
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn _client(&self) -> Client {
        self.imp().client.borrow().clone()
    }
}
