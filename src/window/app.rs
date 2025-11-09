use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, CheckButton, DropDown, Frame, Label,
    ListBox, ListBoxRow, Orientation, PolicyType, ScrolledWindow, Separator, StringList,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::convert::{convert_image, ConvertFormat};
use crate::window::dialog;

pub fn get_app() -> Application {
    let app = Application::builder()
        .application_id("com.example.file-upload-gui")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app
}

fn build_ui(app: &Application) {
    println!("Building UI...");
    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Image Converter Pro")
        .default_width(700)
        .default_height(600)
        .build();
    println!("Main window created");

    // Create the main container with better styling
    let main_box = Box::new(Orientation::Vertical, 24);
    main_box.set_margin_top(24);
    main_box.set_margin_bottom(24);
    main_box.set_margin_start(24);
    main_box.set_margin_end(24);
    main_box.add_css_class("main-container");

    // Title
    let title = Label::new(Some("Image Converter"));
    title.set_markup("<span size='large' weight='bold'>Image Converter</span>");
    title.set_halign(Align::Center);
    title.set_margin_bottom(16);
    main_box.append(&title);

    // Dark theme CSS
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(
        "
        window {
            background-color: #1a1a1a;
            color: #ffffff;
        }

        .main-container {
            background-color: #1a1a1a;
            color: #ffffff;
        }

        frame {
            background-color: #2d2d2d;
            border: 1px solid #404040;
            border-radius: 8px;
            margin: 8px 0;
        }

        frame > label {
            color: #ffffff;
            font-weight: bold;
            background-color: #333333;
            padding: 12px 16px;
            border-radius: 7px 7px 0 0;
        }

        button {
            background-color: #404040;
            color: #ffffff;
            border: 1px solid #666666;
            border-radius: 6px;
            padding: 8px 16px;
            font-weight: 500;
        }

        button:hover {
            background-color: #4a4a4a;
        }

        button.suggested-action {
            background-color: #0969da;
            border: 1px solid #0969da;
        }

        button.suggested-action:hover {
            background-color: #0860ca;
        }

        button.destructive-action {
            background-color: #da3633;
            border: 1px solid #da3633;
        }

        button.destructive-action:hover {
            background-color: #c93229;
        }

        label {
            color: #ffffff;
        }

        .dim-label {
            color: #8b949e;
        }

        entry {
            background-color: #21262d;
            color: #ffffff;
            border: 1px solid #30363d;
            border-radius: 6px;
        }

        dropdown {
            background-color: #21262d;
            color: #ffffff;
            border: 1px solid #30363d;
            border-radius: 6px;
        }

        dropdown > button {
            background-color: transparent;
            border: none;
            color: #ffffff;
        }

        checkbutton {
            color: #ffffff;
        }

        checkbutton check {
            background-color: #21262d;
            border: 1px solid #30363d;
            border-radius: 3px;
        }

        checkbutton check:checked {
            background-color: #0969da;
            border: 1px solid #0969da;
        }

        .boxed-list {
            background-color: #21262d;
            border: 1px solid #30363d;
            border-radius: 6px;
        }

        .boxed-list row {
            background-color: transparent;
        }

        .boxed-list row:hover {
            background-color: #2d3748;
        }

        .file-row {
            background-color: #262626;
            border: 1px solid #404040;
            border-radius: 6px;
            margin: 2px;
        }

        .file-row:hover {
            background-color: #333333;
        }

        scrolledwindow {
            background-color: #21262d;
            border: 1px solid #30363d;
            border-radius: 6px;
        }

        separator {
            background-color: #30363d;
            min-height: 1px;
        }
    ",
    );

    gtk4::style_context_add_provider_for_display(
        &gtk4::prelude::WidgetExt::display(&window),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Single file upload section
    let single_section = create_single_upload_section();
    main_box.append(&single_section);

    // Batch file upload section
    let batch_section = create_batch_upload_section();
    main_box.append(&batch_section);

    // State management
    let single_file: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let batch_files: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::new()));

    // Get widget references
    let single_file_label = get_widget_by_name(&single_section, "single_file_label")
        .unwrap()
        .downcast::<Label>()
        .unwrap();
    let single_select_btn = get_widget_by_name(&single_section, "single_select_btn")
        .unwrap()
        .downcast::<Button>()
        .unwrap();
    let single_clear_btn = get_widget_by_name(&single_section, "single_clear_btn")
        .unwrap()
        .downcast::<Button>()
        .unwrap();
    let format_dropdown = get_widget_by_name(&single_section, "format_dropdown")
        .unwrap()
        .downcast::<DropDown>()
        .unwrap();
    let convert_btn = get_widget_by_name(&single_section, "convert_btn")
        .unwrap()
        .downcast::<Button>()
        .unwrap();
    let convert_btn_for_select = convert_btn.clone();
    let convert_btn_for_clear = convert_btn.clone();
    let single_overwrite_check = get_widget_by_name(&single_section, "single_overwrite_check")
        .unwrap()
        .downcast::<CheckButton>()
        .unwrap();

    let batch_select_btn = get_widget_by_name(&batch_section, "batch_select_btn")
        .unwrap()
        .downcast::<Button>()
        .unwrap();
    let batch_format_dropdown = get_widget_by_name(&batch_section, "batch_format_dropdown")
        .unwrap()
        .downcast::<DropDown>()
        .unwrap();
    let batch_overwrite_check = get_widget_by_name(&batch_section, "batch_overwrite_check")
        .unwrap()
        .downcast::<CheckButton>()
        .unwrap();
    let clear_btn = get_widget_by_name(&batch_section, "clear_btn")
        .unwrap()
        .downcast::<Button>()
        .unwrap();
    let process_btn = get_widget_by_name(&batch_section, "process_btn")
        .unwrap()
        .downcast::<Button>()
        .unwrap();
    let file_list = get_widget_by_name(&batch_section, "file_list")
        .unwrap()
        .downcast::<ListBox>()
        .unwrap();

    // Connect single file upload
    {
        let window_clone = window.clone();
        let single_file_clone = single_file.clone();
        let label_clone = single_file_label.clone();

        single_select_btn.connect_clicked(move |_| {
            let single_file_clone = single_file_clone.clone();
            let label_clone = label_clone.clone();
            let convert_btn_clone = convert_btn_for_select.clone();

            dialog::open_single_file_dialog(&window_clone, move |path_opt| {
                if let Some(path) = path_opt {
                    *single_file_clone.borrow_mut() = Some(path.clone());
                    let filename = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("Unknown file");
                    label_clone.set_text(&format!("Selected: {}", filename));
                    label_clone.set_tooltip_text(Some(&path.to_string_lossy()));
                    convert_btn_clone.set_sensitive(true);
                }
            });
        });
    }

    // Connect single file clear button
    {
        let single_file_clone = single_file.clone();
        let label_clone = single_file_label.clone();
        let convert_btn_clone = convert_btn_for_clear.clone();

        single_clear_btn.connect_clicked(move |_| {
            *single_file_clone.borrow_mut() = None;
            label_clone.set_text("No file selected");
            label_clone.set_tooltip_text(None);
            convert_btn_clone.set_sensitive(false);
        });
    }

    // Connect convert button
    {
        let window_clone = window.clone();
        let single_file_clone = single_file.clone();
        let dropdown_clone = format_dropdown.clone();

        convert_btn.connect_clicked(move |_| {
            let single_file = single_file_clone.borrow();
            if let Some(input_path) = single_file.as_ref() {
                let selected_index = dropdown_clone.selected();
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

                let overwrite = single_overwrite_check.is_active();
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

                match convert_image(input_path.clone(), output_path.clone(), format) {
                    Ok(_) => {
                        let message = format!(
                            "Successfully converted image to:\n{}",
                            output_path.display()
                        );
                        dialog::show_info_dialog(&window_clone, &message);
                    }
                    Err(e) => {
                        let message = format!("Conversion failed: {}", e);
                        dialog::show_error_dialog(&window_clone, &message);
                    }
                }
            }
        });
    }

    // Connect batch file upload
    {
        let window_clone = window.clone();
        let batch_files_clone = batch_files.clone();
        let list_clone = file_list.clone();
        let process_btn_clone = process_btn.clone();

        batch_select_btn.connect_clicked(move |_| {
            let batch_files_clone = batch_files_clone.clone();
            let list_clone = list_clone.clone();
            let process_btn_clone = process_btn_clone.clone();

            dialog::open_multiple_files_dialog(&window_clone, move |paths| {
                if !paths.is_empty() {
                    batch_files_clone.borrow_mut().extend(paths);
                    update_file_list(&list_clone, &batch_files_clone.borrow(), &batch_files_clone);
                    process_btn_clone.set_sensitive(!batch_files_clone.borrow().is_empty());
                }
            });
        });
    }

    // Connect clear button
    {
        let batch_files_clone = batch_files.clone();
        let list_clone = file_list.clone();
        let process_btn_clone = process_btn.clone();

        clear_btn.connect_clicked(move |_| {
            batch_files_clone.borrow_mut().clear();
            clear_file_list(&list_clone);
            process_btn_clone.set_sensitive(false);
        });
    }

    // Connect process button
    {
        let window_clone = window.clone();
        let batch_files_clone = batch_files.clone();

        process_btn.connect_clicked(move |_| {
            let files = batch_files_clone.borrow();
            if !files.is_empty() {
                let selected_index = batch_format_dropdown.selected();
                let format = match selected_index {
                    0 => ConvertFormat::Jpeg,
                    1 => ConvertFormat::Png,
                    2 => ConvertFormat::Webp,
                    3 => ConvertFormat::Bmp,
                    4 => ConvertFormat::Gif,
                    _ => ConvertFormat::Jpeg,
                };

                let overwrite = batch_overwrite_check.is_active();
                let mut success_count = 0;
                let mut error_count = 0;
                let mut errors = Vec::new();

                for file_path in files.iter() {
                    let input_stem = file_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("converted");

                    let output_filename = if overwrite {
                        format!("{}.{}", input_stem, format.extension())
                    } else {
                        let mut counter = 1;
                        let base_path = file_path.parent().unwrap_or(std::path::Path::new("."));
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

                    let output_path = file_path
                        .parent()
                        .unwrap_or(std::path::Path::new("."))
                        .join(output_filename);

                    match convert_image(file_path.clone(), output_path.clone(), format.clone()) {
                        Ok(_) => {
                            success_count += 1;
                            println!("Successfully converted: {}", file_path.display());
                        }
                        Err(e) => {
                            error_count += 1;
                            let error_msg = format!(
                                "{}: {}",
                                file_path.file_name().unwrap_or_default().to_string_lossy(),
                                e
                            );
                            errors.push(error_msg);
                        }
                    }
                }

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
                    dialog::show_info_dialog(&window_clone, &message);
                } else {
                    dialog::show_error_dialog(&window_clone, &message);
                }
            }
        });
    }

    window.set_child(Some(&main_box));
    println!("UI setup complete, presenting window...");
    window.present();
    println!("Window presented");
}

fn create_single_upload_section() -> Box {
    let frame = Frame::new(Some("Single File Conversion"));
    let section_box = Box::new(Orientation::Vertical, 16);
    section_box.set_margin_top(16);
    section_box.set_margin_bottom(16);
    section_box.set_margin_start(16);
    section_box.set_margin_end(16);

    // File selection section
    let file_section = Box::new(Orientation::Vertical, 12);

    let button_box = Box::new(Orientation::Horizontal, 12);
    button_box.set_halign(Align::Fill);

    let select_button = Button::with_label("Select Image");
    select_button.add_css_class("suggested-action");
    select_button.set_widget_name("single_select_btn");

    let file_label = Label::new(Some("No file selected"));
    file_label.set_halign(Align::Start);
    file_label.set_hexpand(true);
    file_label.set_widget_name("single_file_label");
    file_label.add_css_class("dim-label");

    let clear_button = Button::with_label("Clear");
    clear_button.add_css_class("destructive-action");
    clear_button.set_widget_name("single_clear_btn");

    button_box.append(&select_button);
    button_box.append(&file_label);
    button_box.append(&clear_button);
    file_section.append(&button_box);

    // Separator
    let separator = Separator::new(Orientation::Horizontal);
    separator.set_margin_top(12);
    separator.set_margin_bottom(12);
    file_section.append(&separator);

    // Conversion options section
    let options_box = Box::new(Orientation::Vertical, 12);

    let format_box = Box::new(Orientation::Horizontal, 12);
    format_box.set_halign(Align::Fill);

    let format_label = Label::new(Some("Output Format:"));
    format_label.set_halign(Align::Start);

    let format_list = StringList::new(&["JPEG", "PNG", "WebP", "BMP", "GIF"]);
    let format_dropdown = DropDown::new(Some(format_list), None::<gtk4::Expression>);
    format_dropdown.set_selected(0);
    format_dropdown.set_widget_name("format_dropdown");

    let overwrite_check = CheckButton::with_label("Overwrite existing files");
    overwrite_check.set_widget_name("single_overwrite_check");
    overwrite_check.set_hexpand(true);

    let convert_button = Button::with_label("Convert");
    convert_button.add_css_class("suggested-action");
    convert_button.set_widget_name("convert_btn");
    convert_button.set_sensitive(false);

    format_box.append(&format_label);
    format_box.append(&format_dropdown);
    format_box.append(&overwrite_check);
    format_box.append(&convert_button);

    options_box.append(&format_box);

    section_box.append(&file_section);
    section_box.append(&options_box);
    frame.set_child(Some(&section_box));

    let container = Box::new(Orientation::Vertical, 0);
    container.append(&frame);
    container
}

fn create_batch_upload_section() -> Box {
    let frame = Frame::new(Some("Batch File Conversion"));
    let section_box = Box::new(Orientation::Vertical, 16);
    section_box.set_margin_top(16);
    section_box.set_margin_bottom(16);
    section_box.set_margin_start(16);
    section_box.set_margin_end(16);

    // File selection section
    let selection_box = Box::new(Orientation::Horizontal, 12);
    selection_box.set_halign(Align::Fill);

    let select_button = Button::with_label("Select Multiple Images");
    select_button.add_css_class("suggested-action");
    select_button.set_widget_name("batch_select_btn");

    let clear_button = Button::with_label("Clear All");
    clear_button.add_css_class("destructive-action");
    clear_button.set_widget_name("clear_btn");

    selection_box.append(&select_button);
    selection_box.append(&clear_button);
    section_box.append(&selection_box);

    // File list
    let list_frame = Frame::new(Some("Selected Files"));
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(PolicyType::Never, PolicyType::Automatic);
    scrolled_window.set_min_content_height(180);
    scrolled_window.set_margin_top(12);
    scrolled_window.set_margin_bottom(12);

    let file_list = ListBox::new();
    file_list.add_css_class("boxed-list");
    file_list.set_widget_name("file_list");
    file_list.set_selection_mode(gtk4::SelectionMode::None);
    scrolled_window.set_child(Some(&file_list));
    list_frame.set_child(Some(&scrolled_window));

    section_box.append(&list_frame);

    // Separator
    let separator = Separator::new(Orientation::Horizontal);
    separator.set_margin_top(12);
    separator.set_margin_bottom(12);
    section_box.append(&separator);

    // Conversion options section
    let options_box = Box::new(Orientation::Vertical, 12);

    let format_box = Box::new(Orientation::Horizontal, 12);
    format_box.set_halign(Align::Fill);

    let format_label = Label::new(Some("Output Format:"));
    format_label.set_halign(Align::Start);

    let format_list = StringList::new(&["JPEG", "PNG", "WebP", "BMP", "GIF"]);
    let batch_format_dropdown = DropDown::new(Some(format_list), None::<gtk4::Expression>);
    batch_format_dropdown.set_selected(0);
    batch_format_dropdown.set_widget_name("batch_format_dropdown");

    let overwrite_check = CheckButton::with_label("Overwrite existing files");
    overwrite_check.set_widget_name("batch_overwrite_check");
    overwrite_check.set_hexpand(true);

    let process_button = Button::with_label("Convert All Files");
    process_button.add_css_class("suggested-action");
    process_button.set_sensitive(false);
    process_button.set_widget_name("process_btn");

    format_box.append(&format_label);
    format_box.append(&batch_format_dropdown);
    format_box.append(&overwrite_check);
    format_box.append(&process_button);

    options_box.append(&format_box);
    section_box.append(&options_box);

    frame.set_child(Some(&section_box));

    let container = Box::new(Orientation::Vertical, 0);
    container.append(&frame);
    container
}

fn get_widget_by_name(container: &impl IsA<gtk4::Widget>, name: &str) -> Option<gtk4::Widget> {
    fn search_children(widget: &gtk4::Widget, name: &str) -> Option<gtk4::Widget> {
        if widget.widget_name() == name {
            return Some(widget.clone());
        }

        let mut child = widget.first_child();
        while let Some(current_child) = child {
            if let Some(found) = search_children(&current_child, name) {
                return Some(found);
            }
            child = current_child.next_sibling();
        }
        None
    }

    search_children(container.as_ref(), name)
}

fn update_file_list(list_box: &ListBox, files: &[PathBuf], files_rc: &Rc<RefCell<Vec<PathBuf>>>) {
    clear_file_list(list_box);

    for (index, file) in files.iter().enumerate() {
        let row = ListBoxRow::new();
        row.add_css_class("file-row");

        let main_box = Box::new(Orientation::Horizontal, 16);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(16);
        main_box.set_margin_end(16);

        // File info
        let info_box = Box::new(Orientation::Horizontal, 12);
        info_box.set_hexpand(true);

        let text_box = Box::new(Orientation::Vertical, 4);

        let filename = file
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown file");

        let name_label = Label::new(Some(&format!("{}. {}", index + 1, filename)));
        name_label.set_halign(Align::Start);
        name_label.set_markup(&format!("<b>{}. {}</b>", index + 1, filename));

        // File size and path info
        let file_info = if let Ok(metadata) = std::fs::metadata(file) {
            let size_kb = metadata.len() / 1024;
            let size_str = if size_kb > 1024 {
                format!("{:.1} MB", size_kb as f64 / 1024.0)
            } else {
                format!("{} KB", size_kb)
            };
            format!(
                "{} • {}",
                size_str,
                file.parent()
                    .unwrap_or_else(|| std::path::Path::new(""))
                    .display()
            )
        } else {
            file.parent()
                .unwrap_or_else(|| std::path::Path::new(""))
                .display()
                .to_string()
        };

        let info_label = Label::new(Some(&file_info));
        info_label.set_halign(Align::Start);
        info_label.add_css_class("dim-label");
        info_label.set_ellipsize(gtk4::pango::EllipsizeMode::Middle);

        text_box.append(&name_label);
        text_box.append(&info_label);

        info_box.append(&text_box);

        // Remove button
        let remove_button = Button::with_label("Remove");
        remove_button.add_css_class("destructive-action");
        remove_button.set_tooltip_text(Some("Remove file"));

        // Connect remove button
        let list_box_clone = list_box.clone();
        let files_rc_clone = files_rc.clone();
        let file_path = file.clone();
        remove_button.connect_clicked(move |_| {
            let mut files = files_rc_clone.borrow_mut();
            if let Some(pos) = files.iter().position(|f| f == &file_path) {
                files.remove(pos);
                update_file_list(&list_box_clone, &files, &files_rc_clone);
            }
        });

        main_box.append(&info_box);
        main_box.append(&remove_button);
        row.set_child(Some(&main_box));

        list_box.append(&row);
    }
}

fn clear_file_list(list_box: &ListBox) {
    while let Some(child) = list_box.last_child() {
        list_box.remove(&child);
    }
}
