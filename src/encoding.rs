// encoding.rs
use crate::shapes::Hue;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufWriter;
use std::path;
use std::path::Path;
use std::path::PathBuf;

/*pub fn create_pixel_array<const DIM: usize>() -> [u8; DIM] {
    let [0; DIM]
}

#[test]
fn make_arrays() {
    let dimension = DIM as u32;
    let mut array: [u8; DIM] = create_pixel_array(1);
    let b = create_pixel_array(5);
    println!("Array a: {:?}", a);
    println!("Array a: {:?}", a);
}*/
/*#[macro_export]
macro_rules! create_array {
   (&length:expr) => {{
        use constant_function::{create_

 */
/*
fn hues_to_u8_slice(hues: Vec<Hue>) -> [u8] {
    let mut split_pixels: Vec<u8> = Vec::new();
    for i in hues {
        split_pixels.push(i.r as u8);
        split_pixels.push(i.g as u8);
        split_pixels.push(i.b as u8);
        split_pixels.push(i.a as u8);
    }
    return split_pixels[..];
}

#[test]
fn hue_conversion_test() {
    let mut v: Vec<Hue> = Vec::new();
    v.push(Hue::default());
    v.push(Hue::new(1.0, 2.0, 3.0, 4.0));
    let h = hues_to_u8_slice(v);
    println!("{:?}", h);
}
*/

pub fn test_encode(pixels: &Vec<Hue>, width: u32, height: u32) {
    //pixel_wh_agreement(pixels, width, height); add back in later with errors
    let mut split_pixels: Vec<u8> = Vec::new();
    for i in pixels {
        split_pixels.push(i.r as u8);
        split_pixels.push(i.g as u8);
        split_pixels.push(i.b as u8);
        split_pixels.push(i.a as u8);
    }
    let path = Path::new(r"./xenakis_test.png");
    let file = File::create(path).unwrap();
    //let ref mut w = BufWriter::new(file);
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    let source_chromaticities = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&split_pixels[..]).unwrap();

    //let res: std::result::Result<(), std::io::Error> = std::result::Result<(), std::io::Error>;
}

#[test]
fn encoding_test() {
    let v: Vec<Hue> = vec![
        Hue::new(0.0, 0.0, 0.0, 255.0),
        Hue::new(255.0, 0.0, 0.0, 255.0),
    ];
    test_encode(&v, 2, 1);
}

//   return std::result::Result::ok(writer.write_image_data(&pixels).unwrap());
//type Result<T> = std::result::Result<T, PixelCountError>;

/*type Result<T> = std::result::Result<T, std::io::Error>;
#[derive(Debug, Clone)]
struct PixelCountError;

impl fmt::Display for PixelCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width/height and pixels don't add up.")
    }
}

fn pixel_wh_agreement(pixels: Vec<Hue>, width: u32, height: u32) -> bool {
    if pixels.len() == usize::try_from(width * height).unwrap() {
        true
    } else {
        false
    }
}

pub fn make_encode_path(filename: String) -> std::result::Result<PathBuf, std::io::Error> {
    let mut current_path = env::current_exe()?;
    //let path = Path::new;
    current_path.push(r"/");
    current_path.push(filename);
    current_path.set_extension("png");

    let path = Path::new(&current_path.into_os_string().into_string());
    Ok(path)
}
pub fn encode(filename: String, pixels: Vec<Hue>, width: u32, height: u32) {
    //pixel_wh_agreement(pixels, width, height); add back in later with errors
    let path = make_encode_path(filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    let source_chromaticities = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();

    //let res: std::result::Result<(), std::io::Error> = std::result::Result<(), std::io::Error>;
    return;
}
//   return std::result::Result::ok(writer.write_image_data(&pixels).unwrap());
*/
