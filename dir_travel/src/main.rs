pub mod handlers;
use handlers::{format_file_size, multiple_threaded_travel};
use std::path::Path;

fn main() {
    let total_size = multiple_threaded_travel(Path::new("."));
    let total_size = format_file_size(&total_size);

    println!("Total size: {total_size}");
}
