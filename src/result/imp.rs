use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::ResultData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::ResultObject)]
pub struct TaskObject {
    #[property(name = "completed", get, set, type = bool, member = completed)]
    #[property(name = "content", get, set, type = String, member = content)]
    pub data: RefCell<ResultData>,
}

#[glib::object_subclass]
impl ObjectSubclass for TaskObject {
    const NAME: &'static str = "TodoTaskObject";
    type Type = super::ResultObject;
}

#[glib::derived_properties]
impl ObjectImpl for TaskObject {}
