use crate::scene::{Intersectable, Intersection, Scene};
use glam::{Vec3};

pub struct Renderer {
    buffer: Vec<u32>,
    size: (u32, u32),
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            buffer: Vec::new(),
            size: (0, 0),
        }
    }
    pub fn render(&mut self, scene: &Scene, width: u16, height: u16) -> &[u32] {
        // Update size
        if self.size.0 != width as u32 || self.size.1 != height as u32 {
            self.size = (width as u32, height as u32);
            self.buffer.resize(width as usize * height as usize, 0);
        }

        // Clear screen
        let pixel_count = width as u32 * height as u32;
        for index in 0..pixel_count {
            // Compute pixel position
            let px = index % self.size.0;
            let py = index / self.size.0;

            // Generate ray
            let ray = scene
                .camera
                .generate_ray(px, py, width as u32, height as u32);

            // Compute intersection
            if let Intersection::Hit(info) = scene.intersect(&ray) {
                const LIGHT_SOURCE: Vec3 = Vec3::new(-5.0, 5.0, -5.0);
                let l = Vec3::normalize(LIGHT_SOURCE - info.position);
                let k = Vec3::dot(info.normal, l).max(0.0);

                let red = (k * 255.0) as u32;
                let green = (k * 255.0) as u32;
                let blue = (k * 255.0) as u32;
                self.buffer[index as usize] = (blue | (green << 8) | (red << 16)) as u32;
            } else {
                self.buffer[index as usize] = 0;
            }
        }

        // Return buffer as read-only slice
        self.buffer.as_slice()
    }
}
