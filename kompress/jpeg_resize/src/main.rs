use std::fs;
use std::fs::DirEntry;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use anyhow::{Context, Result};
use image::io::Reader as ImageReader;
use image::ImageFormat;

fn get_input_entries() -> Result<impl Iterator<Item = DirEntry>> {
    const INPUT_DIR: &str = "input-jpeg";

    let entries = fs::read_dir(INPUT_DIR)?.map(|val| -> DirEntry {
        val.map_err(|_| Error::new(ErrorKind::Other, "oh no!"))
            .unwrap()
    });

    Ok(entries)
}

fn main() -> Result<()> {
    const OUTPUT_DIR: &str = "output-jpeg";
    fs::create_dir_all(OUTPUT_DIR)?;

    get_input_entries()?
        .try_for_each(|entry| -> Result<()> {
            let entry_path = &entry.path();

            let image_reader = ImageReader::open(entry_path).with_context(|| {
                format!("Error opening image: {}", entry_path.to_string_lossy())
            })?;
            let (_width, _height) = image_reader.into_dimensions()?;

            // ? _width, _height are just for demonstration
            let (width, height) = (1200, 900);

            let output_path: PathBuf = [OUTPUT_DIR, &entry.file_name().to_string_lossy()]
                .iter()
                .collect();

            let image_reader = ImageReader::open(entry_path).with_context(|| {
                format!("Error opening image: {}", entry_path.to_string_lossy())
            })?;

            let _ = &image_reader
                .decode()?
                .thumbnail(width / 2, height / 2)
                .save_with_format(&output_path, ImageFormat::Jpeg)
                .with_context(|| {
                    format!("Error writing thumbnail: {}", output_path.to_string_lossy())
                })?;

            Ok(())
        })
        .with_context(|| "Error creating thumbnails...!")?;

    Ok(())
}
