extern crate rand;

use rand::{thread_rng, Rng};
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;

struct Config {
    width: u32,
    height: u32,
    output_name: String,
}

// -o noise.ppm 1024 500
// 3024 -o noise 1500
// 3024 1500 -o n.ppm
fn parse_config(arguments: Vec<String>) -> Config {
    let mut i = 0;
    let mut output_name = String::from("generated_noise.ppm");
    let mut width = 500;
    let mut height = 500;
    let mut has_width = false;
    while i < arguments.len() {
        if arguments[i] == "-o" {
            if let Some(name) = arguments.get(i + 1) {
                output_name = name.clone();
            }
            i += 1;
        } else {
            if let Ok(parsed) = arguments[i].parse::<u32>() {
                if has_width {
                    height = parsed;
                } else {
                    width = parsed;
                    has_width = true;
                }
            }
        }
        i += 1;
    }

    Config {
        width,
        height,
        output_name,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = parse_config(args);

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(config.output_name)
        .expect("Failed to open file");

    let header = &format!("P2\n{} {} 255\n", config.width, config.height);

    let mut content = String::new();
    content.push_str(header);

    for _ in 0..config.height {
        fn rand() -> String {
            thread_rng().gen_range(0..256).to_string()
        }

        for _ in 0..config.width - 1 {
            content.push_str(&rand());
            content.push(' ');
        }

        content.push_str(&rand());
        content.push('\n');
    }

    write!(&mut file, "{content}").expect("Failed to write to file");
    Ok(())
}
