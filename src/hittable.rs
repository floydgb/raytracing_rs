use crate::vec3::{Vec3, dot};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face : bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool, material: Rc<dyn Material>) -> Self {
        HitRecord{
            p: p,
            normal: normal,
            t: t,
            front_face: front_face,
            material: material,
        }
    }

    pub fn initialize() -> Self {
        HitRecord { 
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0), 
            t: 1E8, 
            front_face: false,
            material: Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        let front_face: bool = dot(r.direction(), outward_normal) < 0.0;
        if front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>    
}

impl HittableList {
    pub fn new(object: Box<dyn Hittable> ) -> Self {
        Self { objects: vec!(object) }
    }

    pub fn add(& mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::initialize();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;

    }
}