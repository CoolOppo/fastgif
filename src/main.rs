use gif::{Encoder, Repeat, SetParameter};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the file
    let args: Vec<String> = env::args().collect();

    let file_path = Path::new(&args[1]);
    let mut out_file_name = String::from(file_path.file_stem().unwrap().to_string_lossy());
    out_file_name.push_str("-fast.gif");
    let mut output_image = BufWriter::new(File::create(Path::new(&out_file_name)).unwrap());

    let decoder = gif::Decoder::new(BufReader::new(File::open(file_path).unwrap()));
    // Configure the decoder such that it will expand the image to RGBA.
    //sadlkfjalskfjlaksdjflkdsa decoder.set(gif::ColorOutput::RGBA);
    // Read the file header
    let mut decoder = decoder.read_info().unwrap();
    let mut encoder = Encoder::new(
        &mut output_image,
        decoder.width(),
        decoder.width(),
        decoder.global_palette().unwrap(),
    )
    .unwrap();
    encoder.set(Repeat::Infinite).unwrap();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        let mut new_frame = frame.clone();
        new_frame.delay = 2;
        encoder.write_frame(&new_frame).unwrap();
    }
    Ok(())
}
