//!
//! Simple implementation of a dielectric material
//! 

extern crate linear_alg;
use linear_alg::{Vec3, dot, reflect, length_sq, normalize, length};

use super::Material;

use crate::utils::{HitInfo, Ray, self};

pub struct Dielectric {
    ind: f64,
    ref_coef: f64,
}

impl Material for Dielectric {

    fn scatter_ray(&self, hit_info: HitInfo) -> Ray {
        let cos_th = dot(normalize(hit_info.in_ray*(-1)), hit_info.normal);
        assert!(cos_th <= 1.);

        match self.refract(cos_th) {
            true => self.get_refracted_ray(hit_info, cos_th),
            false => self.get_reflected_ray(hit_info),
        }

    }

    fn get_attenuation(&self) -> Vec3 {
        Vec3::new(1, 1, 1)
    }

}

impl Dielectric {

    pub fn new(n: f64) -> Self {
        let r0 = (n-1.)/(n+1.);
        Dielectric {
            ind: n,
            ref_coef: r0*r0,
        }
    }

    fn refract(&self, cos_th: f64) -> bool {
        let threshold = self.ref_coef+(1.-self.ref_coef) * f64::powf(1.-cos_th, 5.);
        if utils::unif_unit() > threshold {
            return true;
        }
        false
    }

    fn get_refracted_ray(&self, hit_info: HitInfo, cos_th: f64) -> Ray {
        let mut n_ratio = self.ind;
        if hit_info.front_face {
            n_ratio = 1./n_ratio;
        }

        let sin_i1 = f64::sqrt(1.-cos_th*cos_th);
        let sin_i2 = n_ratio*sin_i1;

        if f64::abs(sin_i2) > 1. {
            return self.get_reflected_ray(hit_info);
        }

        let r_perp =  (hit_info.normal*cos_th + hit_info.in_ray) * n_ratio;
        let new_ray = Ray::new(
            hit_info.pos - hit_info.normal * 0.00001,
             r_perp - hit_info.normal*f64::sqrt(f64::abs(1.-length_sq(r_perp))),
        );
        new_ray
    }

    fn get_reflected_ray(&self, hit_info: HitInfo) -> Ray {
        Ray::new(
            hit_info.pos + hit_info.normal * 0.0001,
            reflect(hit_info.in_ray*(-1), hit_info.normal)
            )
    }

}
