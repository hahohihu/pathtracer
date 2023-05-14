mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hit_list;
mod rt_weekend;

use std::{io::{Write, stdout, BufWriter}, fs::File, rc::Rc};
use hit_list::HitList;
use hittable::Hittable;
use sphere::Sphere;
use rt_weekend::*;

const RESET_LINE: &str = "\x1B[2K\r"; 

fn ray_color(ray: &Ray, world: &HitList) -> Color {
    let white = Color::new(1.0, 1.0, 1.0);
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + white)
    } else {
        let unit_dir = ray.direction.unit_vec();
        let alpha = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - alpha) * white + alpha * Color::new(0.5, 0.7, 1.0)
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

    // World
    let mut world = HitList::default();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

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
            let color = ray_color(&ray, &world);
            color.show_as_color(&mut out);
        }
    }
    print!("{RESET_LINE}");
    println!("Done.");
}
