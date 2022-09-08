//!
//! This module provide a Camera type that allow to cast ray for each pixel of the render image.
//! For now two types of projections are provided : Orthographic and Perspective
//! 
//! Ortographic projection is yet to be implemented
//! 

extern crate linear_alg;
use std::thread;

use linear_alg::{Vec3, normalize, cross, rand_in_unit_cube};

use std::sync::{Arc, Mutex, mpsc};

use crate::surface::Surface;
use crate::utils::{Ray, Base, unif_unit, unif_unit_centered, rand_in_unit_disk};
use crate::world::World;


use minifb::{Key, Window, WindowOptions};


///
/// This Camera struct stores calera values set by the user.
/// 
pub struct Camera {
    viewport: Viewport,
    pub surface: Surface,
    max_ray_scattered: u32,
    max_ray_sent: u32,

    sender: Option<mpsc::Sender<()>>,
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

            sender: Option::None,
        }

    }

    pub fn capture_image(&mut self, world: World) {

        for j in 0..self.surface.height {
            //println!("{}", j as f32/self.surface.height as f32);

            for i in 0..self.surface.width {
                let mut color  = Vec3::zero();
                for _k in 0..self.max_ray_sent {
                    let (x, y) = (i as f64 / self.surface.width as f64,  1. - j as f64 / self.surface.height as f64);
                    let ray = self.viewport.get_ray(x+unif_unit_centered() / self.surface.width as f64, y+unif_unit_centered() / self.surface.height as f64);
                    color = color + world.get_color(&ray, self.max_ray_scattered);
                }

                color = color / self.max_ray_sent;
                self.surface.set_val(i, j, Self::to_tuple(color));
            }
            match &self.sender {
                Option::Some(sender) => sender.send(()).unwrap(),
                Option::None => (),
            };
        }

        
    }

    pub fn capture_image_snail(&mut self, world: World) {

        let mut seg_len = 1;
        let center = (self.surface.width/2, self.surface.height/2);

        let (i, j) = center;
        let mut color  = Vec3::zero();
        for _k in 0..self.max_ray_sent {
            let (x, y) = (i as f64 / self.surface.width as f64,  1. - j as f64 / self.surface.height as f64);
            let ray = self.viewport.get_ray(x+unif_unit_centered() / self.surface.width as f64, y+unif_unit_centered() / self.surface.height as f64);
            color = color + world.get_color(&ray, self.max_ray_scattered);
        }

        color = color / self.max_ray_sent;
        self.surface.set_val(i, j, Self::to_tuple(color));


        loop {
            let dir = [(0 as i32, 1 as i32), (1 as i32, 0 as i32), (0 as i32, -1 as i32), (-1 as i32, 0 as i32)];
            let offset = [(-1 as i32, -1 as i32), (-1 as i32, 1 as i32), (1 as i32, 1 as i32), (1 as i32, -1 as i32)];
            for side in 0..4 {
                for px in 0..=2*seg_len {
                    let (i, j) = (center.0+(offset[side].0*seg_len+dir[side].0*px) as u32, center.1+(offset[side].1*seg_len+dir[side].1*px) as u32);
                    if 0 > i || 0 > j || self.surface.width <= i || self.surface.height <= j {
                        continue
                    }
                    let mut color  = Vec3::zero();
                    for _k in 0..self.max_ray_sent {
                        let (x, y) = (i as f64 / self.surface.width as f64,  1. - j as f64 / self.surface.height as f64);
                        let ray = self.viewport.get_ray(x+unif_unit_centered() / self.surface.width as f64, y+unif_unit_centered() / self.surface.height as f64);
                        color = color + world.get_color(&ray, self.max_ray_scattered);
                    }
    
                    color = color / self.max_ray_sent;
                    self.surface.set_val(i, j, Self::to_tuple(color));
                       
                }
            }
            match &self.sender {
                Option::Some(sender) => sender.send(()).unwrap(),
                Option::None => (),
            };
            seg_len += 1;
            if seg_len > (u32::max(center.0, center.1)) as i32 {
                break;
            }
        }
        
    }

    pub fn display_progress(&mut self) {
        
        let buffer = Arc::clone(&self.surface.buffer);

        let WIDTH = self.surface.width as usize;
        let HEIGHT = self.surface.height as usize;

        let (tx, rx) = mpsc::channel();
        self.sender = Option::Some(tx);

        thread::spawn( move || {
            
            let mut window = Window::new(
                "Simple test window",
                WIDTH,
                HEIGHT,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

            window.limit_update_rate(Some(std::time::Duration::from_micros(1)));

            while window.is_open() && !window.is_key_down(Key::Escape) {

                //println!("update");
                window
                    .update_with_buffer(&(*buffer.lock().unwrap()), WIDTH, HEIGHT)
                    .unwrap();
                rx.recv().unwrap_or_default();
            }
        });
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
