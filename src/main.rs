mod vec3;

use std::{io::{Write, stdout, BufWriter}, fs::File};

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
            let r = ix as f64 / (image_width as f64 - 1.0);
            let g = iy as f64 / (image_height as f64 - 1.0);
            let b = 0.25;

            let ir = (r * 255.999) as i64;
            let ig = (g * 255.999) as i64;
            let ib = (b * 255.999) as i64;

            writeln!(out, "{ir} {ig} {ib}").unwrap();
        }
    }
    print!("{RESET_LINE}");
    println!("Done.");
}
