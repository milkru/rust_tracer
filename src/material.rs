use super::vec::Vec3;
use super::ray::Ray;
use super::hittable::HitRecord;
use rand::Rng;

pub trait Scatter : Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        // Unused: supress warning
        let _ = ray_in;
        
        let mut scatter_dir = rec.normal + Vec3::get_random_in_unit_sphere();
        if scatter_dir.near_zero()
        {
            scatter_dir = rec.normal;
        }

        let scattered = Ray::new(rec.point, scatter_dir);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray_in.dir.reflect(rec.normal).norm();
        let fuzz_offset = self.fuzz * Vec3::get_random_in_unit_sphere();
        let scattered = Ray::new(rec.point, reflected + fuzz_offset);

        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        }
        else {
            None
        }
    }
}

pub struct Dielectric {
    refr_index: f32
}

impl Dielectric {
    pub fn new(refr_index: f32) -> Dielectric {
        Dielectric { refr_index }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let refr_ratio = if rec.front_face {
            1.0 / self.refr_index
        } else {
            self.refr_index
        };

        let unit_dir = ray_in.dir.norm();

        let cos_theta = ((-1.0) * unit_dir).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refr_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f32>() < Self::reflectance(cos_theta, refr_ratio);

        let direction = if cannot_refract || will_reflect {
            unit_dir.reflect(rec.normal)
        } else {
            unit_dir.refract(rec.normal, refr_ratio)
        };

        let scattered = Ray::new(rec.point, direction);

        Some((Vec3::new(1.0, 1.0, 1.0), scattered))
    }
}
