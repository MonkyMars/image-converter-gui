use fltk::{dialog::*, window::Window};
use std::path::PathBuf;

pub fn open_single_file_dialog(_parent: &Window) -> Option<PathBuf> {
    let mut dialog = FileDialog::new(FileDialogType::BrowseFile);
    dialog.set_title("Select an Image File");

    // Set file filter for images
    dialog.set_filter("Image Files\t*.{jpg,jpeg,png,gif,bmp,webp,avif,tiff,tif}");

    dialog.show();

    let filename = dialog.filename();
    if !filename.to_string_lossy().is_empty() {
        return Some(filename);
    }

    None
}

pub fn open_multiple_files_dialog(_parent: &Window) -> Option<Vec<PathBuf>> {
    let mut dialog = FileDialog::new(FileDialogType::BrowseMultiFile);
    dialog.set_title("Select Multiple Image Files");

    // Set file filter for images
    dialog.set_filter("Image Files\t*.{jpg,jpeg,png,gif,bmp,webp,avif,tiff,tif}");

    dialog.show();

    let mut files = Vec::new();

    // For multi-file dialog, we need to handle the filenames differently
    let filename = dialog.filename();
    if !filename.to_string_lossy().is_empty() {
        files.push(filename);

        // Try to get additional files if they exist
        // Note: FLTK's multi-file dialog handling varies by platform
        // This is a simplified implementation
        return Some(files);
    }

    None
}

pub fn show_error_dialog(_parent: &Window, message: &str) {
    alert_default(message);
}

pub fn show_info_dialog(_parent: &Window, message: &str) {
    message_default(message);
}
