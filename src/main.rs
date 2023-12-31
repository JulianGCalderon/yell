mod application;
mod application_window;
mod client;
mod config;
mod video_object;
mod video_preview;

use std::error::Error;

use adw::prelude::*;
use application::Application;
use dotenv::dotenv;
use gtk::glib::once_cell::sync::Lazy;
use gtk::{gio, glib};
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

// type BoxResult<T> = Result<T, Box<dyn Error>>;
type BoxSendResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

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
