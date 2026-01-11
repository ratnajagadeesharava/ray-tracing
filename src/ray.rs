
use crate::vec3::{Point3, Vec3};
pub struct Ray{
    pub o:Point3,
    pub dir:Vec3
} 


impl Ray {
    pub fn new(origin:Point3,direction:Vec3)->Self{
        Self { o: origin, dir: direction }
    }
    pub fn at(&self,t:f64)->Point3{
        let direction  = &self.dir * t;
        &self.o + &direction
    }
}
