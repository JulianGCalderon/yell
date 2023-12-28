mod application;
mod application_window;
mod config;
mod video;

use application::Application;
use dotenv::dotenv;
use gtk::prelude::*;
use gtk::{gio, glib};

fn main() -> glib::ExitCode {
    load_env();
    load_resources();

    Application::new().run()
}

fn load_resources() {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");
}

fn load_env() {
    dotenv().expect("Failed to load .env file");
}
