use std::{fs::File, io::Write};
use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use indicatif::ProgressBar;
use vec3::{Vec3, unit_vector};
use ray::Ray;
use sphere::Sphere;
use interval::Interval;

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod rtweekend;
mod interval;
mod camera;



fn main() {

    let mut world: HittableList = HittableList::new(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

//    let mut world: HittableList = HittableList::new(Box::new(Sphere::new(Vec3::new(0.0, -100.0, -1.0), 100.0)));
//    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));

   let mut cam: Camera = Camera::new();
   cam.aspect_ratio = 16.0 / 9.0;
   cam.image_width = 400;

   cam.render(&world);

}
