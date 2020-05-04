use std::time::SystemTime;

pub mod lib;

use lib::{
    scene::Scene,
    vec3::Vec3,
    light::Light,
    material::Material,
    sphere::Sphere,
    camera::Camera
};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

#[cfg(debug_assertions)]
const RAYS_PER_PIXEL: usize = 1;

#[cfg(not(debug_assertions))]
const RAYS_PER_PIXEL: usize = 100;

const FOV: f32 = std::f32::consts::PI / 3.0;

const MAX_RECURTION: usize = 4;

#[allow(unused_variables)]
fn main() {    
    let ivory = Material::new(Vec3::new(0.4, 0.4, 0.3), 0.6, 50.0, 0.3, 0.1, 1.0, 0.0);
    let red_rubber = Material::new(Vec3::new(0.3, 0.1, 0.1), 0.75, 25.0, 0.2, 0.0, 1.0, 0.0);
    let blue = Material::new(Vec3::new(0.04, 0.1, 0.3), 0.9, 40.0, 0.1, 0.1, 1.0, 0.0);
    let mirror = Material::new(Vec3::new(1.0, 1.0, 1.), 0.0, 1425.0, 10.0, 0.8, 1.0, 0.0);
    let glass = Material::new(Vec3::new(0.6, 0.7, 0.8), 0.0, 125.0, 0.5, 0.1, 1.5, 0.8);


    let spheres = vec![
        Sphere::new(Vec3::new(0.0, -31.5, -10.0), 30.0, ivory.clone()),
        Sphere::new(Vec3::new(0.0, 0.45, -10.0), 2.0, mirror.clone()),
        Sphere::new(Vec3::new(-6.0, -0.11, -10.0), 2.0, red_rubber.clone()),
        Sphere::new(Vec3::new(6.0, -0.11, -10.0), 2.0, blue.clone())
    ]; 

    let lights = vec![
     Light::new(Vec3::new(-20.0, 20.0, 20.0), 1.5),
     Light::new(Vec3::new(30.0, 50.0, -25.0), 1.8),
     Light::new(Vec3::new(30.0, 20.0, 30.0) , 1.7)
    ];
    
    let mut camera = Camera::new(Vec3::new(0.0, 10.0, -10.0), WIDTH, HEIGHT, FOV);
    camera.set_rotation_x(std::f32::consts::PI * 0.5);
    //camera.set_rotation_z(std::f32::consts::PI);

    let mut scene = Scene::new(RAYS_PER_PIXEL, MAX_RECURTION, camera);

    for sphere in spheres {
        scene.push_object(sphere)
    }


    scene.push_light(lights[0].clone());
    scene.push_light(lights[1].clone());
    scene.push_light(lights[2].clone());
    


    let start = SystemTime::now();
    
    scene.render("output.png");

    if let Ok(time) = start.elapsed() {
        println!("Done in {:?}", time)
    } 
}