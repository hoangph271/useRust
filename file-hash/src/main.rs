use std::fs::File;
use std::path::PathBuf;
use std::io::{BufReader, Read};
use anyhow::Result;
use ring::digest::{Digest, SHA256, Context};
use data_encoding::BASE64;

fn sha256_digest<R: Read> (mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let bytes_count = reader.read(&mut buffer)?;

        if bytes_count == 0 {
            break;
        } else {
            context.update(&buffer[..bytes_count]);
        }
    }

    Ok(context.finish())
}

fn main() -> Result<()> {
    let path: PathBuf = ["src", "main.rs"].iter().collect();
    let input_file = File::open(path)?;
    let input_reader = BufReader::new(input_file);

    // * Get SHA 256 digest
    let sha256 = sha256_digest(input_reader)?;

    // * Get SHA 256 digest in base64
    let sha256_base64 = BASE64.encode(sha256.as_ref());
    println!("{}", sha256_base64);

    Ok(())
}
