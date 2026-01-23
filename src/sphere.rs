use crate::vec3::{Point3,Vec3};
use crate::hittable::{Hittable,HitRecord};
use crate::ray::{Ray};
pub struct Sphere{
    pub radius:f64,
    pub center:Point3
}

impl Hittable for Sphere {
    fn hit(&self,ray:Ray,t_min:f64,t_max:f64,hit_record:&mut HitRecord)->bool {
        let oc =&self.center- &(ray.o);
        let dir = ray.dir;
        let a = Vec3::dot(&dir,&dir);
        let b = -2.0*Vec3::dot(&dir,&oc);
        let c = Vec3::dot(&oc,&oc)-self.radius*self.radius;
        let discriminant = b*b-4.0*a*c;
        if discriminant <0.0{
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-b-sqrtd)/(2.0*a);
        if root<=t_min ||root>= t_max{
            root = (-b+sqrtd)/(2.0*a);
            if root<=t_min ||root>= t_max{
                return false;
            }
        }
        let point = ray.at(root);
        let normal  = &point-&self.center;
        hit_record.point = point;
        hit_record.normal = normal.normalize();
        hit_record.set_face_normal(ray, normal.normalize());
        hit_record.t = root;
        true
    }
}
