use std::{rc::Rc, time::Instant};
mod vec3;
use vec3::{ Point3, Vec3};
mod ray;
mod hittable;
mod sphere;
use sphere::Sphere;
mod hittable_list;
use crate::hittable_list::HittableList;
mod camera;
use crate::camera::Camera;
mod utility;


fn main() {
   let start_timer = Instant::now();
    let camera: Camera = Camera::new(1080, 1.0, 2.0, Point3::new(0.0, 0.0, 0.0));

    //world
    let mut world:HittableList = HittableList::new();
    let sphere: Sphere = Sphere{ radius: 0.5, center:Point3::new(0.0, 0.0, -1.0) };
    world.add(Rc::new(sphere));
    let sphere2 : Sphere = Sphere { radius:100.0, center: Point3{e:(0.0 ,-100.5,-1.0)} };
    world.add(Rc::new(sphere2));

    camera.render(world);

    let duration = start_timer.elapsed();

    println!("total rendering time {:?}",duration);
}
