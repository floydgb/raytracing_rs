use camera::Camera;
use hittable::HittableList;
use vec3::Vec3;
use sphere::Sphere;

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
    cam.sample_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);

}
