use image::{ImageFormat, ImageReader};
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ConvertFormat {
    Jpeg,
    Png,
    Webp,
    Bmp,
    Gif,
}

impl ConvertFormat {
    pub fn extension(&self) -> &str {
        match self {
            ConvertFormat::Jpeg => "jpg",
            ConvertFormat::Png => "png",
            ConvertFormat::Webp => "webp",
            ConvertFormat::Bmp => "bmp",
            ConvertFormat::Gif => "gif",
        }
    }

    pub fn image_format(&self) -> ImageFormat {
        match self {
            ConvertFormat::Jpeg => ImageFormat::Jpeg,
            ConvertFormat::Png => ImageFormat::Png,
            ConvertFormat::Webp => ImageFormat::WebP,
            ConvertFormat::Bmp => ImageFormat::Bmp,
            ConvertFormat::Gif => ImageFormat::Gif,
        }
    }
}

pub fn anything_to_jpg(path: PathBuf, output_path: PathBuf) -> Result<(), Error> {
    convert_image(path, output_path, ConvertFormat::Jpeg)
}

pub fn convert_image(
    input_path: PathBuf,
    output_path: PathBuf,
    format: ConvertFormat,
) -> Result<(), Error> {
    let img = ImageReader::open(&input_path)?.decode();
    match img {
        Ok(img) => {
            let result = img.save_with_format(&output_path, format.image_format());
            match result {
                Ok(_) => {
                    println!(
                        "Successfully converted {} to {}",
                        input_path.display(),
                        output_path.display()
                    );
                }
                Err(e) => {
                    eprintln!("Failed to save image: {}", e);
                    return Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!("Save failed: {}", e),
                    ));
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to convert image: {}", e);
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Conversion failed: {}", e),
            ));
        }
    }

    Ok(())
}

pub fn convert_batch_parallel(
    files: Vec<PathBuf>,
    format: ConvertFormat,
    overwrite: bool,
    progress_callback: impl Fn(usize, usize) + Send + Sync + 'static,
) -> (usize, usize, Vec<String>) {
    use rayon::prelude::*;
    use std::sync::{Arc, Mutex};

    let total_files = files.len();
    let success_count = Arc::new(Mutex::new(0));
    let error_count = Arc::new(Mutex::new(0));
    let errors = Arc::new(Mutex::new(Vec::new()));
    let processed_count = Arc::new(Mutex::new(0));

    // Process files in parallel using rayon
    files.par_iter().for_each(|file_path| {
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
                *success_count.lock().unwrap() += 1;
                println!("Successfully converted: {}", file_path.display());
            }
            Err(e) => {
                *error_count.lock().unwrap() += 1;
                let error_msg = format!(
                    "{}: {}",
                    file_path.file_name().unwrap_or_default().to_string_lossy(),
                    e
                );
                errors.lock().unwrap().push(error_msg);
            }
        }

        // Update progress
        let mut processed = processed_count.lock().unwrap();
        *processed += 1;
        progress_callback(*processed, total_files);
    });

    let final_success = *success_count.lock().unwrap();
    let final_errors = *error_count.lock().unwrap();
    let error_list = errors.lock().unwrap().clone();

    (final_success, final_errors, error_list)
}
