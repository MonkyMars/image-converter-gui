pub mod convert;
mod window;

use gtk4::glib::ExitCode;
use gtk4::prelude::*;
use window::get_app;

fn main() -> ExitCode {
    // Initialize GTK
    match gtk4::init() {
        Ok(_) => println!("GTK4 initialized successfully"),
        Err(e) => {
            eprintln!("Failed to initialize GTK4: {}", e);
            return ExitCode::FAILURE;
        }
    }

    // Run the app
    println!("Creating application...");
    let app = get_app();
    app.run()
}
