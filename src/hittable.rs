use super::vec::Vec3;
use super::ray::Ray;
use super::material::Scatter;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub mat: Arc<dyn Scatter>,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable : Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub type Scene = Vec<Box<dyn Hittable>>;

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut tmp_rec = None;

        for hittable in self {
            if let Some(rec) = hittable.hit(ray, t_min, closest) {
                closest = rec.t;
                tmp_rec = Some(rec);
            }
        }
        
        tmp_rec
    }
}
