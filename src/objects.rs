use std::rc::Rc;

use ndarray::*;

use super::material::Material;
use super::math::RaytracerMatrix;
use super::*;

pub trait RaytracerObject {
    fn id(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub id: usize,
    pub origin: Point,
    pub transform: TransformationMatrix,
    pub material: Material,
}

impl Sphere {
    pub fn new(id: usize, origin: Point) -> Self {
        let transform = Array::eye(4);
        let material = Material::new();

        Sphere {
            origin,
            id,
            transform,
            material,
        }
    }

    pub fn hit_on_intersect(&self, ray: &Ray) -> Option<Rc<Intersection>> {
        let mut potential_hit = Intersection {
            time: 0.0,
            object: Rc::new(self.clone()),
        };

        let mut any: bool = false;

        for t in self.intersect(ray).iter() {
            any = true;
            let time = *t;
            if time >= potential_hit.time && time >= 0.0 {
                potential_hit.time = time;
            }
        }

        if any {
            Some(Rc::new(potential_hit))
        } else {
            None
        }
    }
}

impl RaytracerObject for Sphere {
    fn id(&self) -> usize {
        self.id
    }
}

impl Interceptable for Sphere {
    fn intersect(&self, original_ray: &Ray) -> Vec<Time> {
        let inverse = self.transform.inverse();

        let ray = original_ray.transform(&inverse);

        let sphere_to_ray = ray.origin - self.origin;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        // return in increasing order
        if t1 < t2 {
            vec![t1, t2]
        } else {
            vec![t2, t1]
        }
    }

    fn normal_at(&self, world_point: Point) -> Vector {
        let sphere_transform_inverse = self.transform.inverse();

        let object_point = &sphere_transform_inverse * world_point;
        let object_normal = object_point - CENTER_ORIGIN;
        let world_normal = &sphere_transform_inverse.transposed() * object_normal;

        world_normal.norm()
    }

    fn material(&self) -> Material {
        self.material
    }
}
