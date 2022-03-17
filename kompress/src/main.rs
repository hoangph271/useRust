#![feature(seek_stream_len)]

use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::path::{Path, PathBuf};

use anyhow::Result;
use directories::UserDirs;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

fn main() {
    if let Some(user_dirs) = UserDirs::new() {
        let downloads_dir = user_dirs.download_dir();

        match downloads_dir {
            Some(downloads_dir) => {
                let zip_path: PathBuf = [downloads_dir.to_string_lossy().as_ref(), "zip.zip"]
                    .iter()
                    .collect();

                // // * Read zip file
                // match read_zip_file(&zip_path) {
                //     Ok(file_count) => {
                //         println!("Filecount: {}", file_count);
                //     }
                //     Err(e) => {
                //         println!("Can NOT read the zip file...!");
                //         println!("{}", e)
                //     }
                // }

                // * Write zip file
                match create_zip_file(&zip_path, downloads_dir) {
                    Ok(file_size) => {
                        println!("Zip file size: {}", file_size);
                    }
                    Err(e) => {
                        println!("Can NOT create the zip file...!");
                        println!("{}", e);
                    }
                }
            }
            None => {
                panic!("Can NOT get download dirs")
            }
        }
    } else {
        panic!("Can NOT get user dirs")
    }
}

fn create_zip_file(zip_path: &Path, downloads_path: &Path) -> Result<u64> {
    let file_writer = File::options()
        .create(true)
        .write(true)
        .append(false)
        .open(zip_path)?;

    let mut zip_writter = ZipWriter::new(&file_writer);
    let options = FileOptions::default().compression_method(CompressionMethod::Stored);

    let entries = read_dir(downloads_path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|_| Error::new(ErrorKind::Other, "oh no!"))?;

        if file_name == "zip.zip" {
            continue
        }

        zip_writter.start_file(&file_name, options)?;

        let file_reader = File::options().read(true).open(entry.path())?;
        const BASE: u64 = 2;
        let mut buffer = [0; BASE.pow(12) as usize];

        loop {
            let reader = BufReader::new(&file_reader);
            let size = &reader.take(BASE.pow(12)).read(&mut buffer)?;

            if *size == 0 {
                println!("{} saved...!", file_name);
                break;
            } else {
                zip_writter.write_all(&buffer)?;
            }
        }
    }

    Ok(zip_writter.finish()?.stream_len()?)
}

#[allow(dead_code)]
fn read_zip_file(zip_path: &Path) -> Result<usize> {
    let zip_reader = File::open(zip_path)?;
    let mut zip = ZipArchive::new(zip_reader)?;

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        println!("Filename: {}", file.name());
    }

    Ok(zip.len())
}
