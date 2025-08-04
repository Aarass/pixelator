use std::{collections::HashMap, env};

pub struct Settings {
    pub input_path: String,
    pub output_path: String,
}

pub fn get_settings() -> Settings {
    let values = take_input();

    Settings {
        input_path: values.get("input_path").expect("No input path").to_owned(),
        output_path: values
            .get("output_path")
            .expect("No output path")
            .to_owned(),
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
