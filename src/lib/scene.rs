use std::fs;
// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;


use png;
use rand::Rng;


use crate::lib::{
    vec3::Vec3,
    ray::Ray,
    light::Light,
    hitable::{Hitable, HitInfos},
};

pub struct Scene {
    width: usize,
    height: usize,
    rays_origin: Vec3,
    rays_per_pixel: usize,
    max_recurtion: usize,
    fov: f64,
    objects: Vec<Box<dyn Hitable + Sync>>,
    lights: Vec<Light>
}

impl Scene {
    pub fn new(width: usize, height: usize, rays_origin: Vec3, rays_per_pixel: usize, max_recurtion: usize, fov: f64) -> Scene {
        Scene {
            width,
            height,
            rays_origin,
            rays_per_pixel,
            max_recurtion,
            fov,
            objects: vec![],
            lights: vec![]
        }
    }

    pub fn push_object<T: Hitable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object))
    }

    pub fn push_light(&mut self, light: Light) {
        self.lights.push(light)
    }

    pub fn render_as_png(&self, path: &str) {
        let data = self.render();

        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32); 
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&data).unwrap();
    }

    pub fn render_as_ppm(&self, path: &str) {
        let data = self.render();
        let mut buffer = format!("P3\n{} {}\n255\n", self.width, self.height);
        let _: Vec<_> = data.chunks(3).map(|chunk| buffer.push_str(format!(
                "{} {} {}\n", 
                chunk[0], 
                chunk[1], 
                chunk[2]
            ).as_str())).collect();

        fs::write(path, buffer).expect("Unable to find file location");
    }

    fn render(&self) -> Vec<u8> {
        let mut data = vec![0; 0];
        
        let half_fov_tan = (self.fov / 2.0).tan();
        for j in 0..self.height {
            if (self.height - j) % 5 == 0 {
                println!("{} rows remaining", self.height - j);
            }
            
            for i in 0..self.width {
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                
                for _ in 0..self.rays_per_pixel {
                    let mut rng = rand::thread_rng();
                    let rand_x: f32 = rng.gen(); 
                    let rand_y: f32 = rng.gen(); 

                    let x = (2.0 * (i as f32 + rand_x)  as f32/ self.width as f32 - 1.0) * half_fov_tan as f32 * self.width as f32 / self.height as f32; 
                    let y = -(2.0 * (j as f32 + rand_y) as f32 / self.height as f32 - 1.0) * half_fov_tan as f32; 

                    let dir = Vec3::new(x, y, -1.0).normalize();
                    let ray = Ray::new(self.rays_origin.clone(), dir);

                    color = color + self.cast_ray(&ray, 0);
                }
                let c = self.get_color(color);
                
                data.push(c[0]);
                data.push(c[1]);
                data.push(c[2]);    
            } 
        }
        data
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
            
            // Shadows calculs
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
            
            // Diffuse lightning calculs
            let dot = Vec3::dot(&light_dir, &closest.normal);
            diffuse_light_intensity += light.intensity * dot.max(0.0);
    
            // Specular lightning calculs
            let specular_reflect_dir = Self::reflect(&light_dir, &closest.normal);
            specular_light_intensity += (Vec3::dot(&specular_reflect_dir, &ray.direction)).max(0.0).powf(material.specular_exponent) * light.intensity;   
        }
        
        // Reflection calculs
        let reflect_dir = Self::reflect(&ray.direction, &closest.normal);
    
        let reflect_origin = if Vec3::dot(&reflect_dir, &closest.normal) > 0.0 {
            &closest.hit_point + &closest.normal * 0.001
        } else {
            &closest.hit_point - &closest.normal * 0.001
        };
    
        let reflect_ray = Ray::new(reflect_origin, reflect_dir);
        let reflect_color = self.cast_ray(&reflect_ray, recurtion + 1);
    

        // Refraction calculs
        let refract_dir = Self::refract(&ray.direction, &closest.normal, material.refraction_index).normalize();
    
        let refract_origin = if Vec3::dot(&refract_dir, &closest.normal) > 0.0 {
            &closest.hit_point + &closest.normal * 0.001
        } else {
            &closest.hit_point - &closest.normal * 0.001
        };
    
        let refract_ray = Ray::new(refract_origin, refract_dir);
        let refract_color = self. cast_ray(&refract_ray, recurtion + 1);
        

        //let mut returned_color = material.color;


        material.color * diffuse_light_intensity * material.albedo[0] + Vec3::new(1.0, 1.0, 1.0) * specular_light_intensity * material.albedo[1] + reflect_color * material.albedo[2] + refract_color * material.albedo[3]
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
}