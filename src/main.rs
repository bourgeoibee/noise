extern crate rand;

use rand::{thread_rng, Rng};
use std::fs;
use std::io::Write;

fn main() {
    let width = 64;
    let height = 64;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("generated_noise.pgm")
        .expect("Failed to open file");

    let header = &format!("P2\n{} {} 255\n", width, height);

    let mut content = String::new();
    content.push_str(header);

    for _ in 0..height {
        fn rand() -> String {
            thread_rng().gen_range(0..256).to_string()
        }

        for _ in 0..width - 1 {
            content.push_str(&rand());
            content.push(' ');
        }

        content.push_str(&rand());
        content.push('\n');
    }

    write!(&mut file, "{text}").expect("Failed to write to file");
}
