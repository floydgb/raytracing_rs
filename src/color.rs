use crate::vec3::Vec3;
use crate::interval::Interval;

pub fn write_color(pixel_color: &Vec3, samples_per_pixel: u32) -> String {
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();

    let scale: f64 = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    let intensity = Interval::new(0.000, 0.999);

    let result = format!("{} {} {}\n",
      (255.999 * intensity.clamp(r)) as u8,
      (255.999 * intensity.clamp(g)) as u8,
      (255.999 * intensity.clamp(b)) as u8);
    return result;
}   