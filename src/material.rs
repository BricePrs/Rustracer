//!
//! This module is intended to set the standards to use a material so that one can easily append
//! new behaviours.
//! There is exactly one static instance of a material throughout the entire process, the only 
//! thing that differs are values specific to each object in the scene.
//! 

extern crate linear_alg;
use linear_alg::Vec3;

use crate::utils::{Ray, HitInfo};

pub trait Material {
    fn scatter_ray(&self, hit_info: HitInfo) -> Ray;
    fn get_attenuation(&self) -> Vec3;
}

mod lambertian;
mod metal;
mod dielectric;

pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielectric::Dielectric;