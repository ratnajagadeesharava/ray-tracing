use crate::{ray::Ray, vec3::{Point3, Vec3}};
pub struct  Hit_Record{
    pub point:Point3,
    pub normal:Vec3,
    pub t:f64
}
pub trait Hittable{
    fn hit(ray:Ray,t_min:f64,t_max:f64,hit_record:Hit_Record)->bool;
}