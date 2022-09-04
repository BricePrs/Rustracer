//!
//! This module provide a Camera type that allow to cast ray for each pixel of the render image.
//! For now two types of projections are provided : Orthographic and Perspective
//! 
//! Ortographic projection is yet to be implemented
//! 

extern crate linear_alg;
use linear_alg::{Vec3, normalize, cross, rand_in_unit_cube};

use crate::surface::Surface;
use crate::utils::{Ray, Base, unif_unit, unif_unit_centered, rand_in_unit_disk};
use crate::world::World;

///
/// This Camera struct stores calera values set by the user.
/// 
pub struct Camera {
    viewport: Viewport,
    pub surface: Surface,
    max_ray_scattered: u32,
    max_ray_sent: u32,
}

struct Viewport {
    width: f64,
    height: f64,
    aperture: f64,
    origin: Vec3,
    base: Base,
    lower_left_corner: Vec3,
}

impl Camera {

    pub fn new(position: Vec3, look_at: Vec3, vfov: f64, width_res: u32, aspect_ratio: f64, focal_length: f64, aperture: f64, max_ray_scattered: u32, max_ray_sent: u32) -> Camera {

        let height_res = (width_res as f64 * aspect_ratio) as u32;
        let surface = Surface::new(width_res, height_res, &String::from("render.ppm"));
        
        let viewport_height = 2. * f64::tan(vfov.to_radians() / 2.) * focal_length;
        let viewport_width = viewport_height / aspect_ratio;

        let viewport = Viewport::new(viewport_width, viewport_height, position, look_at, focal_length, aperture);

        Camera {
            viewport,
            surface,
            max_ray_scattered,
            max_ray_sent,
        }

    }

    pub fn capture_image(&mut self, world: World) {
        for j in 0..self.surface.height {
            println!("{}", j as f32/self.surface.height as f32);
            for i in 0..self.surface.width {
                let mut color  = Vec3::zero();
                for _k in 0..self.max_ray_sent {
                    let (x, y) = (i as f64 / self.surface.width as f64,  1. - j as f64 / self.surface.height as f64);
                    let ray = self.viewport.get_ray(x+unif_unit_centered() / self.surface.width as f64, y+unif_unit_centered() / self.surface.height as f64);
                    color = color + world.get_color(&ray, self.max_ray_scattered);
                }
                color = color / self.max_ray_sent;
                self.surface.set_val(i, j, Self::to_tuple(color))
            }
        }

    }

    
    pub fn export(&self) {
        self.surface.export();
    }
    
    fn to_tuple(color: Vec3) -> (u8, u8, u8) {
        (
            (Self::gamma_correction(color.x)*255.999) as u8,
            (Self::gamma_correction(color.y)*255.999) as u8,
            (Self::gamma_correction(color.z)*255.999) as u8,
        )
    }

    fn gamma_correction(val: f64) -> f64 {
        f64::sqrt(val)
    }

}


impl Viewport {
    pub fn new(width: f64, height: f64, origin: Vec3, look_at: Vec3, focal_length: f64, aperture: f64) -> Self {

        let mut base = Base::canonical();
        base.z = normalize(look_at-origin) * focal_length;
        base.x = normalize(cross(Vec3::new(0, 1, 0), base.z)) * width;
        base.y = normalize(cross(base.z, base.x)) * height;

        Viewport { 
            width, 
            height,
            aperture,
            origin,
            lower_left_corner: origin - base.x / 2. - base.y / 2. + base.z, 
            base,
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {

        let (rx, ry) = rand_in_unit_disk();
        let ray_ogn = self.origin + self.base.x * self.aperture / self.width * rx + self.base.y * self.aperture / self.height * ry;

        Ray {
            ogn: ray_ogn,
            dir: normalize(self.lower_left_corner + self.base.x * x + self.base.y * y - ray_ogn),
        }
    }
}
