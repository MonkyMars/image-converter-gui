use gtk4::prelude::*;
use gtk4::{
    ButtonsType, FileChooserAction, FileChooserDialog, MessageDialog, MessageType, ResponseType,
    Window,
};
use std::path::PathBuf;

pub fn open_single_file_dialog(
    parent: &impl IsA<Window>,
    callback: impl Fn(Option<PathBuf>) + 'static,
) {
    let dialog = FileChooserDialog::new(
        Some("Select a File"),
        Some(parent),
        FileChooserAction::Open,
        &[
            ("Cancel", ResponseType::Cancel),
            ("Open", ResponseType::Accept),
        ],
    );

    // Add file filters - only allow images
    let filter_images = gtk4::FileFilter::new();
    filter_images.set_name(Some("Image Files"));
    filter_images.add_pixbuf_formats();
    filter_images.add_pattern("*.jpg");
    filter_images.add_pattern("*.jpeg");
    filter_images.add_pattern("*.png");
    filter_images.add_pattern("*.gif");
    filter_images.add_pattern("*.bmp");
    filter_images.add_pattern("*.webp");
    filter_images.add_pattern("*.avif");
    filter_images.add_pattern("*.tiff");
    filter_images.add_pattern("*.tif");
    dialog.add_filter(&filter_images);
    dialog.set_filter(&filter_images);

    dialog.set_modal(true);

    dialog.connect_response(move |dialog, response| {
        let result = if response == ResponseType::Accept {
            dialog.file().and_then(|file| file.path())
        } else {
            None
        };
        callback(result);
        dialog.close();
    });

    dialog.present();
}

pub fn open_multiple_files_dialog(
    parent: &impl IsA<Window>,
    callback: impl Fn(Vec<PathBuf>) + 'static,
) {
    let dialog = FileChooserDialog::new(
        Some("Select Multiple Files"),
        Some(parent),
        FileChooserAction::Open,
        &[
            ("Cancel", ResponseType::Cancel),
            ("Open", ResponseType::Accept),
        ],
    );

    dialog.set_select_multiple(true);
    dialog.set_modal(true);

    // Add file filters - only allow images
    let filter_images = gtk4::FileFilter::new();
    filter_images.set_name(Some("Image Files"));
    filter_images.add_pixbuf_formats();
    filter_images.add_pattern("*.jpg");
    filter_images.add_pattern("*.jpeg");
    filter_images.add_pattern("*.png");
    filter_images.add_pattern("*.gif");
    filter_images.add_pattern("*.bmp");
    filter_images.add_pattern("*.webp");
    filter_images.add_pattern("*.avif");
    filter_images.add_pattern("*.tiff");
    filter_images.add_pattern("*.tif");
    dialog.add_filter(&filter_images);
    dialog.set_filter(&filter_images);

    dialog.connect_response(move |dialog, response| {
        let result = if response == ResponseType::Accept {
            let files = dialog.files();
            let mut paths = Vec::new();

            for i in 0..files.n_items() {
                if let Some(file) = files.item(i) {
                    if let Some(gio_file) = file.downcast_ref::<gtk4::gio::File>() {
                        if let Some(path) = gio_file.path() {
                            paths.push(path);
                        }
                    }
                }
            }
            paths
        } else {
            Vec::new()
        };
        callback(result);
        dialog.close();
    });

    dialog.present();
}

pub fn show_error_dialog(parent: &impl IsA<Window>, message: &str) {
    let dialog = MessageDialog::builder()
        .transient_for(parent)
        .modal(true)
        .message_type(MessageType::Error)
        .buttons(ButtonsType::Ok)
        .text(message)
        .build();

    dialog.connect_response(move |dialog, _| {
        dialog.close();
    });

    dialog.present();
}

pub fn show_info_dialog(parent: &impl IsA<Window>, message: &str) {
    let dialog = MessageDialog::builder()
        .transient_for(parent)
        .modal(true)
        .message_type(MessageType::Info)
        .buttons(ButtonsType::Ok)
        .text(message)
        .build();

    dialog.connect_response(move |dialog, _| {
        dialog.close();
    });

    dialog.present();
}
