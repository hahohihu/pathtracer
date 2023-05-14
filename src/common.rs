pub mod random;
pub mod ray;
pub mod vec3;

pub use std::f64::consts::PI;

pub use ray::*;
pub use vec3::*;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
