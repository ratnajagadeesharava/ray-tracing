use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
mod vec3;
use vec3::{Color, Point3, Vec3};
mod ray;
use ray::Ray;
mod hittable;
use hittable::Hittable;
mod sphere;
use sphere::Sphere;
mod hittable_list;
use crate::hittable::HitRecord;
use crate::hittable_list::HittableList;
const ASPECT_RATIO: f64 = 1.7778;
const WHITE: Vec3 = Vec3 { e: (1.0, 1.0, 1.0) };
const BLUE: Vec3 = Vec3 { e: (0.5, 0.7, 1.0) };
const T_MAX: f64 = f64::MAX;
fn write_color(color: &Color) -> String {
    let r = color.x() as f64;
    let g = color.y() as f64;
    let b = (color.z()) as f64;
    let ir = (r * 255.99) as i32;
    let ig = (g * 255.99) as i32;
    let ib = (b * 255.99) as i32;
    format!("{} {} {} \n ", &ir, &ig, &ib)
}

fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

fn ray_color(ray: Ray, world: HittableList) -> Color {
    let mut hit_record: HitRecord = HitRecord::default();
    let unit: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    let unit_direction = ray.dir.normalize();
    let a: f64 = 0.5 * (unit_direction.y() + 1.0);
    if world.hit(ray, 0.001, T_MAX, &mut hit_record) {
        (hit_record.normal + unit) * 0.5
    } else {
        WHITE * (1.0 - a) + BLUE * a
    }
}

fn main() {
    // Image settings
    let image_height: i64 = 1080;
    let image_width: i64 = ((image_height as f64) * ASPECT_RATIO) as i64;

    //world
    let mut world:HittableList = HittableList::new();
    let sphere: Sphere = Sphere{ radius: 0.5, center:Point3::new(0.0, 0.0, -1.0) };
    world.add(Rc::new(sphere));
    let sphere2 : Sphere = Sphere { radius:100.0, center: Point3{e:(0.0 ,-100.5,-1.0)} };
    world.add(Rc::new(sphere2));

    // Camera settings
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = (viewport_height * image_width as f64) / image_height as f64;

    let camera_center: Vec3 = Point3::new(0.0, 0.0, 0.0);
    // Viewport vectors
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u: Vec3 = &viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = &viewport_v / image_height as f64;

    // PPM header
    let mut file: File = File::create("image.ppm").unwrap();
    let mut content: String = String::from("P3\n");

    let s: String = format!("{} {}", &image_width, &image_height);
    content.push_str(&s);
    content.push_str("\n255\n");

    // Viewport origin
    let view_port_center_from_top: Vec3 = (viewport_u + viewport_v) * 0.5;

    let view_port_upper_left: Vec3 =
        &(&camera_center - &Vec3::new(0.0, 0.0, focal_length)) - &view_port_center_from_top;

    let up_left_pixel: Vec3 =
        &(&view_port_upper_left + &(&pixel_delta_u * 0.5)) + &(&pixel_delta_v * 0.5);

    for i in 0..image_height {
        for j in 0..image_width {
            let pixel_distance: Vec3 = &pixel_delta_u * j as f64 + &pixel_delta_v * i as f64;
            let pixel_position: Vec3 = &pixel_distance + &up_left_pixel;

            let pixel_direction: Vec3 = &pixel_position - &camera_center;

            let ray: Ray = Ray::new(camera_center, pixel_direction);
            let pixel_color: Vec3 = ray_color(ray,world.clone());

            content.push_str(&write_color(&pixel_color));
        }
    }

    let buffer: &[u8] = &content.into_bytes();
    file.write_all(buffer).unwrap();
    println!("file written successfully");
}
