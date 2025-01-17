use std::rc::Rc;

use super::material::Material;
use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RaytracerObjectType {
    Plane,
    Sphere,
    TestShape,
}

type ROT = RaytracerObjectType;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RaytracerObject {
    obj_id: usize,
    pub obj_type: ROT,
    pub origin: Point,
    pub transform: TransformationMatrix,
    pub material: Material,
}

impl RaytracerObject {
    fn new(obj_id: usize, obj_type: ROT, origin: Point) -> Self {
        let transform = TransformationMatrix::identity();
        let material = Material::new();

        Self {
            obj_id,
            obj_type,
            origin,
            transform,
            material,
        }
    }

    pub fn new_sphere(obj_id: usize, origin: Point) -> Self {
        Self::new(obj_id, ROT::Sphere, origin)
    }

    pub fn new_plane(obj_id: usize, origin: Point) -> Self {
        Self::new(obj_id, ROT::Plane, origin)
    }

    pub fn new_test_shape(obj_id: usize, origin: Point) -> Self {
        Self::new(obj_id, ROT::TestShape, origin)
    }

    pub fn id(&self) -> usize {
        self.obj_id
    }

    pub fn local_normal_at(&self, _point: Point) -> Vector {
        match &self.obj_type {
            ROT::Plane => Vector::new(0.0, 1.0, 0.0),
            _ => unimplemented!("Not yet implemented for {:?}", self.obj_type),
        }
    }

    pub fn hit_on_intersect(&self, ray: &Ray) -> Option<Rc<Intersection>> {
        match &self.obj_type {
            ROT::Sphere => hit_on_intersect_sphere(self, ray),
            _ => unimplemented!("Not yet implemented for {:?}", self.obj_type),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Time> {
        match &self.obj_type {
            ROT::Sphere => self.intersect_sphere(ray),
            ROT::Plane => self.intersect_plane(ray),
            _ => unimplemented!("Not yet implemented for {:?}", self.obj_type),
        }
    }

    pub fn local_intersect(&self, world_ray: &Ray) -> Vec<Time> {
        let local_ray = &world_ray.transform(&self.transform.inverse());

        match &self.obj_type {
            ROT::Plane => self.local_intersect_plane(local_ray),
            _ => unimplemented!("Not yet implemented for {:?}", self.obj_type),
        }
    }

    fn local_intersect_plane(&self, ray: &Ray) -> Vec<Time> {
        if ray.direction.y.abs() < EPSILON {
            return vec![];
        }

        let time = -ray.origin.y / ray.direction.y;

        vec![time]
    }

    fn intersect_sphere(&self, original_ray: &Ray) -> Vec<Time> {
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

    fn intersect_plane(&self, ray: &Ray) -> Vec<Time> {
        self.local_intersect_plane(ray)
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        // TODO: match on obj_type. Current sphere specific
        let sphere_transform_inverse = self.transform.inverse();

        let object_point = &sphere_transform_inverse * world_point;
        let object_normal = object_point - CENTER_ORIGIN;
        let world_normal = &sphere_transform_inverse.transposed() * object_normal;

        world_normal.norm()
    }

    pub fn material(&self) -> Material {
        self.material
    }
}

fn hit_on_intersect_sphere(sphere: &RaytracerObject, ray: &Ray) -> Option<Rc<Intersection>> {
    let mut potential_hit = Intersection {
        time: 0.0,
        object: Rc::new(sphere.clone()),
    };

    let mut any: bool = false;

    for t in sphere.intersect(ray).iter() {
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
