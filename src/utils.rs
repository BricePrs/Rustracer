//!
//! This module provides basic utility struct and functions for a raytracer.
//! 

extern crate linear_alg;
use linear_alg::Vec3;

use rand::Rng;

#[derive(Debug)]
pub struct Ray {
    pub ogn: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(ogn: Vec3, dir: Vec3) -> Ray {
        Ray {
            ogn,
            dir,
        }
    }

    pub fn to(&self, d: f64) -> Vec3 {
        self.ogn + self.dir*d
    }
}

#[derive(Debug)]
pub struct Base {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

impl Base {
    pub fn canonical() -> Base {
        Base { x: Vec3::new(1, 0, 0), y: Vec3::new(0, 1, 0), z: Vec3::new(0, 0, 1) }
    }
}

#[derive(Debug)]
pub struct HitInfo {
    pub dst: f64,
    pub pos: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub in_ray: Vec3,
}

pub fn unif_unit() -> f64 {
    rand::thread_rng().gen_range(0.0..=1.0)
}

pub fn unif_unit_centered() -> f64 {
    rand::thread_rng().gen_range(-0.5..=0.5)
}

pub fn rand_in_unit_disk() -> (f64, f64) {
    loop {
        let x = rand::thread_rng().gen_range(-1.0..=1.0);
        let y = rand::thread_rng().gen_range(-1.0..=1.0);
        if f64::sqrt(x*x + y*y) < 1. {
            return (x, y);
        }
    }
}
