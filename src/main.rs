use std::fs::File;
use std::io::prelude::*;
mod vec3;
use vec3::{Color, Point3, Vec3};
mod ray;
use ray::Ray;
mod hittable;
use hittable::Hittable
const ASPECT_RATIO: f64 = 1.7778;

fn write_color(color: &Color) -> String {
    let r = color.x() as f64;
    let g = color.y() as f64;
    let b = (color.z()) as f64;
    let ir = (r * 255.99) as i32;
    let ig = (g * 255.99) as i32;
    let ib = (b * 255.99) as i32;
    format!("{} {} {} \n ", &ir, &ig, &ib)
}

fn dot(v1:&Vec3,v2:&Vec3)->f64{
    v1.x()*v2.x()+v1.y()*v2.y()+v1.z()*v2.z()
}

fn hit_sphere(ray:&Ray,center:Point3,radius:f64)->f64{
    let oc =&center- &(ray.o);
    let dir = ray.dir;
    let a = dot(&dir,&dir);
    let b = -2.0*dot(&dir,&oc);
    let c = dot(&oc,&oc)-radius*radius;
    let discriminant = b*b-4.0*a*c;
    if discriminant < 0.0{
        -1.0
    }
    else {
        (-1.0*b-discriminant.sqrt())/(2.0*a)
    }

}

fn ray_color(ray: &Ray) -> Color {
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    let a = 0.5 * (ray.dir.y() + 1.0);
    let sphere_center = Point3::new(0.0,0.0,-1.0);
    let t = hit_sphere(ray, sphere_center, 0.5);
    if t!=-1.0{
        // println!("{t}");
        let  p = ray.at(t);
        let normal = (&p-&sphere_center).normalize();
        &(normal+Vec3::new(1.0,1.0,1.0))*0.5
    }
    else{
        white * (1.0 - a) + blue * a
    }
}


fn main() {
     let mut file = File::create("image.ppm").unwrap();
    let mut content: String = String::from("P3\n");
    let image_height: i64 = 1080;
    let image_width: i64 = ((image_height as f64) * ASPECT_RATIO) as i64;
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = (viewport_height * image_width as f64) / image_height as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let pixel_delta_u = &viewport_u / image_width as f64;
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_v = &viewport_v / image_height as f64;
    let s = format!("{} {}", &image_width, &image_height);
    let view_port_center_from_top: Vec3 = (&viewport_u + &viewport_v) * 0.5;
    let view_port_upper_left =&(&camera_center - &Vec3::new(0.0, 0.0, focal_length)) - &view_port_center_from_top;
    let up_left_pixel = &(&view_port_upper_left + &(&pixel_delta_u * 0.5)) + &(&pixel_delta_v * 0.5);
    content.push_str(&s);
    content.push_str("\n255\n");
    for i in 0..image_height {
        for j in 0..image_width {
            let pixel_distance = &pixel_delta_u * j as f64 + &pixel_delta_v * i as f64;
            let pixel_position = &pixel_distance + &up_left_pixel;
            let pixel_direction = &pixel_position - &camera_center;
            let ray = Ray::new(camera_center, pixel_direction);
            let pixel_color = ray_color(&ray);
            content.push_str(&write_color(&pixel_color));
        }
    }
    let buffer: &[u8] = &content.into_bytes();
    file.write_all(buffer).unwrap();
    println!("file written successfully");
}
