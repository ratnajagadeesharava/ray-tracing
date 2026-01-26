use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use crate::{
    hittable::HitRecord, hittable_list::HittableList, ray::Ray, utility::{float_random_value, write_color}, vec3::{Color, Point3, Vec3}
};

const ASPECT_RATIO: f64 = 1.7778;
const WHITE: Vec3 = Vec3 { e: (1.0, 1.0, 1.0) };
const BLUE: Vec3 = Vec3 { e: (0.5, 0.7, 1.0) };
const T_MAX: f64 = f64::MAX;
const SAMPLES_PER_PIXEL:i32 = 10;
const PIXEL_SAMPLE_SCALE:f64 = 1.0 / SAMPLES_PER_PIXEL as f64;
const MAX_DEPTH:i32  =2;
pub struct Camera {
    pub image_height: i64,
    image_width: i64,
    pub center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_up_left:Vec3  //up left fixel of the image before camera
}

impl Camera {
    pub fn new(image_height: i64, focal_length: f64, viewport_height: f64, center: Point3) -> Self {
        let image_width: i64 = ((image_height as f64) * ASPECT_RATIO) as i64;
        let viewport_width: f64 = (viewport_height * image_width as f64) / image_height as f64;
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u: Vec3 = &viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = &viewport_v / image_height as f64;
         let view_port_center_from_top: Vec3 = (viewport_u + viewport_v) * 0.5;

        let view_port_upper_left: Vec3 =
            &(&center - &Vec3::new(0.0, 0.0, focal_length)) - &view_port_center_from_top;

        let pixel_up_left: Vec3 =
            &(&view_port_upper_left + &(&pixel_delta_u * 0.5)) + &(&pixel_delta_v * 0.5);
        Self {
            image_height,
            image_width,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel_up_left,
        }
    }
    pub fn render(&self, world:HittableList) {
        // PPM header
        let mut file: File = File::create("image.ppm").unwrap();
        let mut content: String = String::from("P3\n");

        let s: String = format!("{} {}", &self.image_width, &self.image_height);
        content.push_str(&s);
        content.push_str("\n255\n");
       println!("writing to the file. please wait....");

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let mut pixel_color:Color = Vec3 { e: (0.0,0.0,0.0) };
                for _ in 1..SAMPLES_PER_PIXEL{
                    let ray: Ray = self.get_ray(i, j);
                    pixel_color = &pixel_color + &Camera::ray_color(ray,MAX_DEPTH, world.clone());
                }
                content.push_str(&write_color(&pixel_color * PIXEL_SAMPLE_SCALE));
            }
        }
        // println!("{content}");
        let buffer: &[u8] = &content.into_bytes();
        file.write_all(buffer).unwrap();
        println!("file written successfully");
    }

    pub fn sample_square()->Vec3{
        Vec3 { e: (float_random_value()-0.5,float_random_value()-0.5,0.0) }
    }


    pub fn get_ray(&self,i:i64,j:i64)->Ray{
        let offset = Camera::sample_square();
        let pixel_distance: Vec3 =  &self.pixel_delta_u * (j as f64+ offset.x()) + &self.pixel_delta_v * (i as f64 +offset.y()) ;
        let pixel_position: Vec3 = &pixel_distance +&self.pixel_up_left;
        let pixel_direction: Vec3 = &pixel_position-&self.center;
        let ray:Ray = Ray::new(self.center,pixel_direction);
        ray
    }

    fn ray_color(ray: Ray,depth:i32, world: HittableList) -> Color {
        if depth <= 0{
            return Color{e:(0.0,0.0,0.0)};
        }
        let mut hit_record: HitRecord = HitRecord::default();
        let unit: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let unit_direction: Vec3 = ray.dir.normalize();
        let a: f64 = 0.5 * (unit_direction.y() + 1.0);
        if world.hit(ray, 0.001, T_MAX, &mut hit_record) {
            let direction = Vec3::random_unit_vector(hit_record.normal);
            let r = Ray::new(hit_record.point, direction);
            &Self::ray_color(r,depth-1,world)*0.5
            //  (direction + unit) * 0.5
        } else {
            WHITE * (1.0 - a) + BLUE * a
        }
    }
}
