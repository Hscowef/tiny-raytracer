use rand::Rng;

use crate::lib::vec3::Vec3;
use crate::lib::ray::Ray;

pub struct Camera {
    pub position: Vec3,
    pub width: usize,
    pub height: usize,
    pub fov: f32,
    image_aspect_ration: f32,
    half_fov_tan: f32,
    rotation_matrix: [[f32; 3]; 3]
}

impl Camera {
    pub fn new(position: Vec3, width: usize, height: usize, fov: f32) -> Camera {
        let image_aspect_ration = width as f32 / height as f32;
        let half_fov_tan = (fov / 2.0).tan();

        Camera {
            position,
            width,
            height,
            fov,
            image_aspect_ration,
            half_fov_tan,
            rotation_matrix: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0 ,0.0],
                [0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn set_rotation_x(&mut self, angle: f32) {
        let c = angle.cos();
        let s = angle.sin();
        self.rotation_matrix = Self::multiply_matrices(
            [
            [1.0, 0.0, 0.0],
            [0.0, c  , s  ],
            [0.0, -s , c  ]
            ],
            self.rotation_matrix
        );
    }

    pub fn set_rotation_y(&mut self, angle: f32) {
        let c = angle.cos();
        let s = angle.sin();
        self.rotation_matrix = Self::multiply_matrices(
            [
            [c  , 0.0, -s ],
            [0.0, 1.0, 0.0],
            [s  , 0.0, c  ]
            ],
            self.rotation_matrix
        );
    }

    pub fn set_rotation_z(&mut self, angle: f32) {
        let c = angle.cos();
        let s = angle.sin();
        self.rotation_matrix = Self::multiply_matrices(
            [
            [c  , -s  , 0.0],
            [s  , c   , 0.0],
            [0.0, 0.0 , 1.0]
            ],
            self.rotation_matrix
        );
    }

    pub fn compute_camera(&self, i: usize, j: usize) -> Ray {

        let mut rng = rand::thread_rng();
        let rand_x: f32 = rng.gen(); 
        let rand_y: f32 = rng.gen();
        
        let norm_i = (i as f32 + rand_x) / self.width as f32;
        let norm_j = (j as f32 + rand_y) / self.height as f32;

        let pixel_camera_x = (2.0 * norm_i - 1.0) * self.image_aspect_ration * self.half_fov_tan;
        let pixel_camera_y = (1.0 - 2.0 * norm_j) * self.half_fov_tan;

        let ray_p_world = Vec3::new(pixel_camera_x, pixel_camera_y, - 1.0);
        let mut ray_direction = (ray_p_world - Vec3::new(0.0, 0.0, 0.0)).normalize();
        
        ray_direction = self.camera_to_world(&ray_direction);
        

        Ray::new(self.position.clone(), ray_direction)
    }


    fn camera_to_world(&self, other: &Vec3) -> Vec3 {
        let matrix = self.rotation_matrix;
        Vec3::new(
            matrix[0][0] * other.x + matrix[0][1] * other.y + matrix[0][2] * other.z,
            matrix[1][0] * other.x + matrix[1][1] * other.y + matrix[1][2] * other.z,
            matrix[2][0] * other.x + matrix[2][1] * other.y + matrix[2][2] * other.z
        )
    }

    fn multiply_matrices(a: [[f32; 3]; 3], b: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
        let a1 = Vec3::new(a[0][0], a[0][1], a[0][2]);
        let a2 = Vec3::new(a[1][0], a[1][1], a[1][2]);
        let a3 = Vec3::new(a[2][0], a[2][1], a[2][2]);

        let b1 = Vec3::new(b[0][0], b[1][0], b[2][0]);
        let b2 = Vec3::new(b[0][1], b[1][1], b[2][1]);
        let b3 = Vec3::new(b[0][2], b[1][2], b[2][2]);

        [
            [Vec3::dot(&a1, &b1), Vec3::dot(&a1, &b2), Vec3::dot(&a1, &b3),],
            [Vec3::dot(&a2, &b1), Vec3::dot(&a2, &b2), Vec3::dot(&a2, &b3),],
            [Vec3::dot(&a3, &b1), Vec3::dot(&a3, &b2), Vec3::dot(&a3, &b3),],
        ]
    }
}
