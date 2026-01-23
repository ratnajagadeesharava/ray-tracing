use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use crate::{
    hittable::HitRecord,
    hittable_list::HittableList,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

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
pub struct Camera {
    pub image_height: i64,
    image_width: i64,
    pub focal_length: f64,
    pub viewport_height: f64,
    pub center: Vec3,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(image_height: i64, focal_length: f64, viewport_height: f64, center: Point3) -> Self {
        let image_width: i64 = ((image_height as f64) * ASPECT_RATIO) as i64;
        let viewport_width: f64 = (viewport_height * image_width as f64) / image_height as f64;
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u: Vec3 = &viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = &viewport_v / image_height as f64;
        Self {
            image_height,
            image_width,
            focal_length,
            viewport_height,
            center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world:HittableList) {
        // PPM header
        let mut file: File = File::create("image.ppm").unwrap();
        let mut content: String = String::from("P3\n");

        let s: String = format!("{} {}", &self.image_width, &self.image_height);
        content.push_str(&s);
        content.push_str("\n255\n");
        let view_port_center_from_top: Vec3 = (self.viewport_u + self.viewport_v) * 0.5;

        let view_port_upper_left: Vec3 =
            &(&self.center - &Vec3::new(0.0, 0.0, self.focal_length)) - &view_port_center_from_top;

        let up_left_pixel: Vec3 =
            &(&view_port_upper_left + &(&self.pixel_delta_u * 0.5)) + &(&self.pixel_delta_v * 0.5);

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let pixel_distance: Vec3 = &self.pixel_delta_u * j as f64 + &self.pixel_delta_v * i as f64;
                let pixel_position: Vec3 = &pixel_distance + &up_left_pixel;

                let pixel_direction: Vec3 = &pixel_position - &self.center;

                let ray: Ray = Ray::new(self.center, pixel_direction);
                let pixel_color: Vec3 = Camera::ray_color(ray, world.clone());

                content.push_str(&write_color(&pixel_color));
            }
        }

        let buffer: &[u8] = &content.into_bytes();
        file.write_all(buffer).unwrap();
        println!("file written successfully");
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
}
