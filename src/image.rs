use std::{fs::File, io::BufWriter};

extern crate png;

pub fn create_image(name: &str, width: u32, height: u32, data: &[u8]) {
    let image = File::create(name).unwrap();
    let ref mut w = BufWriter::new(image);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap(); // Save
    writer.finish().unwrap();
}