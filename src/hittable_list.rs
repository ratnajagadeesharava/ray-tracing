use crate::{

    hittable::{HitRecord,
    Hittable},
    ray::Ray,
    vec3::Point3
    
};

use std::rc::Rc;

#[derive(Clone,Default)]
pub struct HittableList{
    pub list:Vec<Rc<dyn Hittable>>,
    pub size:usize
}

impl HittableList{
    pub fn new()->Self{
        Self{
            list:Vec::<Rc<dyn Hittable>>::new(),
            size :0
        }
    }
    pub fn add(&mut self,object:Rc<dyn Hittable>){
        self.list.push(object);
        self.size +=1;
    }
    pub fn hit(&self,ray:Ray,t_min:f64,t_max:f64,hit_record:&mut HitRecord)->bool{
        let mut temp_record:HitRecord= HitRecord{
            point:Point3::new(0.0,0.0,0.0),
            normal:Point3::new(0.0,0.0,0.0),
            t:0.0,
            front_face:false
        };
        let mut hit_anything =false;
        let mut closest_so_far = t_max;
        for record in &self.list{
            if record.hit(ray.clone(),t_min,closest_so_far,&mut temp_record){
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record.clone();
            }
        }

        hit_anything    
    }
}
