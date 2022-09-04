//!
//! Module that stored all a scene's data into a World type.
//! 

extern crate linear_alg;
use linear_alg::{Vec3, normalize, length, lerp};

use crate::utils::{self, Ray};

pub mod sphere;
use sphere::Sphere;

mod light;
use light::{PointLight, SpotLight, DirectionalLight};

use crate::material::Material;

use utils::HitInfo;

trait Computable {

    fn compute_intersection_dist(&self, ray: &utils::Ray) -> f64;
    fn compute_intersection(&self, ray: &utils::Ray, dst: f64) -> HitInfo;

}

pub struct Object {
    primitive: Sphere,
    material: Box<dyn Material>,
}

enum Light {
    Point(PointLight),
    Directional(DirectionalLight),
    Spot(SpotLight),
}

pub struct World {
    objects: Vec<Object>,
    lights: Vec<Light>,
    skybox_color: Vec3,
}

impl World {

    pub fn get_color(&self, ray: &Ray, max_ray_scattered: u32) -> Vec3 {

        let mut d_min = f64::INFINITY;
        let mut closest_object = Option::None;

        for object in self.objects.iter() {

            let dst = object.compute_intersection_dist(&ray);

            if d_min > dst {
                d_min = dst;
                closest_object = Option::Some(object);
            }

        }

        match closest_object {
            Option::None => {
                lerp(Vec3::new(0.8, 0.8, 1.), self.skybox_color, ray.dir.y)
            },
            Option::Some(object) => {
                let hit_info = object.compute_intersection(ray, d_min);
                //println!("len = {}", length(hit_info.in_ray));
                if max_ray_scattered == 0 {
                    return Vec3::new(0, 0, 0); 
                }
                let scattered = object.material.scatter_ray(hit_info);
                let attenuation = object.material.get_attenuation();
                return attenuation * self.get_color(&scattered, max_ray_scattered - 1);
            },
        }

    }

    pub fn new(skybox_color: &Vec3) -> World {
        World {
            objects: vec![],
            lights: vec![],
            skybox_color: skybox_color.clone(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }

}

impl Object {
    pub fn new(sphere: Sphere, material: impl Material + 'static) -> Object {
        Object {
            primitive: sphere,
            material: Box::new(material),
        }
    }

    pub fn compute_intersection_dist(&self, ray: &Ray) -> f64 {
        self.primitive.compute_intersection_dist(&ray)
    }

    pub fn compute_intersection(&self, ray: &Ray, dst: f64) -> HitInfo {
        self.primitive.compute_intersection(&ray, dst)
    }
}
