use gtk::{Application, glib};
use gtk::{ApplicationWindow, prelude::*};

const APP_ID: &str = "com.example.easy-serve";
const TITLE: &str = "Easy Server";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .default_width(900)
        .default_height(700)
        .build();
    window.present();
}
