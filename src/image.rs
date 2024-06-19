// src/image.rs
use std::fs;
use std::path::Path;
use base64;
use image::{ImageOutputFormat, ImageFormat, io::Reader as ImageReader};

pub fn optimize_image(img_path: &Path) -> Result<(Vec<u8>, ImageFormat), image::ImageError> {
    let img = image::open(img_path)?;
    let mut optimized_img = Vec::new();

    let format = ImageReader::open(img_path)?.format().unwrap_or(ImageFormat::Png); // Default to PNG if format detection fails

    match format {
        ImageFormat::Png => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Png)?;
        },
        ImageFormat::Jpeg => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Jpeg(80))?; // 80 is the quality setting
        },
        ImageFormat::Gif => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Gif)?;
        },
        ImageFormat::Bmp => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Bmp)?;
        },
        ImageFormat::Farbfeld => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Farbfeld)?;
        },
        ImageFormat::Ico => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Ico)?;
        },
        _ => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Png)?;
        }
    }
    Ok((optimized_img, format))
}

pub fn embed_images_as_base64(html_output: &mut String, base_path: &Path) {
    let img_tag_pattern = "<img src=\"";
    let mut index = 0;

    while let Some(start) = html_output[index..].find(img_tag_pattern) {
        let start = start + index;
        let end = html_output[start..].find("\"").unwrap() + start + img_tag_pattern.len();
        let img_path_str = &html_output[start + img_tag_pattern.len()..end];
        let img_path = base_path.join(img_path_str);

        if fs::read(img_path.clone()).is_ok() {
            let (optimized_data, img_format) = match optimize_image(&img_path) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Warning: Failed to optimize image {}: {}", img_path.display(), e);
                    (fs::read(img_path.clone()).unwrap(), ImageFormat::Png) // default to PNG if optimization fails
                }
            };
            
            let encoded = base64::encode(&optimized_data);
            let prefix = get_data_url_prefix(img_format);
            let data_url = format!("{}{}", prefix, encoded);
            html_output.replace_range((start + img_tag_pattern.len())..end, &data_url);
        }

        index = end + 1;
    }
}

fn get_data_url_prefix(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Png => "data:image/png;base64,",
        ImageFormat::Jpeg => "data:image/jpeg;base64,",
        ImageFormat::Gif => "data:image/gif;base64,",
        ImageFormat::Bmp => "data:image/bmp;base64,",
        ImageFormat::Farbfeld => "data:image/ff;base64,",
        ImageFormat::Ico => "data:image/ico;base64",
        _ => "data:image/png;base64,", // default to PNG
    }
}