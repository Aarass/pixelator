use crate::{section::Section, settings::get_settings};
use image::ImageReader;

fn main() {
    let settings = get_settings();

    let img = ImageReader::open(settings.input_path)
        .unwrap()
        .decode()
        .unwrap();

    let w = img.width();
    let h = img.height();

    let mut section = Section::new(100, 100, w - 200, h - 200);
    section.subdivide(2);
    section.init(&img);

    let img = section.fill_leaves(img);

    img.save(settings.output_path).unwrap();
}

// let mut section = Section::new(200, 60, 100, 100);

mod section;
mod settings;
mod utils;
