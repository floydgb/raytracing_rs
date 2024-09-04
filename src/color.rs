use crate::{interval::Interval, vec3::Vec3};
use rand::Rng;

pub fn write_color(pixel_color: &Vec3, samples_per_pixel: u32) -> String {
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();
    let scale: f64 = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);
    let intensity = Interval::new(0.0, 0.999);
    let result = format!(
        "{} {} {}\n",
        (255.999 * intensity.clamp(r)) as u8,
        (255.999 * intensity.clamp(g)) as u8,
        (255.999 * intensity.clamp(b)) as u8
    );
    return result;
}

pub fn random_color() -> Vec3 {
    let r = rand::thread_rng().gen_range(0.0..=1.0);
    let g = rand::thread_rng().gen_range(0.0..=1.0);
    let b = rand::thread_rng().gen_range(0.0..=1.0);
    Vec3::new(r, g, b)
}

pub fn random_radius() -> f64 {
    rand::thread_rng().gen_range(0.1..=0.3)
}

pub fn random_color_min_max(min: f64, max: f64) -> Vec3 {
    let r = rand::thread_rng().gen_range(min..=max);
    let g = rand::thread_rng().gen_range(min..=max);
    let b = rand::thread_rng().gen_range(min..=max);
    Vec3::new(r, g, b)
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
