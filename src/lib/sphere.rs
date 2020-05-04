use crate::lib::vec3::Vec3;
use crate::lib::ray::Ray;
use crate::lib::hitable::{Hitable, HitInfos};
use crate::lib::material::Material;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hitable for Sphere {
    fn ray_intersect(&self, ray: &Ray) -> Option<HitInfos> {
        let v = &self.center - &ray.origin;

        let p = Vec3::dot(&v, &ray.direction);
        if p <= 0.0 {
            // The sphere is behind the ray
            return None;
        } 

        // Projection of center on the ray
        let pc = &ray.direction * (Vec3::dot(&ray.direction, &v) / ray.direction.lenght());

        // Distance between pc and the center 
        let d = (&self.center - &(&ray.origin + &pc)).lenght(); 

        if d < self.radius {
            // Distance between the hit point and pc
            let a = (self.radius * self.radius - d * d).sqrt();
            let hit_distance = p - a;
            
            let exit_distance = p + a;
            
            if hit_distance < 0.0 {
                return None
            }

            let hit_point = &ray.origin + &(&ray.direction * hit_distance); 
            let exit_point = &ray.origin + &(&ray.direction * exit_distance); 
            let normal = (&hit_point - &self.center).normalize();
            
            return Some (HitInfos {
                hit_point,
                exit_point,
                hit_distance,
                normal,
                material: self.material.clone()
            });
        }        
        None
    }
}