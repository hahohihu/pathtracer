mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hit_list;

use std::{io::{Write, stdout, BufWriter}, fs::File};
use hittable::Hittable;
use ray::Ray;
use sphere::Sphere;
use vec3::*;

const RESET_LINE: &str = "\x1B[2K\r"; 

fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> f64 {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    if let Some(record) = sphere.hit(ray, 0.0, 5.0) {
        record.time
    } else {
        -1.0
    }
}

fn ray_color(ray: &Ray) -> Color {
    let center = Point::new(0.0, 0.0, -1.0);
    let time = hit_sphere(&center, 0.5, ray);
    if time > 0.0 {
        let unit_vec = ray.at(time) - center;
        let unit_vec = unit_vec.unit_vec();
        0.5 * (unit_vec + 1.0)
    } else {
        let unit_dir = ray.direction.unit_vec();
        let alpha = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - alpha) * Color::new(1.0, 1.0, 1.0) + alpha * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let output = File::create("image.ppm").unwrap();
    let mut out = BufWriter::new(output);

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width / aspect_ratio;

    writeln!(out, "P3").unwrap();
    writeln!(out, "{image_width} {image_height}").unwrap();
    writeln!(out, "255").unwrap();

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    for iy in (0..image_height as i64).rev() {
        print!("{RESET_LINE}");
        print!("Scanlines remaining: {iy}");
        stdout().flush().unwrap();
        for ix in 0..image_width as i64 {
            let u = (ix as f64) / (image_width - 1.0);
            let v = (iy as f64) / (image_height - 1.0);
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let color = ray_color(&ray);
            color.show_as_color(&mut out);
        }
    }
    print!("{RESET_LINE}");
    println!("Done.");
}
