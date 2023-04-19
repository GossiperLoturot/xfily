use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
}

const IMAGE_FORMAT: &[image::ImageFormat] = &[
    image::ImageFormat::Gif,
    image::ImageFormat::Jpeg,
    image::ImageFormat::Png,
    image::ImageFormat::Pnm,
    image::ImageFormat::Tga,
    image::ImageFormat::Tiff,
    image::ImageFormat::WebP,
    image::ImageFormat::Bmp,
];

#[wasm_bindgen]
pub fn convert_image(in_data: &[u8], in_format: usize, out_format: usize) -> Box<[u8]> {
    let reader = Cursor::new(in_data.to_vec());
    let img = image::load(reader, IMAGE_FORMAT[in_format]).unwrap();

    let mut writer = Cursor::new(Vec::new());
    img.write_to(&mut writer, IMAGE_FORMAT[out_format]).unwrap();
    writer.into_inner().into_boxed_slice()
}
