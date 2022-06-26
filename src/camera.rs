use glam::{Mat4, Vec2, Vec3, Vec4};

use crate::scene::Ray;

pub struct Camera {
    pub position: Vec3,
    pub inverse_vp_matrix: Mat4,
}

impl Camera {
    pub fn new(position: Vec3, lookat: Vec3, aspect_ratio: f32) -> Camera {
        let direction = lookat - position;
        let left = direction.cross(Vec3::Y);
        let view_matrix = Mat4::look_at_rh(position, lookat, left.cross(direction).normalize());
        let projection_matrix =
            Mat4::perspective_rh(f32::to_radians(90.0), aspect_ratio, 0.01, 100.0);
        Camera {
            position,
            inverse_vp_matrix: (projection_matrix * view_matrix).inverse(),
        }
    }

    pub fn generate_ray(&self, px: u32, py: u32, width: u32, height: u32) -> Ray {
        // Create the ray
        let pos = Vec2 {
            x: px as f32,
            y: py as f32,
        };
        let near = Vec4::new(
            (pos.x - (width / 2) as f32) / width as f32,
            -1.0 * (pos.y - (height / 2) as f32) / height as f32,
            -1.0,
            1.0,
        );
        let far = Vec4::new(
            (pos.x - (width / 2) as f32) / width as f32,
            -1.0 * (pos.y - (height / 2) as f32) / height as f32,
            1.0,
            1.0,
        );
        let mut near_result = self.inverse_vp_matrix * near;
        let mut far_result = self.inverse_vp_matrix * far;
        near_result /= near_result.w;
        far_result /= far_result.w;
        let dir: Vec3 = Vec3::from_slice(&(far_result - near_result).normalize().to_array()[0..3]);

        Ray {
            origin: self.position,
            direction: dir,
        }
    }
}
