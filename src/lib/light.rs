use crate::lib::vec3::Vec3;

#[derive(Clone)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f32
}

impl Light {
    pub fn new(position: Vec3, intensity: f32) -> Light {
        Light {
            position,
            intensity
        }
    }
}

