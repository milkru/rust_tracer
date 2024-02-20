use super::vec::Vec3;
use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};
use super::material::Scatter;
use std::sync::Arc;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Arc<dyn Scatter>) -> Sphere {
        Sphere { center, radius, mat }
    }
}

// (x−Cx)^2+(y−Cy)^2+(z−Cz)^2=r^2
// (P−C)⋅(P−C)=r^2
// (A+tb−C)⋅(A+tb−C)=r^2
// t^2b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r^2=0
// Solve as quadratic equation x = (-b +- sqrt(b^2 - 4ac)) / 2a (use minus for closer point)
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let a = ray.dir.dot(ray.dir);
        let oc = ray.origin - self.center;
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
    
        if discriminant < 0.0 {
            return None
        }
        
        let mut quad_res = (-b - discriminant.sqrt()) / (2.0 * a);
        if quad_res < t_min || quad_res > t_max {
            quad_res = (-b + discriminant.sqrt()) / (2.0 * a);
            if quad_res < t_min || quad_res > t_max {
                return None    
            }
        }

        let point = ray.at(quad_res);
        let normal = (point - self.center).norm();

        let mut rec = HitRecord{
            point,
            normal,
            mat: self.mat.clone(),
            t: quad_res,
            front_face: ray.dir.dot(normal) < 0.0};

        if !rec.front_face {
            rec.normal = -1.0 * normal;
        }

        Some(rec)
    }
}
