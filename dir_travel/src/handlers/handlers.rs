extern crate humansize;
use humansize::{file_size_opts as options, FileSize};
use std::fs::{metadata, read_dir};
use std::path::Path;

pub fn format_file_size(file_size: &u64) -> String {
    file_size.file_size(options::CONVENTIONAL).unwrap()
}

fn print_file_metadata(path: &Path) -> u64 {
    let file_path = path.to_string_lossy();
    if let Ok(metadata) = metadata(&path) {
        println!("{} - {}", file_path, format_file_size(&metadata.len()));
        metadata.len()
    } else {
        println!("{} - Unknown file size", file_path);
        0
    }
}
fn is_hidden(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        file_name.to_string_lossy().starts_with(".")
    } else {
        false
    }
}

pub fn single_threaded_travel(path: &Path) -> u64 {
    if is_hidden(path) {
        println!("{} - Skipped", path.to_string_lossy());
        return 0;
    }

    if path.is_file() {
        print_file_metadata(&path)
    } else {
        if let Ok(entries) = read_dir(&path) {
            let mut total_file_size = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    total_file_size += single_threaded_travel(&entry.path());
                } else {
                    println!("{} - Cannot read DirEntry", path.to_string_lossy());
                }
            }

            total_file_size
        } else {
            println!("{} - Cannot read_dir", path.to_string_lossy());
            0
        }
    }
}
// TODO: Singled threaded vs multiple threaded...!
