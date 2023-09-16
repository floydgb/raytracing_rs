use core::panic;
use std::ops::{Add, Sub, Div, Mul, Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign};
use rand::{random, Rng};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3{

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn random() -> Self {
        Self {
            x: random::<f64>(),
            y: random::<f64>(),
            z: random::<f64>(),
        }
    }

    pub fn random_minmax(min: f64, max: f64) -> Self {
        Self {
            x: rand::thread_rng().gen_range(min..max),
            y: rand::thread_rng().gen_range(min..max),
            z: rand::thread_rng().gen_range(min..max),
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1E-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }  

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

}

// overloading for the addition operator +
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

// overloading for the unary negation operator -
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

// overloading for the substraction operation -
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// overloading for the multiplication operation for Vec3 * f64
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// overloading the multiplication operation for f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}


// overloading for the mumtiplication by float
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

//overloading for the divide operator /
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t:f64) -> Self::Output {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

// overloading for the [] operation
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bound"),
        }
    }
}

// overloading for the [] operation in the mutable context 
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bound"),
        }
    }
}

// Overloading for the += operation
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Overloading for the *= operation
impl MulAssign<f64> for Vec3 {

    fn mul_assign(&mut self, t: f64) {
        self.x = self.x * t;
        self.y = self.y * t;
        self.z = self.z * t;
    }
}

impl DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, t: f64) {
        self.x = self.x / t;
        self.y = self.y / t;
        self.z = self.z / t;
    }
}


pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let result = v / v.length();
    result
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::random_minmax(-1.0,1.0);
    while p.length_squared() > 1.0 {
        p = Vec3::random_minmax(-1.0, 1.0);
    }
    p
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere: Vec3 = random_unit_vector();
    if dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}