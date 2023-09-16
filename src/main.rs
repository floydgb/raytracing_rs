use std::rc::Rc;
use camera::Camera;
use hittable::HittableList;
use vec3::Vec3;
use sphere::Sphere;
use crate::material::{Lambertian, Metal};

mod vec3;
mod color;
mod ray;
pub mod hittable;
mod sphere;
mod rtweekend;
mod interval;
mod camera;
pub mod material;


fn main() {

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));

    let mut world: HittableList = HittableList::new(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

//    let mut world: HittableList = HittableList::new(Box::new(Sphere::new(Vec3::new(0.0, -100.0, -1.0), 100.0)));
//    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);

}
