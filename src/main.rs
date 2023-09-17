use std::rc::Rc;
use rand::{random, Rng};
use camera::Camera;
use hittable::HittableList;
use vec3::Vec3;
use sphere::Sphere;
use crate::material::{Lambertian, Metal, Dielectric};
use color::random_color;
use crate::color::random_color_min_max;

mod vec3;
mod color;
mod ray;
pub mod hittable;
mod sphere;
mod rtweekend;
mod interval;
mod camera;
pub mod material;

fn test() {
    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    let mut world: HittableList = HittableList::new(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

//    let mut world: HittableList = HittableList::new(Box::new(Sphere::new(Vec3::new(0.0, -100.0, -1.0), 100.0)));
//    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(-2.0, 2.0, 1.0);
    cam.look_at = Vec3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}


fn make_cover() {

    let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let mut world: HittableList = HittableList::new(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, -1.0), 1000.0, ground_material)));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = random::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * random::<f64>() );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_color();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = random_color_min_max(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..=0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }

        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

//    let mut world: HittableList = HittableList::new(Box::new(Sphere::new(Vec3::new(0.0, -100.0, -1.0), 100.0)));
//    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.sample_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.look_from = Vec3::new(13.0, 2.0, 3.0);
    cam.look_at = Vec3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

fn main() {

    make_cover();

}
