use crate::lib::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Vec3,
    pub albedo: [f32; 4],
    pub refraction_index: f32,
    pub specular_exponent: f32
}

impl Material {
    pub fn new(color: Vec3, albedo: [f32; 4], refraction_index: f32, specular_exponent: f32) -> Material {
        Material {
            color,
            albedo,
            refraction_index,
            specular_exponent
        }
    }
}