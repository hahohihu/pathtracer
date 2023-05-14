mod camera;
mod hit_list;
mod hittable;
mod random;
mod ray;
mod rt_weekend;
mod sphere;
mod vec3;
mod material;
mod metal;

use hit_list::HitList;
use hittable::Hittable;
use rt_weekend::*;
use sphere::Sphere;
use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
    rc::Rc,
};

use crate::{camera::Camera, metal::Metal};

const RESET_LINE: &str = "\x1B[2K\r";

fn ray_color(ray: &Ray, world: &HitList, depth: usize) -> Color {
    let black = Color::new(0.0, 0.0, 0.0);
    if depth == 0 {
        return black;
    }
    let white = Color::new(1.0, 1.0, 1.0);
    if let Some(hit_rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(mat_rec) = hit_rec.material.scatter(ray, &hit_rec) {
            mat_rec.attenuation * ray_color(&mat_rec.scattered, world, depth - 1)
            // let target = hit_rec.point + hit_rec.normal + Vec3::random_unit_vector();
            // let target = rec.point + Vec3::random_in_hemisphere(&rec.normal);
            // 0.5 * ray_color(&Ray::new(hit_rec.point, target - hit_rec.point), world, depth - 1)
        } else {
            black
        }
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
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    writeln!(
        f,
        "{} {} {}",
        translate_rgb_to_int(r),
        translate_rgb_to_int(g),
        translate_rgb_to_int(b)
    )
    .unwrap()
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
    world.add(Rc::new(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8))))));
    // world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

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
