mod vec3;

use std::{io::{Write, stdout, BufWriter}, fs::File};
use vec3::*;

const RESET_LINE: &str = "\x1B[2K\r"; 

fn main() {
    let image_width = 256;
    let image_height = 256;
    let output = File::create("image.ppm").unwrap();
    let mut out = BufWriter::new(output);

    writeln!(out, "P3").unwrap();
    writeln!(out, "{image_width} {image_height}").unwrap();
    writeln!(out, "255").unwrap();

    for iy in (0..image_height).rev() {
        print!("{RESET_LINE}");
        print!("Scanlines remaining: {iy}");
        stdout().flush().unwrap();
        for ix in 0..image_width {
            let color = Color::new(ix as f64 / (image_width as f64 - 1.0), iy as f64 / (image_height as f64 - 1.0), 0.25);

            writeln!(out, "{color}").unwrap();
        }
    }
    print!("{RESET_LINE}");
    println!("Done.");
}
