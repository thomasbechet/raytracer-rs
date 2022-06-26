use glam::Vec3;

use crate::camera::Camera;

struct Sphere {
    position: Vec3,
    radius: f32,
}

struct Plane {
    position: Vec3,
    normal: Vec3,
}

enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

pub struct HitInfo {
    pub position: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}

pub enum Intersection {
    Hit(HitInfo),
    Miss,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Intersection;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let l = self.position - ray.origin;
        let tca = l.dot(ray.direction);
        if tca < 0.0 {
            Intersection::Miss
        } else {
            let d2 = Vec3::dot(l, l) - tca * tca;
            if d2 > self.radius {
                Intersection::Miss
            } else {
                let thc = (self.radius - d2).sqrt();
                let t0 = tca - thc;
                let t1 = tca + thc;

                let position = ray.origin + ray.direction * t0;

                Intersection::Hit(HitInfo {
                    position,
                    normal: Vec3::normalize(position - self.position),
                    distance: t0,
                })
            }
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let denom = Vec3::dot(self.normal, ray.direction);
        if denom.abs() > 0.0001 {
            let t = Vec3::dot(self.position - ray.origin, self.normal) / denom;
            if t >= 0.0 {
                return Intersection::Hit(HitInfo {
                    position: self.position, // TODO: to complete
                    normal: self.normal,
                    distance: 10000.0,
                });
            }
        }
        Intersection::Miss
    }
}

pub struct Scene {
    pub camera: Camera,
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        let mut scene = Scene {
            camera: Camera::new(
                Vec3 {
                    x: 0.0,
                    y: 2.0,
                    z: -4.0,
                },
                Vec3::ZERO,
                16.0 / 9.0,
            ),
            objects: Vec::new(),
        };
        // Fill the scene
        scene.objects.push(Object::Sphere(Sphere {
            position: Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.5,
            },
            radius: 1.0,
        }));
        scene.objects.push(Object::Sphere(Sphere {
            position: Vec3 {
                x: 2.0,
                y: 0.0,
                z: 0.5,
            },
            radius: 1.0,
        }));
        // Setup the camera

        scene
    }
}

impl Intersectable for Scene {
    fn intersect(&self, ray: &Ray) -> Intersection {
        for object in &self.objects {
            let intersection = match object {
                Object::Plane(plane) => plane.intersect(ray),
                Object::Sphere(sphere) => sphere.intersect(ray),
            };
            if let Intersection::Hit(info) = intersection {
                return Intersection::Hit(info);
            }
        }
        Intersection::Miss
    }
}
