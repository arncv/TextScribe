use std::sync::{Arc, Mutex};
use std::fs;
use std::path::Path;
use base64;
use image::{ImageOutputFormat, ImageFormat, io::Reader as ImageReader};
use anyhow::{Result, Context};
use log::{info, warn};
use rayon::prelude::*;

pub fn optimize_image(img_path: &Path, quality: Option<u8>) -> Result<(Vec<u8>, ImageFormat)> {
    let img = image::open(img_path).context("Failed to open image")?;
    let mut optimized_img = Vec::new();

    let format = ImageReader::open(img_path)?.format().unwrap_or(ImageFormat::Png); // Default to PNG if format detection fails

    match format {
        ImageFormat::Png => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Png).context("Failed to write PNG image")?;
        },
        ImageFormat::Jpeg => {
            let quality = quality.unwrap_or(80); // Default to quality 80 if not specified
            img.write_to(&mut optimized_img, ImageOutputFormat::Jpeg(quality)).context("Failed to write JPEG image")?;
        },
        ImageFormat::Gif => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Gif).context("Failed to write GIF image")?;
        },
        ImageFormat::Bmp => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Bmp).context("Failed to write BMP image")?;
        },
        ImageFormat::Farbfeld => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Farbfeld).context("Failed to write Farbfeld image")?;
        },
        ImageFormat::Ico => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Ico).context("Failed to write ICO image")?;
        },
        _ => {
            img.write_to(&mut optimized_img, ImageOutputFormat::Png).context("Failed to write default PNG image")?;
        }
    }
    Ok((optimized_img, format))
}

pub fn embed_images_as_base64(
    html_output: &mut String,
    base_path: &Path,
    quality: Option<u8>,
) {
    let img_tag_pattern = "<img src=\"";
    let mut index = 0;

    let mut tasks: Vec<_> = Vec::new();

    while let Some(start) = html_output[index..].find(img_tag_pattern) {
        let start = start + index;
        let end = html_output[start..].find("\"").unwrap() + start + img_tag_pattern.len();
        let img_path_str = &html_output[start + img_tag_pattern.len()..end];
        let img_path = base_path.join(img_path_str);

        if fs::read(img_path.clone()).is_ok() {
            tasks.push((start, end, img_path.to_path_buf()));
        }

        index = end + 1;
    }

    let html_output_arc = Arc::new(Mutex::new(html_output.clone()));

    tasks.into_par_iter().for_each(|(start, end, img_path)| {
        match optimize_image(&img_path, quality) {
            Ok((optimized_data, img_format)) => {
                let encoded = base64::encode(&optimized_data);
                let prefix = get_data_url_prefix(img_format);
                let data_url = format!("{}{}", prefix, encoded);

                let mut html_output = html_output_arc.lock().expect("Failed to lock mutex");
                html_output.replace_range((start + img_tag_pattern.len())..end, &data_url);
                info!("Embedded image {} as base64", img_path.display());
            },
            Err(e) => {
                warn!("Failed to optimize image {}: {}", img_path.display(), e);
            }
        }
    });

    let final_html_output = Arc::try_unwrap(html_output_arc).expect("Failed to unwrap Arc").into_inner().expect("Failed to get inner value from Mutex");
    *html_output = final_html_output;
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