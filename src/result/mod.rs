mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct ResultObject(ObjectSubclass<imp::TaskObject>);
}

impl ResultObject {
    pub fn new(data: ResultData) -> Self {
        let obj: Self = Object::builder().build();

        *obj.imp().data.borrow_mut() = data;

        obj
    }
}

#[derive(Default)]
pub struct ResultData {
    pub completed: bool,
    pub content: String,
}
