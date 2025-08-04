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

    let mut section = Section::new(0, 0, w, h);
    section.subdivide(settings.subdivisions);
    section.init(&img);

    let img = section.pixelate(img, settings.threshold);

    img.save(settings.output_path).unwrap();
}

// let mut section = Section::new(200, 60, 100, 100);

mod section;
mod settings;
mod utils;
