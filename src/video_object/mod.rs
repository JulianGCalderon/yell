mod deserialize;

pub use deserialize::VideoResponse;

use glib::Object;
use gtk::glib;

mod imp {
    use std::cell::RefCell;

    use glib::Properties;
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::VideoObject)]
    pub struct VideoObject {
        #[property(get, set)]
        pub id: RefCell<String>,
        #[property(get, set)]
        pub title: RefCell<String>,
        #[property(get, set)]
        pub description: RefCell<String>,
        #[property(get, set)]
        pub thumbnail: RefCell<String>,
        #[property(get, set)]
        pub channel_title: RefCell<String>,
        #[property(get, set)]
        pub published_at: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VideoObject {
        const NAME: &'static str = "YellVideoObject";
        type Type = super::VideoObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for VideoObject {}
}

glib::wrapper! {
    pub struct VideoObject(ObjectSubclass<imp::VideoObject>);
}

impl Default for VideoObject {
    fn default() -> Self {
        Object::builder().build()
    }
}
