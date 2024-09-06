use crate::{
    hittable::Hit,
    ray::Ray,
    vec3::{ dot, random_unit_vector, reflect, refract, unit_vector, Vec3 },
};
use rand::{ random, rngs::ThreadRng };

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut Hit,
        atten: &mut Vec3,
        scat: &mut Ray,
        rng: &mut ThreadRng
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &mut Hit,
        atten: &mut Vec3,
        scatt: &mut Ray,
        rng: &mut ThreadRng
    ) -> bool {
        let mut scatter_direction = rec.norm + random_unit_vector(rng);
        if scatter_direction.near_zero() {
            scatter_direction = rec.norm;
        }
        *scatt = Ray::new(rec.p, scatter_direction);
        *atten = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let mut f = fuzz;
        if fuzz < 0.0 || fuzz > 1.0 {
            f = 1.0;
        }
        Metal { albedo, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut Hit,
        atten: &mut Vec3,
        scatt: &mut Ray,
        rng: &mut ThreadRng
    ) -> bool {
        let reflected: Vec3 = reflect(unit_vector(r_in.direction()), rec.norm);
        *scatt = Ray::new(rec.p, reflected + self.fuzz * random_unit_vector(rng));
        *atten = self.albedo;
        dot(scatt.direction(), rec.norm) > 0.0
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut Hit,
        atten: &mut Vec3,
        scatt: &mut Ray,
        _rng: &mut ThreadRng
    ) -> bool {
        *atten = Vec3::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = self.ir;
        if rec.frnt {
            refraction_ratio = 1.0 / self.ir;
        }
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.norm).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random::<f64>() {
            direction = reflect(unit_direction, rec.norm);
        } else {
            direction = refract(unit_direction, rec.norm, refraction_ratio);
        }
        *scatt = Ray::new(rec.p, direction);
        true
    }
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
