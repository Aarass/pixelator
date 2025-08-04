use image::Rgba;
use std::fmt::Display;

pub fn print_clr<T: Display>(col: Rgba<T>) {
    println!("{}", col.0.map(|e| e.to_string()).join(" "));
}
