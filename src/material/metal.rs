//!
//! Simple implementation of a metalic material
//! 

extern crate linear_alg;
use linear_alg::{Vec3, reflect, rand_on_unit_sphere};

use crate::utils::{Ray, HitInfo};

use super::Material;

pub struct Metal {
    color: Vec3,
    lbd: f64,
}

impl Material for Metal {

    fn scatter_ray(&self, hit_info: HitInfo) -> Ray {
        let reflected_ray = reflect(hit_info.in_ray*(-1), hit_info.normal);
        Ray::new(
            hit_info.pos + hit_info.normal * 0.0001,
             reflected_ray + rand_on_unit_sphere() * self.lbd
            )
    }

    fn get_attenuation(&self) -> Vec3 {
        return self.color;
    }
}

impl Metal {
    pub fn new(color: Vec3, lbd: f64) -> Metal {
        Metal { color, lbd }
    }
}
