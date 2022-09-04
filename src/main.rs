
extern crate linear_alg;
use linear_alg::{Vec3, length};

use basic_raytracer::camera::Camera;
use basic_raytracer::world::{World};

use basic_raytracer::world::sphere::Sphere; 
use basic_raytracer::material;

use basic_raytracer::world::Object;

fn main() {
    
    // Set up camera
    let mut camera = Camera::new(
        Vec3::new(-7.0, 0.5, 2.0),
        Vec3::new(0, 0., 0.),
        30.,
        1500,
        0.6,
        length(Vec3::new(-7.0, 0.5, 2.0))-1.5,
        0.13456789,
        150,
        250
    );
    
    camera.surface.save_to(&String::from("render.ppm"));

    // Set up World
    let skybox_color = Vec3::new(0.6, 0.7, 1.0);
    let mut world = World::new(&skybox_color);
    
    
    let lamb_sphere = Object::new(
        Sphere::new(Vec3::new(0, 0, 0), 0.5),
        material::Lambertian::new(Vec3::new(0.95, 0.95, 0.95)),
    );

    let red_sphere = Object::new(
        Sphere::new(Vec3::new(-1.3, 0, 0), 0.5),
        material::Lambertian::new(Vec3::new(0.95, 0.3, 0.2)),
    );

    let glass_sphere = Object::new(
        Sphere::new(Vec3::new(-2.5, 0.0, 0.), 0.5),
        material::Dielectric::new(1.5),
    );

    let ground_sphere = Object::new(
        Sphere::new(Vec3::new(0, -500, 0), 499.5),
        material::Lambertian::new(Vec3::new(0.45, 0.95, 0.35)),
    );

    let mirror_sphere = Object::new(
        Sphere::new(Vec3::new(2., 0.5, 0), 1.),
        material::Metal::new(Vec3::new(0.9, 0.9, 0.9), 0.)
    );

    world.add(lamb_sphere);
    world.add(ground_sphere);
    world.add(red_sphere);
    world.add(glass_sphere);
    world.add(mirror_sphere);

    // Render scene
    camera.capture_image(world);

    // Write to file
    camera.export()
}
