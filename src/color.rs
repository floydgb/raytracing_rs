use std::fmt::format;

use crate::vec3::Vec3;

pub fn write_color(pixel_color: &Vec3) -> String {
    let result = format!("{} {} {}\n",
      (255.999 * pixel_color.x()) as u8,
      (255.999 * pixel_color.y()) as u8,
      (255.999 * pixel_color.z()) as u8); 
    return result;
}   