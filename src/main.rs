use std::time::SystemTime;

pub mod lib;

use lib::{
    scene::Scene,
    vec3::Vec3,
    light::Light,
    material::Material,
    sphere::Sphere
};

#[cfg(debug_assertions)] 
const WIDTH: usize = 600;
#[cfg(debug_assertions)] 
const HEIGHT: usize = 300;
#[cfg(debug_assertions)]
const RAYS_PER_PIXEL: usize = 1;

#[cfg(not(debug_assertions))]
const WIDTH: usize = 1920;
#[cfg(not(debug_assertions))]
const HEIGHT: usize = 1080;
#[cfg(not(debug_assertions))]
const RAYS_PER_PIXEL: usize = 100;

const FOV: f64 = std::f64::consts::PI / 3.0;

const ORIGIN: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0
};

const MAX_RECURTION: usize = 4;

fn main() {
    let glass = Material::new(Vec3::new(0.6, 0.7, 0.8), [0.0, 0.5, 0.1, 0.8], 1.5, 125.0);
    let mirror = Material::new(Vec3::new(1.0, 1.0, 1.), [0.0, 10.0, 0.8, 0.0], 1.0, 1425.0);
    let ivory = Material::new(Vec3::new(0.4, 0.4, 0.3), [0.6, 0.3, 0.1, 0.0], 1.0, 50.0);
    let red_rubber = Material::new(Vec3::new(0.3, 0.1, 0.1), [0.9, 0.1, 0.1, 0.0], 1.0, 10.0);
    let blue = Material::new(Vec3::new(0.04, 0.1, 0.3), [0.9, 0.1, 0.2, 0.0], 1.0, 40.0);

    let spheres = vec![
        Sphere::new(Vec3::new(-3.0, 0.0 , -16.0) , 2.0, ivory.clone()),
        Sphere::new(Vec3::new(-1.0, -1.5, -12.0) , 2.0, ivory.clone()),
        Sphere::new(Vec3::new(1.5 , -0.5, -18.0) , 3.0, red_rubber.clone()),
        Sphere::new(Vec3::new(7.0 , 5.0 , -18.0) , 4.0, mirror.clone())
    ];

    let lights = vec![
        Light::new(Vec3::new(-20.0, 20.0, 20.0), 1.5),
        Light::new(Vec3::new(30.0, 50.0, -25.0), 1.8),
        Light::new(Vec3::new(30.0, 20.0, 30.0) , 1.7)
    ];

    let mut scene = Scene::new(WIDTH, HEIGHT, ORIGIN, RAYS_PER_PIXEL, MAX_RECURTION, FOV);

    scene.push_object(spheres[0].clone());
    scene.push_object(spheres[1].clone());
    scene.push_object(spheres[2].clone());
    scene.push_object(spheres[3].clone());

    scene.push_light(lights[0].clone());
    scene.push_light(lights[1].clone());
    scene.push_light(lights[2].clone());
    

    let start = SystemTime::now();
    
    scene.render_as_png("output.png");
    //scene.render_as_ppm("output.ppm");

    if let Ok(time) = start.elapsed() {
        println!("Done in {:?}", time)
    } 
}