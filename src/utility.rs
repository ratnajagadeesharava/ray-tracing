use rand::Rng;

use crate::{
    vec3::Color
};



pub fn write_color(color: Color) -> String {
    let r = color.x() as f64;
    let g = color.y() as f64;
    let b = (color.z()) as f64;
    let ir = (r * 255.99) as i32;
    let ig = (g * 255.99) as i32;
    let ib = (b * 255.99) as i32;
    format!("{} {} {} \n ", &ir, &ig, &ib)
}

#[inline]
pub fn float_random_value()->f64{
    let mut rng = rand::rng();
    let val = rng.random();
    val
}

#[inline]
pub fn float_random_value_in_range(min:f64,max:f64)->f64{
    let mut rng = rand::rng();
    let val = rng.random_range(min..=max);
    val
}



#[test]
fn test_random(){
    let val = float_random_value();
    println!("{val}")
}


#[test]
fn test_random_range(){
    let val= float_random_value_in_range(-8.0,8.0, );
    println!("random varaible in the rang 4.0 to 8.0 {val}")
}