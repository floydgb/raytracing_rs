use crate::{
    interval::Interval,
    material::{Lambertian, Material},
    ray::Ray,
    vec3::{dot, Vec3},
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Hit {
    pub p: Vec3,
    pub norm: Vec3,
    pub t: f64,
    pub frnt: bool,
    pub mat: Rc<dyn Material>,
}

impl Hit {
    pub fn new(p: Vec3, norm: Vec3, t: f64, frnt: bool, mat: Rc<dyn Material>) -> Self {
        Hit {
            p,
            norm,
            t,
            frnt,
            mat,
        }
    }

    pub fn initialize() -> Self {
        Hit {
            p: Vec3::new(0.0, 0.0, 0.0),
            norm: Vec3::new(0.0, 0.0, 0.0),
            t: 1e8,
            frnt: false,
            mat: Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.frnt = dot(r.direction(), outward_normal) < 0.0;
        if self.frnt {
            self.norm = outward_normal;
        } else {
            self.norm = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut Hit) -> bool {
        let mut temp_rec: Hit = Hit::initialize();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max;
        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }
}
