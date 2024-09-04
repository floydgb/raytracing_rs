use core::f64;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub static PI: f64 = f64::consts::PI;
pub static INF: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180.0
}
