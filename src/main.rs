mod video;
mod window;

use dotenv::dotenv;
use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

const APP_ID: &str = "juliangcalderon.yell";

fn main() -> glib::ExitCode {
    dotenv().ok();

    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
