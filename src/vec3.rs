use std::ops::{Add, Div, Mul, Sub};

use crate::utility::{float_random_value, float_random_value_in_range};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub e: (f64, f64, f64),
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: (x, y, z) }
    }
    // #[inline]
    // fn length_squared(&self)->f64{
    //     self.e.0*self.e.0+self.e.1*self.e.1+self.e.2*self.e.2
    // }
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }
    #[inline]
    pub fn length(&self) -> f64 {
        (self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2).sqrt()
    }
    #[inline]
    pub fn normalize(&self) -> Self {
        let l = self.length();
        self / l
    }
    pub fn x(&self) -> f64 {
        self.e.0
    }
    pub fn y(&self) -> f64 {
        self.e.1
    }
    pub fn z(&self) -> f64 {
        self.e.2
    }

    pub fn random() -> Vec3 {
        Vec3 {
            e: (
                float_random_value(),
                float_random_value(),
                float_random_value(),
            ),
        }
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: (
                float_random_value_in_range(min, max),
                float_random_value_in_range(min, max),
                float_random_value_in_range(min, max),
            ),
        }
    }

    pub fn random_unit_vector(normal: Vec3) -> Vec3 {
        let random_vector = Vec3::random_in_range(-1.0, 1.0);
        if Vec3::dot(&random_vector, &normal) > 0.0 {
            return random_vector.normalize();
        } else {
            return random_vector.normalize() * -1.0;
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            e: (self.e.0 + rhs.e.0, self.e.1 + rhs.e.1, self.e.2 + rhs.e.2),
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: (self.e.0 - rhs.e.0, self.e.1 - rhs.e.1, self.e.2 - rhs.e.2),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: (self.e.0 * rhs, self.e.1 * rhs, self.e.2 * rhs),
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: (self.e.0 * rhs, self.e.1 * rhs, self.e.2 * rhs),
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: (self.e.0 / rhs, self.e.1 / rhs, self.e.2 / rhs),
        }
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use super::Vec3;
    #[test]
    fn test_scalar_multiplication() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 = v1 * 3.9;
        println!("{:?}", v1);
    }
    #[test]
    fn test_substraction() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let v3 = Vec3::new(2.5, 3.4, 5.6);
        println!("{:?}", v2);
        println!("{:?}", v3);
        println!("{:?}", v1);
    }
}
