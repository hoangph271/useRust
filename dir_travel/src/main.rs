pub mod handlers;
use handlers::handlers::{format_file_size, single_threaded_travel};
use std::path::Path;

fn main() {
    let total_size = single_threaded_travel(&Path::new("."));
    let total_size = format_file_size(&total_size);

    println!("Total size: {total_size}");
}

// TODO: impl travel functions
// - List out files and size
// - List out number of skipped directories
// - Total size in GB
