use fltk::{
    app, browser::*, button::*, enums::*, frame::*, group::*, menu::*, prelude::*, window::*,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::convert::{convert_batch_parallel, convert_image, ConvertFormat};
use crate::window::dialog;

pub fn create_app() -> Window {
    let mut wind = Window::new(100, 100, 800, 700, "Image Converter");
    wind.set_color(Color::from_rgb(26, 26, 26));
    wind.set_border(false);

    // State management
    let single_file: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let batch_files: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));

    // Create main vertical pack
    let mut main_pack = Pack::new(20, 20, 760, 660, "");
    main_pack.set_spacing(20);
    main_pack.set_type(PackType::Vertical);
    main_pack.set_color(Color::from_rgb(26, 26, 26));
    main_pack.set_frame(FrameType::NoBox);

    // Title
    let mut title = Frame::new(0, 0, 760, 50, "Image Converter");
    title.set_label_size(26);
    title.set_label_font(Font::HelveticaBold);
    title.set_label_color(Color::White);
    title.set_align(Align::Center);
    title.set_frame(FrameType::FlatBox);
    title.set_color(Color::from_rgb(26, 26, 26));

    // Single file conversion section
    create_single_upload_section(&mut main_pack, &single_file, &wind);

    // Batch file conversion section
    create_batch_upload_section(&mut main_pack, &batch_files, &wind);

    main_pack.end();
    wind.end();
    wind.resizable(&main_pack);
    wind
}

fn create_single_upload_section(
    parent: &mut Pack,
    single_file: &Rc<RefCell<Option<PathBuf>>>,
    parent_window: &Window,
) {
    let mut section = Group::new(0, 0, 760, 200, "");
    section.set_frame(FrameType::RFlatBox);
    section.set_color(Color::from_rgb(35, 40, 47));

    // Simple styling without problematic shadows

    let mut title_frame = Frame::new(15, 15, 730, 25, "Single File Conversion");
    title_frame.set_label_font(Font::HelveticaBold);
    title_frame.set_label_color(Color::White);
    title_frame.set_align(Align::Left | Align::Inside);
    title_frame.set_frame(FrameType::NoBox);

    // File selection row
    let mut select_btn = Button::new(20, 45, 120, 30, "Select Image");
    style_primary_button(&mut select_btn);

    let mut file_label = Frame::new(150, 45, 450, 30, "No file selected");
    file_label.set_label_color(Color::from_rgb(139, 148, 158));
    file_label.set_align(Align::Left | Align::Inside);

    let mut clear_btn = Button::new(620, 45, 100, 30, "Clear");
    style_destructive_button(&mut clear_btn);

    // Options row
    let mut format_label = Frame::new(20, 90, 120, 30, "Output Format:");
    format_label.set_label_color(Color::White);
    format_label.set_align(Align::Left | Align::Inside);

    let mut format_choice = Choice::new(150, 90, 100, 30, "");
    format_choice.add_choice("JPEG");
    format_choice.add_choice("PNG");
    format_choice.add_choice("WebP");
    format_choice.add_choice("BMP");
    format_choice.add_choice("GIF");
    format_choice.set_value(0);
    style_choice_widget(&mut format_choice);

    let mut overwrite_check = CheckButton::new(270, 90, 200, 30, "Overwrite existing files");
    style_checkbox(&mut overwrite_check);

    let mut convert_btn = Button::new(620, 90, 100, 30, "Convert");
    style_primary_button(&mut convert_btn);
    convert_btn.deactivate();

    // Progress info
    let mut progress_label = Frame::new(20, 130, 700, 25, "");
    progress_label.set_label_color(Color::from_rgb(139, 148, 158));
    progress_label.set_align(Align::Left | Align::Inside);

    // Setup callbacks
    {
        let single_file_clone = single_file.clone();
        let parent_clone = parent_window.clone();
        let mut file_label_clone = file_label.clone();
        let mut convert_btn_clone = convert_btn.clone();

        select_btn.set_callback(move |_| {
            if let Some(path) = dialog::open_single_file_dialog(&parent_clone) {
                *single_file_clone.borrow_mut() = Some(path.clone());
                let filename = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Unknown file");
                file_label_clone.set_label(&format!("Selected: {}", filename));
                convert_btn_clone.activate();
                app::redraw();
            }
        });
    }

    {
        let single_file_clone = single_file.clone();
        let mut file_label_clone = file_label.clone();
        let mut convert_btn_clone = convert_btn.clone();

        clear_btn.set_callback(move |_| {
            *single_file_clone.borrow_mut() = None;
            file_label_clone.set_label("No file selected");
            convert_btn_clone.deactivate();
            app::redraw();
        });
    }

    {
        let single_file_clone = single_file.clone();
        let format_choice_clone = format_choice.clone();
        let overwrite_check_clone = overwrite_check.clone();
        let mut progress_label_clone = progress_label.clone();
        let parent_clone = parent_window.clone();

        convert_btn.set_callback(move |_| {
            let single_file = single_file_clone.borrow();
            if let Some(input_path) = single_file.as_ref() {
                let selected_index = format_choice_clone.value();
                let format = match selected_index {
                    0 => ConvertFormat::Jpeg,
                    1 => ConvertFormat::Png,
                    2 => ConvertFormat::Webp,
                    3 => ConvertFormat::Bmp,
                    4 => ConvertFormat::Gif,
                    _ => ConvertFormat::Jpeg,
                };

                let input_stem = input_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("converted");

                let overwrite = overwrite_check_clone.is_checked();
                let output_filename = if overwrite {
                    format!("{}.{}", input_stem, format.extension())
                } else {
                    let mut counter = 1;
                    let base_path = input_path.parent().unwrap_or(std::path::Path::new("."));
                    loop {
                        let filename = format!(
                            "{}_converted_{}.{}",
                            input_stem,
                            counter,
                            format.extension()
                        );
                        let test_path = base_path.join(&filename);
                        if !test_path.exists() {
                            break filename;
                        }
                        counter += 1;
                    }
                };

                let output_path = input_path
                    .parent()
                    .unwrap_or(std::path::Path::new("."))
                    .join(output_filename);

                progress_label_clone.set_label("Converting...");
                app::redraw();

                match convert_image(input_path.clone(), output_path.clone(), format) {
                    Ok(_) => {
                        progress_label_clone.set_label("Conversion completed successfully!");
                        let message = format!(
                            "Successfully converted image to:\n{}",
                            output_path.display()
                        );
                        dialog::show_info_dialog(&parent_clone, &message);
                    }
                    Err(e) => {
                        progress_label_clone.set_label("Conversion failed!");
                        let message = format!("Conversion failed: {}", e);
                        dialog::show_error_dialog(&parent_clone, &message);
                    }
                }
                app::redraw();
            }
        });
    }

    section.end();
    parent.add(&section);
}

fn create_batch_upload_section(
    parent: &mut Pack,
    batch_files: &Rc<RefCell<Vec<PathBuf>>>,
    parent_window: &Window,
) {
    let mut section = Group::new(0, 0, 760, 400, "");
    section.set_frame(FrameType::RFlatBox);
    section.set_color(Color::from_rgb(35, 40, 47));

    // Simple styling without problematic shadows

    let mut title_frame = Frame::new(15, 15, 730, 25, "Batch File Conversion");
    title_frame.set_label_font(Font::HelveticaBold);
    title_frame.set_label_color(Color::White);
    title_frame.set_align(Align::Left | Align::Inside);
    title_frame.set_frame(FrameType::NoBox);

    // File selection row
    let mut select_btn = Button::new(20, 45, 150, 30, "Select Multiple Images");
    style_primary_button(&mut select_btn);

    let mut clear_btn = Button::new(620, 45, 100, 30, "Clear All");
    style_destructive_button(&mut clear_btn);

    // File list
    let mut list_title = Frame::new(20, 85, 700, 20, "Selected Files:");
    list_title.set_label_color(Color::White);
    list_title.set_align(Align::Left | Align::Inside);

    let mut file_browser = Browser::new(20, 110, 700, 140, "");
    file_browser.set_color(Color::from_rgb(28, 33, 40));
    file_browser.set_selection_color(Color::from_rgb(9, 105, 218));
    file_browser.set_frame(FrameType::DownBox);

    // Options row
    let mut format_label = Frame::new(20, 265, 120, 30, "Output Format:");
    format_label.set_label_color(Color::White);
    format_label.set_align(Align::Left | Align::Inside);

    let mut format_choice = Choice::new(150, 265, 100, 30, "");
    format_choice.add_choice("JPEG");
    format_choice.add_choice("PNG");
    format_choice.add_choice("WebP");
    format_choice.add_choice("BMP");
    format_choice.add_choice("GIF");
    format_choice.set_value(0);
    style_choice_widget(&mut format_choice);

    let mut overwrite_check = CheckButton::new(270, 265, 200, 30, "Overwrite existing files");
    style_checkbox(&mut overwrite_check);

    let mut process_btn = Button::new(620, 265, 100, 30, "Convert All");
    style_primary_button(&mut process_btn);
    process_btn.deactivate();

    // Progress info
    let mut progress_label = Frame::new(20, 305, 700, 25, "");
    progress_label.set_label_color(Color::from_rgb(139, 148, 158));
    progress_label.set_align(Align::Left | Align::Inside);

    // Setup callbacks
    {
        let batch_files_clone = batch_files.clone();
        let parent_clone = parent_window.clone();
        let mut file_browser_clone = file_browser.clone();
        let mut process_btn_clone = process_btn.clone();

        select_btn.set_callback(move |_| {
            if let Some(paths) = dialog::open_multiple_files_dialog(&parent_clone) {
                if !paths.is_empty() {
                    batch_files_clone.borrow_mut().extend(paths);
                    update_file_list(&mut file_browser_clone, &batch_files_clone.borrow());
                    process_btn_clone.activate();
                    app::redraw();
                }
            }
        });
    }

    {
        let batch_files_clone = batch_files.clone();
        let mut file_browser_clone = file_browser.clone();
        let mut process_btn_clone = process_btn.clone();

        clear_btn.set_callback(move |_| {
            batch_files_clone.borrow_mut().clear();
            file_browser_clone.clear();
            process_btn_clone.deactivate();
            app::redraw();
        });
    }

    {
        let batch_files_clone = batch_files.clone();
        let format_choice_clone = format_choice.clone();
        let overwrite_check_clone = overwrite_check.clone();
        let mut progress_label_clone = progress_label.clone();
        let mut process_btn_clone = process_btn.clone();
        let parent_clone = parent_window.clone();

        process_btn.set_callback(move |_| {
            let files = batch_files_clone.borrow().clone();
            if !files.is_empty() {
                let selected_index = format_choice_clone.value();
                let format = match selected_index {
                    0 => ConvertFormat::Jpeg,
                    1 => ConvertFormat::Png,
                    2 => ConvertFormat::Webp,
                    3 => ConvertFormat::Bmp,
                    4 => ConvertFormat::Gif,
                    _ => ConvertFormat::Jpeg,
                };

                let overwrite = overwrite_check_clone.is_checked();

                // Show progress and disable button
                process_btn_clone.deactivate();
                process_btn_clone.set_label("Converting...");
                progress_label_clone.set_label("Starting conversion...");
                app::redraw();

                // Run conversion with parallel processing
                let (success_count, error_count, errors) = convert_batch_parallel(
                    files,
                    format.clone(),
                    overwrite,
                    |_processed, _total| {
                        // Progress callback - simplified for FLTK
                    },
                );

                let format_name = match format {
                    ConvertFormat::Jpeg => "JPEG",
                    ConvertFormat::Png => "PNG",
                    ConvertFormat::Webp => "WebP",
                    ConvertFormat::Bmp => "BMP",
                    ConvertFormat::Gif => "GIF",
                };

                let message = if error_count == 0 {
                    format!(
                        "Successfully converted {} files to {}",
                        success_count, format_name
                    )
                } else {
                    let error_list = errors.join("\n");
                    format!(
                        "Conversion completed:\n{} successful\n{} failed\n\nErrors:\n{}",
                        success_count, error_count, error_list
                    )
                };

                if error_count == 0 {
                    dialog::show_info_dialog(&parent_clone, &message);
                    progress_label_clone.set_label("All conversions completed successfully!");
                } else {
                    dialog::show_error_dialog(&parent_clone, &message);
                    progress_label_clone.set_label("Conversion completed with errors");
                }

                // Reset UI
                process_btn_clone.activate();
                process_btn_clone.set_label("Convert All");
                app::redraw();
            }
        });
    }

    section.end();
    parent.add(&section);
}

fn update_file_list(browser: &mut Browser, files: &[PathBuf]) {
    browser.clear();

    for (index, file) in files.iter().enumerate() {
        let filename = file
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown file");

        let display_text = format!("{}. {}", index + 1, filename);
        browser.add(&display_text);
    }
    app::redraw();
}

fn style_primary_button(btn: &mut Button) {
    btn.set_color(Color::from_rgb(13, 110, 253));
    btn.set_selection_color(Color::from_rgb(10, 88, 202));
    btn.set_label_color(Color::White);
    btn.set_label_font(Font::HelveticaBold);
    btn.set_label_size(12);
    btn.set_frame(FrameType::RFlatBox);

    // Simple hover effect using built-in selection color
    btn.clear_visible_focus();
}

fn style_destructive_button(btn: &mut Button) {
    btn.set_color(Color::from_rgb(220, 53, 69));
    btn.set_selection_color(Color::from_rgb(187, 45, 59));
    btn.set_label_color(Color::White);
    btn.set_label_font(Font::HelveticaBold);
    btn.set_label_size(12);
    btn.set_frame(FrameType::RFlatBox);

    // Simple hover effect using built-in selection color
    btn.clear_visible_focus();
}

fn style_choice_widget(choice: &mut Choice) {
    choice.set_color(Color::from_rgb(33, 37, 41));
    choice.set_selection_color(Color::from_rgb(13, 110, 253));
    choice.set_label_color(Color::White);
    choice.set_text_color(Color::White);
    choice.set_text_size(12);
    choice.set_frame(FrameType::FlatBox);

    // Remove custom drawing that causes visibility issues

    // Clear visible focus for cleaner appearance
    choice.clear_visible_focus();
}

fn style_checkbox(checkbox: &mut CheckButton) {
    checkbox.set_label_color(Color::White);
    checkbox.set_label_font(Font::Helvetica);
    checkbox.set_label_size(12);
    checkbox.set_color(Color::from_rgb(33, 37, 41));
    checkbox.set_selection_color(Color::from_rgb(13, 110, 253));
    checkbox.set_frame(FrameType::FlatBox);

    // Remove custom drawing that causes visibility issues

    // Clear visible focus for cleaner appearance
    checkbox.clear_visible_focus();
}
