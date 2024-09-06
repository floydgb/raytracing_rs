// Assuming lib is a dependency in your Cargo.toml file
use crate::INF;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn initialize() -> Self {
        Self { min: INF, max: INF }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { self.min } else if x > self.max { self.max } else { x }
    }
}
