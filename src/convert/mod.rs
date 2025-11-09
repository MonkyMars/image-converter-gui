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
