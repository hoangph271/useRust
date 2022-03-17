use immeta::load_from_file;
use immeta::markers::Png;

fn main() {
    if let Ok(metadata) = load_from_file("code.png") {
        let dimensions = metadata.dimensions();
        println!("{}x{}", dimensions.width, dimensions.height);

        println!("It's a {} image...!", metadata.mime_type());

        let metadata = metadata.into::<Png>();
        match metadata {
            Ok(val) => {
                println!(
                    "color_type: {}, color_depth: {}",
                    val.color_type, val.color_depth
                );

                println!("{} - {} - {}", val.compression_method, val.filter_method, val.interlace_method)
            }
            Err(_) => println!("NOT sure if this is a Png...!"),
        }
    } else {
        println!("Can't read image metadata...!")
    }
}
