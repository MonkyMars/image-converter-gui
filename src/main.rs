pub mod convert;
mod window;

use fltk::{app, prelude::*};
use window::create_app;

fn main() {
    println!("Initializing FLTK application...");

    // Initialize FLTK
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    // Set a dark theme
    app::set_background_color(26, 26, 26);
    app::set_background2_color(45, 45, 45);
    app::set_foreground_color(255, 255, 255);

    println!("Creating application window...");

    // Create and show the main window
    let mut wind = create_app();
    wind.show();

    println!("Running application...");

    // Run the application
    app.run().unwrap();
}
