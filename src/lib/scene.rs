use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use png;

use crate::lib::{
    vec3::Vec3,
    ray::Ray,
    light::Light,
    hitable::{Hitable, HitInfos},
    camera::Camera
};

pub struct Scene {
    rays_per_pixel: usize,
    max_recurtion: usize,
    objects: Vec<Box<dyn Hitable + Sync>>,
    lights: Vec<Light>,
    camera: Camera
}

impl Scene {
    pub fn new(rays_per_pixel: usize, max_recurtion: usize, camera: Camera) -> Scene {
        Scene {
            rays_per_pixel,
            max_recurtion,
            objects: vec![],
            lights: vec![],
            camera
        }
    }

    pub fn push_object<T: Hitable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object))
    }

    pub fn push_light(&mut self, light: Light) {
        self.lights.push(light)
    }

    pub fn render(&self, path: &str) {        
        let mut data = vec![];
        for j in 0..self.camera.height {
            if (self.camera.height - j) % 5 == 0 {
                println!("{} rows remaining", self.camera.height - j);
            }
            
            for i in 0..self.camera.width {
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                
                for _ in 0..self.rays_per_pixel {
                    let ray = self.camera.compute_camera(i, j);
                    color = color + self.cast_ray(&ray, 0);
                }
                let c = self.get_color(color);
                
                data.push(c[0]);
                data.push(c[1]);
                data.push(c[2]);    
            } 
        }
        self.save_as_png(path, data);
    }

    fn cast_ray(&self, ray: &Ray, recurtion: usize) -> Vec3 { 
        let mut infos = vec![];
        let _: Vec<_> = self.objects.iter().map(|sphere| infos.push(sphere.ray_intersect(&ray))).collect();
        let closest = HitInfos::get_closest(infos);
        
        if recurtion > self.max_recurtion || closest.is_none() {
            return Vec3::new(0.2, 0.7, 0.9)
        }

        let closest = closest.unwrap();
        let material = closest.material;

        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;
    
        for light in &self.lights {
            let light_dir = (&light.position - &closest.hit_point).normalize();
            let light_dist = (&light.position - &closest.hit_point).lenght();
            
            let shadow_origin = if Vec3::dot(&light_dir, &closest.normal) > 0.0 {
                &closest.hit_point + &closest.normal * 0.001
            } else {
                &closest.hit_point - &closest.normal * 0.001
            };
    
            let shadow_ray = Ray::new(shadow_origin.clone(), light_dir.clone());
            let mut shadows_infos = vec![];
            let _: Vec<_> = self.objects.iter().map(|sphere| shadows_infos.push(sphere.ray_intersect(&shadow_ray))).collect();
    
            let closest_shadow = HitInfos::get_closest(shadows_infos);
            if closest_shadow.is_some() && (closest_shadow.unwrap().hit_point - shadow_origin).lenght() < light_dist {
                continue;
            }
    
            
            let dot = Vec3::dot(&light_dir, &closest.normal);
            diffuse_light_intensity += light.intensity * dot.max(0.0);
    
            
            let specular_reflect_dir = Self::reflect(&light_dir, &closest.normal);
            specular_light_intensity += (Vec3::dot(&specular_reflect_dir, &ray.direction)).max(0.0).powf(material.specular_exponent)  * light.intensity;   
        }
        
        let reflect_dir = Self::reflect(&ray.direction, &closest.normal);
    
        let reflect_origin = if Vec3::dot(&reflect_dir, &closest.normal) > 0.0 {
            &closest.hit_point + &closest.normal * 0.001
        } else {
            &closest.hit_point - &closest.normal * 0.001
        };
    
        let reflect_ray = Ray::new(reflect_origin, reflect_dir);
        let reflect_color = self.cast_ray(&reflect_ray, recurtion + 1);
    

        let refract_dir = Self::refract(&ray.direction, &closest.normal, material.refraction_index).normalize();
    
        let refract_origin = if Vec3::dot(&refract_dir, &closest.normal) > 0.0 {
            &closest.hit_point + &closest.normal * 0.001
        } else {
            &closest.hit_point - &closest.normal * 0.001
        };
    
        let refract_ray = Ray::new(refract_origin, refract_dir);
        let refract_color = self.cast_ray(&refract_ray, recurtion + 1);
        

        let mut color = material.color;

        color = color * diffuse_light_intensity * material.color_albedo;

        color = color + Vec3::new(1.0, 1.0, 1.0) * specular_light_intensity * material.specular_albedo;

        color = color + reflect_color * material.reflexion_factor;

        color = color + refract_color * material.transparency_factor;

        color
    }

    fn reflect(impident: &Vec3, normal: &Vec3) -> Vec3 {
        impident - normal * 2.0 * Vec3::dot(impident, normal)
    }

    fn refract(impident: &Vec3, normal: &Vec3, refraction_index: f32) -> Vec3 {
        let mut cosi = -Vec3::dot(&impident, &normal).min(1.0).max(-1.0);
        let mut etai = 1.0;
        let mut etat = refraction_index;
        let mut n = normal.clone();
        if cosi < 0.0 {
            cosi = -cosi;
            n = -normal;
            std::mem::swap(&mut etai, &mut etat);
        }
        let eta = etai / etat;
        let k = 1.0 - etat * etat * (1.0 - cosi * cosi);
        if k > 0.0 {
            return impident * eta + n * (eta * cosi - k.sqrt())
        }
        Vec3::new(0.0, 0.0, 0.0)
    }
    
    fn get_color(&self, color: Vec3) -> [u8; 3] {
        let max = color.x.max(color.y.max(color.z));
        
        let mut return_value = color;
        if max > self.rays_per_pixel as f32 {
            return_value = return_value * (self.rays_per_pixel as f32 / max) 
        }

        let scale = 1.0 / self.rays_per_pixel as f32;

        let r = (((return_value.x.min(self.rays_per_pixel as f32)).max(0.0) * 255.0) * scale) as u8;
        let g = (((return_value.y.min(self.rays_per_pixel as f32)).max(0.0) * 255.0) * scale) as u8;
        let b = (((return_value.z.min(self.rays_per_pixel as f32)).max(0.0) * 255.0) * scale) as u8;

        [r, g, b]
    }

    fn save_as_png(&self, path: &str, data: Vec<u8>) {
        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.camera.width as u32, self.camera.height as u32); 
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&data).unwrap();
    }
}