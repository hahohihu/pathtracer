mod camera;
mod common;
mod hittable;
mod material;

use common::*;
use hittable::Hittable;
use hittable::{hit_list::HitList, sphere::Sphere};
use material::Material;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
    sync::Arc,
};

use crate::camera::Camera;

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

fn make_random_scene() -> HitList {
    let mut world = HitList::default();
    let ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));
    for a in -11..11 {
        for b in -11..11 {
            let material_choice = random::unit();
            let center = Point::new(
                a as f64 + 0.9 * random::unit(),
                0.2,
                b as f64 + 0.9 * random::unit(),
            );
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: Arc<dyn Material> = if material_choice < 0.8 {
                    let albedo = Color::random_unit() * Color::random_unit();
                    Arc::new(Lambertian::new(albedo))
                } else if material_choice < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random::range(0.0, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Arc::new(Sphere::new(center, 0.2, mat)));
            }
        }
    }
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}

fn main() {
    let output = File::create("image.ppm").unwrap();
    let mut out = BufWriter::new(output);

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 1;
    let max_depth = 50;

    writeln!(out, "P3").unwrap();
    writeln!(out, "{image_width} {image_height}").unwrap();
    writeln!(out, "255").unwrap();

    // World
    let world = make_random_scene();

    // Camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    let worker_count = 8;

    let rand_aa_modifier = || random::range(0.0, 1.0);
    std::thread::scope(|s| {
        let mut workers = vec![];
        let mut i = image_height as i64;
        let step = image_height as i64 / worker_count + 1;
        while i > 0 {
            let worker_start = i;
            i -= step;
            let worker_stop = i.max(0);
            let world = &world;
            let camera = &camera;
            workers.push(s.spawn(move || {
                let mut res = vec![];
                for iy in (worker_stop..worker_start).rev() {
                    for ix in 0..image_width as i64 {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..samples_per_pixel {
                            let u = (ix as f64 + rand_aa_modifier()) / (image_width - 1.0);
                            let v = (iy as f64 + rand_aa_modifier()) / (image_height - 1.0);
                            let ray = camera.get_ray(u, v);
                            pixel_color += ray_color(&ray, world, max_depth);
                        }
                        res.push(pixel_color);
                    }
                }
                res
            }));
        }
        for worker in workers {
            let res = worker.join().unwrap();
            for color in res {
                write_color(&mut out, &color, samples_per_pixel);
            }
        }       
        print!("{RESET_LINE}");
        println!("Done.");
    })
}
