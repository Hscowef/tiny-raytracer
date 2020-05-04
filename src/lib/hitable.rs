use crate::lib::vec3::Vec3;
use crate::lib::ray::Ray;
use crate::lib::material::Material;

pub trait Hitable: Sync + Send {
    fn ray_intersect(&self, ray: &Ray) -> Option<HitInfos>;
}

#[derive(Debug, Clone)]
pub struct HitInfos {
    pub hit_point: Vec3,
    pub exit_point: Vec3,
    pub hit_distance: f32,
    pub normal: Vec3,
    pub material: Material
}

impl HitInfos {
    pub fn get_closest(infos: Vec<Option<HitInfos>>) -> Option<HitInfos> {
        let mut min = None;
        for info in infos {
            if (&min).is_none() {
                min = info;
                continue;
            }
            if let Some(inf) = info {
                if inf.hit_distance <= min.clone().unwrap().hit_distance {
                    min = Some(inf)
                }
            }
        }
        min
    }
}