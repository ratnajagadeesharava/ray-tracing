use crate::{
    dot,
    ray::Ray,
    vec3::{Point3, Vec3},
};
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = dot(&ray.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            outward_normal.clone() * -1.0
        }
    }
}
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}
