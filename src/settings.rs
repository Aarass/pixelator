use std::{collections::HashMap, env};

pub struct Settings {
    pub input_path: String,
    pub output_path: String,
    pub subdivisions: u32,
}

pub fn get_settings() -> Settings {
    let values = take_input();

    Settings {
        input_path: values.get("input_path").expect("No input path").to_owned(),
        output_path: values
            .get("output_path")
            .expect("No output path")
            .to_owned(),
        subdivisions: values
            .get("subdivisions")
            .map(|s| s.parse().expect("Subdivisions must be a number"))
            .unwrap_or(2),
    }
}

fn take_input() -> HashMap<&'static str, String> {
    let mut args = env::args().skip(1);

    let mut values = HashMap::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-o" => {
                let next = args.next();
                let output_path = next.expect("Expected output path after -o");

                let prev = values.insert("output_path", output_path);
                if let Some(_) = prev {
                    eprintln!("Warning: Output path was already set.")
                }
            }
            "-s" => {
                let next = args.next();
                let subdivisions = next.expect("Expected number of subdivisions after -s");

                let prev = values.insert("subdivisions", subdivisions);
                if let Some(_) = prev {
                    eprintln!("Warning: Number of subdivisions was already set.")
                }
            }
            _ => {
                if values.contains_key("input_path") {
                    panic!("Unexpected argument")
                }

                values.insert("input_path", arg);
            }
        }
    }

    return values;
}
