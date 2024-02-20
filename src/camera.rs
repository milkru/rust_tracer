use super::vec::Vec3;
use super::ray::Ray;

pub struct Camera {
    origin: Vec3,
    hor_dir: Vec3,
    ver_dir: Vec3,
    lower_left_corner: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(look_from: Vec3,
               look_at: Vec3,
               vup: Vec3,
               vfov: f32,
               aspect_ratio: f32,
               aperture: f32,
               focus_dist: f32) -> Camera {
        let vfon_rad = std::f32::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (vfon_rad / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (look_from - look_at).norm();
        let cu = vup.cross(cw).norm();
        let cv = cw.cross(cu);

        let hor_dir = focus_dist * viewport_width * cu;
        let ver_dir = focus_dist * viewport_height * cv;

        let lower_left_corner = look_from - hor_dir / 2.0 - ver_dir / 2.0 - focus_dist * cw;

        Camera {
            origin: look_from,
            hor_dir,
            ver_dir,
            cu,
            cv,
            lower_left_corner,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rand_offset = self.lens_radius * Vec3::get_random_in_unit_disk();
        let lens_offset = rand_offset.x * self.cu + rand_offset.y * self.cv;

        Ray::new(self.origin + lens_offset,
                self.lower_left_corner + u * self.hor_dir + v * self.ver_dir - self.origin - lens_offset)
    }
}
