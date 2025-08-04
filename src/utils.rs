use image::Rgba;
use std::fmt::Display;

pub fn print_clr<T: Display>(col: Rgba<T>) {
    println!("{}", col.0.map(|e| e.to_string()).join(" "));
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn weak_random() -> u32 {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let nanos = since_epoch.subsec_nanos();
    (nanos ^ 0x5f3759df) % 100 + 1 // samo za ilustraciju
}
