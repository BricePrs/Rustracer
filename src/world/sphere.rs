
extern crate linear_alg;
use linear_alg::{Vec3, normalize, length_sq, dot};

use super::*;
use crate::utils;
use utils::HitInfo;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }

    pub fn empty() -> Sphere {
        Sphere {
            center: Vec3::zero(),
            radius: 0.,
        }
    }
}

impl Computable for Sphere {

    fn compute_intersection_dist(&self, ray: &utils::Ray) -> f64 {

        let oc = ray.ogn - self.center;
        let a = length_sq(ray.dir);
        let half_b = dot(oc, ray.dir);
        let c = length_sq(oc) - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
    
        if discriminant < 0. {
            return f64::INFINITY;
        }

        let mut  d = (-half_b - f64::sqrt(discriminant) ) / a;
        
        if d < 0. {
            d = (-half_b + f64::sqrt(discriminant) ) / a;
        }

        if d < 0. {
            return f64::INFINITY;
        }

        d
    
    }

    fn compute_intersection(&self, ray: &utils::Ray, dst: f64) -> HitInfo {

        let pos = ray.to(dst);
        let mut front_face = true;
        let mut normal = self.get_normal(pos);

        if dot(normal, ray.dir) > 0. {
            normal = normal * -1;
            front_face = false;
        }

         HitInfo {
             dst,
             pos,
             normal,
             front_face,
             in_ray: ray.dir,
         }
    }

}

impl Sphere {

    fn get_normal(&self, surface_point: Vec3) -> Vec3 {
        (surface_point-self.center)/self.radius
    }

}
