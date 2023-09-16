use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, reflect, unit_vector, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}


pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self { Lambertian{albedo} }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self{ Metal{albedo} }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected: Vec3 = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}