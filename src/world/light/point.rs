
extern crate linear_alg;
use linear_alg::Vec3;

pub struct PointLight {
    pos: Vec3,
    color: Vec3,
}

impl PointLight {
    
    pub fn new(pos: Vec3, color: Vec3) -> PointLight {
        PointLight {
            pos,
            color,
        }
    }

}
