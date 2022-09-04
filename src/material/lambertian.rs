//!
//! Simple implementation of a diffuse material
//! 

extern crate linear_alg;
use linear_alg::{Vec3, rand_on_unit_sphere};

use super::Material;
use crate::utils::{Ray, HitInfo};

pub struct Lambertian {
    color: Vec3,
}

impl Material for Lambertian {

    fn scatter_ray(&self, hit_info: HitInfo) -> Ray {
        Ray::new(hit_info.pos + hit_info.normal * 0.0001, hit_info.normal + rand_on_unit_sphere())
    }

    fn get_attenuation(&self) -> Vec3 {
        return self.color;
    }
}

impl Lambertian {
    pub fn new(color: Vec3) -> Lambertian {
        Lambertian { color }
    }
}
