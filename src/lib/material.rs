use crate::lib::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Vec3,
    pub color_albedo: f32,
    pub specular_exponent: f32,
    pub specular_albedo: f32,
    pub reflexion_factor: f32,
    pub refraction_index: f32,
    pub transparency_factor: f32
}

impl Material {
    pub fn new(
        color: Vec3,
        color_albedo: f32,
        specular_exponent: f32,
        specular_albedo: f32,
        reflexion_factor: f32,
        refraction_index: f32,
        transparency_factor: f32
    ) -> Material {

        Material {
            color,
            color_albedo,
            specular_exponent,
            specular_albedo,
            reflexion_factor,
            refraction_index,
            transparency_factor
        }
    }
}

