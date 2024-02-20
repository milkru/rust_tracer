use std::ops::{Add, Sub, Mul, Div, Range};
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Vec3 {
    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    
    pub fn len(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn norm(self) -> Vec3 {
        let len_sqare = self.x * self.x + self.y * self.y + self.z * self.z;
        let len_inv = 1.0 / len_sqare.sqrt();
        Vec3 {
            x: self.x * len_inv,
            y: self.y * len_inv,
            z: self.z * len_inv,
        }
    }

    pub fn get_random(range: Range<f32>) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range.clone()),
        }
    }

    pub fn get_random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::get_random(-1.0..1.0);
            if p.len() < 1.0 {
                return p;
            }
        }
    }

    pub fn get_random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0);

            if p.len() < 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(self) -> bool {
        const EPS: f32 = 1.0e-8;
        self.x.abs() < EPS && self.x.abs() < EPS && self.x.abs() < EPS
    }
    
    pub fn reflect(self, norm: Vec3) -> Vec3 {
        self - 2.0 * self.dot(norm) * norm
    }

    pub fn refract(self, norm: Vec3, refr_ratio: f32) -> Vec3 {
        let cos_theta = ((-1.0) * self).dot(norm).min(1.0);
        let perp = refr_ratio * (self + cos_theta * norm);
        let paral = -(1.0 - perp.len().powi(2)).abs().sqrt() * norm;
        perp + paral
    }
}
