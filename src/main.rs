mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hit_list;
mod rt_weekend;
mod camera;
mod random;

use std::{io::{Write, stdout, BufWriter}, fs::File, rc::Rc};
use hit_list::HitList;
use hittable::Hittable;
use sphere::Sphere;
use rt_weekend::*;

use crate::camera::Camera;

const RESET_LINE: &str = "\x1B[2K\r"; 

fn ray_color(ray: &Ray, world: &HitList, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let white = Color::new(1.0, 1.0, 1.0);
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        let target = rec.point + rec.normal + Vec3::random_in_unit_sphere();
        0.5 * ray_color(&Ray::new(rec.point, target - rec.point), world, depth - 1)
    } else {
        let unit_dir = ray.direction.unit_vec();
        let alpha = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - alpha) * white + alpha * Color::new(0.5, 0.7, 1.0)
    }
}

fn write_color(f: &mut impl Write, color: &Color, samples_per_pixel: u32) {
    fn translate_rgb_to_int(value: f64) -> i64 {
        (value.clamp(0.0, 0.999) * 256.0) as i64
    }
    let mut r = color.0;
    let mut g = color.1;
    let mut b = color.2;

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    writeln!(
        f,
        "{} {} {}",
        translate_rgb_to_int(r),
        translate_rgb_to_int(g),
        translate_rgb_to_int(b)
    ).unwrap()
}

fn main() {
    let output = File::create("image.ppm").unwrap();
    let mut out = BufWriter::new(output);

    // Image
    let image_width = 400.0;
    let image_height = image_width / ASPECT_RATIO;
    let samples_per_pixel = 100;
    let max_depth = 50;

    writeln!(out, "P3").unwrap();
    writeln!(out, "{image_width} {image_height}").unwrap();
    writeln!(out, "255").unwrap();

    // World
    let mut world = HitList::default();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let rand_aa_modifier = || random::range(0.0, 1.0);

    // Render
    for iy in (0..image_height as i64).rev() {
        print!("{RESET_LINE}");
        print!("Scanlines remaining: {iy}");
        stdout().flush().unwrap();
        for ix in 0..image_width as i64 {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (ix as f64 + rand_aa_modifier()) / (image_width - 1.0);
                let v = (iy as f64 + rand_aa_modifier()) / (image_height - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            write_color(&mut out, &pixel_color, samples_per_pixel);
        }
    }
    print!("{RESET_LINE}");
    println!("Done.");
}
