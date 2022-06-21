extern crate humansize;
use humansize::{file_size_opts as options, FileSize};
use std::fs::{metadata, read_dir};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

pub fn format_file_size(file_size: &u64) -> String {
    file_size.file_size(options::CONVENTIONAL).unwrap()
}

const PRINT_OUTPUT: bool = false;
fn get_file_size(path: &Path) -> u64 {
    let file_path = path.to_string_lossy();
    if let Ok(metadata) = metadata(&path) {
        if PRINT_OUTPUT {
            println!("{} - {}", file_path, format_file_size(&metadata.len()));
        }

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
        if PRINT_OUTPUT {
            println!("{} - Skipped", path.to_string_lossy());
        }
        return 0;
    }

    if path.is_file() {
        get_file_size(&path)
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

// TODO: Share the same thread pool...?
pub fn multiple_threaded_travel(path: &Path) -> u64 {
    if is_hidden(path) {
        if PRINT_OUTPUT {
            println!("{} - Skipped", path.to_string_lossy());
        }
        return 0;
    }

    if path.is_file() {
        get_file_size(&path)
    } else {
        if let Ok(entries) = read_dir(&path) {
            let total_size = Arc::new(Mutex::new(0));
            let mut threads = vec![];

            for entry in entries {
                if let Ok(entry) = entry {
                    let total_size_clone = total_size.clone();
                    threads.push(spawn(move || {
                        let mut total_size = total_size_clone.lock().unwrap();
                        *total_size += multiple_threaded_travel(&entry.path());
                    }));
                } else {
                    println!("{} - Cannot read DirEntry", path.to_string_lossy());
                }
            }

            for thread in threads {
                thread.join().unwrap();
            }

            let total_size = *total_size.lock().unwrap();
            total_size
        } else {
            println!("{} - Cannot read_dir", path.to_string_lossy());
            0
        }
    }
}
