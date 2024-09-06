use crate::{
    color::color_to_string,
    degrees_to_radians,
    hittable::{ Hit, HitList },
    interval::Interval,
    ray::Ray,
    vec3::{ cross, random_in_unit_disk, unit_vector, Vec3 },
};
use indicatif::ProgressBar;
use rand::rngs::ThreadRng;
use std::{ fs::File, io::Write };

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

fn ray_color(r: Ray, depth: u32, world: &HitList, rng: &mut ThreadRng) -> Vec3 {
    let mut rec: Hit = Hit::default();
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if world.hit(&r, Interval::new(0.001, crate::INF), &mut rec) {
        let mut scatt = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut atten = Vec3::new(0.0, 0.0, 0.0);
        if rec.clone().mat.scatter(&r, &mut rec, &mut atten, &mut scatt, rng) {
            return atten * ray_color(scatt, depth - 1, world, rng);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = unit_vector(r.direction());
    let a: f64 = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 400,
            image_height: 400,
            center: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            sample_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: Vec3::new(0.0, 0.0, -1.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.center = self.look_from;
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));
        self.w = unit_vector(self.look_from - self.look_at);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);
        let viewport_u: Vec3 = viewport_width * self.u;
        let viewport_v: Vec3 = viewport_height * -self.v;
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);
        let viewport_upper_left: Vec3 =
            self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(&mut self, world: &HitList) {
        self.initialize();
        let mut rng = ThreadRng::default();
        match File::create("image.ppm") {
            Ok(mut file) => {
                write!(&mut file, "P3\n{} {}\n255\n", self.image_width, self.image_height).expect(
                    "Write Succeeds"
                );
                let progress_bar = ProgressBar::new(self.image_height as u64);
                let mut buf = String::new();
                buf.reserve((self.image_width * self.image_height * 12) as usize);
                for j in 0..self.image_height {
                    progress_bar.inc(1);
                    for i in 0..self.image_width {
                        let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                        for _sample in 0..self.sample_per_pixel {
                            let r: Ray = self.get_ray(i, j, &mut rng);
                            pixel_color += ray_color(r, self.max_depth, world, &mut rng);
                        }
                        let result = color_to_string(&pixel_color, self.sample_per_pixel);
                        buf.push_str(&result);
                    }
                }
                file.write_all(buf.as_bytes()).expect("Write Succeeds");
                progress_bar.finish_with_message("Done!");
            }
            Err(e) => { println!("Could not open file... {}", e) }
        }
    }

    fn get_ray(&mut self, i: u32, j: u32, rng: &mut ThreadRng) -> Ray {
        let pixel_center: Vec3 =
            self.pixel00_loc + (i as f64) * self.pixel_delta_u + (j as f64) * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = self.defocus_disk_sample(rng);
        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Vec3 {
        let p = random_in_unit_disk(rng);
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }

    fn pixel_sample_square(&mut self) -> Vec3 {
        let px: f64 = -0.5 + rand::random::<f64>();
        let py: f64 = -0.5 + rand::random::<f64>();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }
}
