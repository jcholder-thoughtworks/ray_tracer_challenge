use std::ops;
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::thread;

use self::canvas::Canvas;
use self::color::{Color, BLACK, WHITE};
use self::light::Light;
use self::math::transforms::{scaling, TransformationMatrix};
use self::objects::RaytracerObject;

pub mod canvas;
pub mod color;
pub mod light;
pub mod material;
pub mod math;
pub mod objects;
pub mod physics;

pub const EPSILON: f32 = 0.00001;
pub const EPSILON_DIGITS: i32 = 5;

pub const CENTER_ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
pub const STATIONARY: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub type Time = f32;
pub type Intersections = Vec<Rc<Intersection>>;

pub fn round(v: f32) -> f32 {
    let factor = (10.0 as f32).powi(EPSILON_DIGITS);
    (v * factor).round() / factor
}

#[derive(Clone, Debug)]
pub struct RaytracerWorld {
    next_id: usize,
    pub light: Option<Light>,
    objs: Vec<RaytracerObject>,
}

impl RaytracerWorld {
    pub fn new() -> Self {
        RaytracerWorld {
            next_id: 0,
            light: None,
            objs: vec![],
        }
    }

    // TODO: Should probably return an Rc<Sphere>
    pub fn new_sphere(&mut self, origin: Point) -> RaytracerObject {
        let id = self.next_id;
        self.next_id += 1;

        RaytracerObject::new_sphere(id, origin)
    }

    pub fn objects(&self) -> Vec<Rc<RaytracerObject>> {
        // TODO: Return an iterator of objects, instead. No Vec/Rc necessary
        self.objs.iter().map({ |o| Rc::new(o.clone()) }).collect()
    }

    pub fn add_object(&mut self, obj: RaytracerObject) {
        self.objs.push(obj);
    }

    pub fn get_object_mut(&mut self, index: usize) -> &mut RaytracerObject {
        &mut self.objs[index]
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        // TODO: Eliminate this cloning. More references
        let mut intersections = self
            .objs
            .iter()
            .map(|o| intersect(Rc::new(o.clone()), &ray))
            .flatten()
            .collect::<Intersections>();

        intersections.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        intersections
    }

    pub fn shade_hit(&self, comp: &PrecomputedHit) -> Color {
        let in_shadow = self.is_shadowed(comp.over_point);

        comp.object.material.lighting(
            self.light.unwrap(),
            comp.over_point,
            comp.eyev,
            comp.normalv,
            in_shadow,
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);

        let hit = intersections.hit();

        if hit.is_none() {
            return BLACK;
        }

        let comps = hit.unwrap().prepare_computations(ray);

        self.shade_hit(&comps)
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        assert!(
            self.light != None,
            "The world has no light for casting shadows"
        );

        let vector = self.light.unwrap().position - point;
        let distance = vector.mag();
        let direction = vector.norm();

        let ray = Ray::new(point, direction);
        let intersections = self.intersect(&ray);

        let hit = intersections.hit();

        if let Some(h) = hit {
            h.time < distance
        } else {
            false
        }
    }
}

impl Default for RaytracerWorld {
    fn default() -> Self {
        let mut world = Self::new();

        let mut s1 = world.new_sphere(CENTER_ORIGIN);
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = world.new_sphere(CENTER_ORIGIN);
        s2.transform = scaling(0.5, 0.5, 0.5);

        world.light = Some(Light::new(Point::new(-10.0, 10.0, -10.0), WHITE));
        world.objs = vec![s1, s2];

        world
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn equalish_to(&self, other: &Self) -> bool {
        equalish(self.x, other.x) && equalish(self.y, other.y) && equalish(self.z, other.z)
    }

    pub fn rounded(&self) -> Self {
        Self::new(round(self.x), round(self.y), round(self.z))
    }

    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// TODO: Template for float type
impl ops::Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl ops::Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// TODO: Template for float type
impl ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// TODO: Template for float type
impl ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn equalish_to(&self, other: &Self) -> bool {
        equalish(self.x, other.x) && equalish(self.y, other.y) && equalish(self.z, other.z)
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn norm(self) -> Self {
        let mag = self.mag();

        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    pub fn rounded(&self) -> Self {
        Self::new(round(self.x), round(self.y), round(self.z))
    }

    // TODO: Refactor away unnecessary cloning here
    pub fn reflect(&self, normal: &Vector) -> Self {
        let incoming: Vector = *self;

        incoming - *normal * 2.0_f32 * incoming.dot(*normal)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, time: Time) -> Point {
        self.origin + (self.direction * time)
    }

    pub fn transform(&self, transformation_matrix: &TransformationMatrix) -> Self {
        let origin = transformation_matrix * self.origin;

        let direction = transformation_matrix * self.direction;

        Self { origin, direction }
    }
}

#[derive(Debug)]
pub struct Intersection {
    pub time: Time,
    pub object: Rc<RaytracerObject>,
}

impl Intersection {
    pub fn prepare_computations(&self, ray: &Ray) -> PrecomputedHit {
        let time = self.time;
        let point = ray.position(self.time);
        let object = Rc::clone(&self.object);
        let eyev = -(ray.direction);
        let normalv = self.object.normal_at(point);
        let inside = normalv.dot(eyev) < 0.0;

        let normalv = if inside { -normalv } else { normalv };

        let over_point = point + normalv * EPSILON;

        PrecomputedHit {
            time,
            object,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }
}

pub fn intersect(object: Rc<RaytracerObject>, ray: &Ray) -> Intersections {
    let times = object.intersect(ray);

    times
        .iter()
        .map({
            |t| {
                Rc::new(Intersection {
                    time: *t,
                    object: Rc::clone(&object),
                })
            }
        })
        .collect()
}

pub trait Hittable {
    fn hit(&self) -> Option<Rc<Intersection>>;
}

impl Hittable for Intersections {
    fn hit(&self) -> Option<Rc<Intersection>> {
        let mut h: Option<Rc<Intersection>> = None;
        let mut min_t: Time = 0.0;

        for i in self.iter() {
            if i.time < 0.0 {
                continue;
            }

            if h.is_none() {
                min_t = i.time;
                h = Some(i.clone());
            }

            if min_t > i.time {
                min_t = i.time;
                h = Some(i.clone());
            }
        }

        h
    }
}

#[derive(Debug)]
pub struct PrecomputedHit {
    pub time: Time,
    pub object: Rc<RaytracerObject>,
    pub point: Point,
    pub over_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

pub fn equalish(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub hsize: f32,
    pub vsize: f32,
    pub field_of_view: f32,
    pub transform: TransformationMatrix,
    pub pixel_size: f32, // Pixel Size
    pub half_width: f32,
    pub half_height: f32,
}

impl Camera {
    pub fn new(hsize: f32, vsize: f32, field_of_view: f32) -> Self {
        let transform = TransformationMatrix::identity();

        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize / vsize;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / hsize;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform,
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f32 + 0.5) * self.pixel_size;
        let yoffset = (py as f32 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let world_point = Point::new(world_x, world_y, -1.0);

        let transform_inverse = self.transform.inverse();

        let pixel: Point = (transform_inverse * world_point).into();

        let origin: Point = (transform_inverse * CENTER_ORIGIN).into();

        let direction = (pixel - origin).norm();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &RaytracerWorld) -> Canvas {
        let mut image = Canvas::new(self.hsize as u32, self.vsize as u32);

        for y in 0..(self.vsize as usize) {
            for x in 0..(self.hsize as usize) {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x as u32, y as u32, color);
            }
        }

        image
    }

    // TODO: Rename to render_row_to since that's what's actually being rendered
    pub fn render_column_to(&self, world: &RaytracerWorld, y: usize, image: &mut Canvas) {
        for x in 0..(self.hsize as usize) {
            let ray = self.ray_for_pixel(x, y);
            let color = world.color_at(&ray);

            image.write_pixel(x as u32, y as u32, color);
        }
    }
}

pub fn render_threaded(world: RaytracerWorld, camera: Camera) -> Canvas {
    let mut image = Canvas::new(camera.hsize as u32, camera.vsize as u32);

    let mut handles = vec![];

    let cols: Vec<(usize, usize, Canvas)> = vec![]; // usizes = start/end range of larger canvas
    let cols = Arc::new(Mutex::new(cols));

    let hsize = camera.hsize.round() as usize;
    let vsize = camera.vsize.round() as usize;

    let mut start_y = 0;
    let mut end_y = 0;

    while start_y < vsize && end_y < vsize {
        let world = world.clone();
        let camera = camera.clone();
        let cols = Arc::clone(&cols);

        end_y = start_y + (vsize / 10);
        if end_y > vsize {
            end_y = vsize;
        }

        let mut canvas = Canvas::new(hsize as u32, vsize as u32);

        let handle = thread::spawn(move || {
            let cols = &mut *cols.lock().unwrap();

            for y in start_y..end_y {
                camera.render_column_to(&world, y, &mut canvas);
            }

            cols.push((start_y, end_y, canvas));
        });

        handles.push(handle);

        start_y = end_y;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for col in cols.lock().unwrap().iter() {
        let (start_y, end_y, canvas) = col;

        for x in 0..hsize {
            for y in *start_y..*end_y {
                let x = x as u32;
                let y = y as u32;
                image.write_pixel(x, y, canvas.pixel_at(x, y));
            }
        }
    }

    image.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equalish_is_true_for_diff_below_epsilon() {
        assert!(equalish(1.0, 1.000001));
    }

    #[test]
    fn equalish_is_false_for_diff_above_epsilon() {
        assert!(!equalish(1.0, 1.00002));
    }

    mod point_tests {
        use super::*;

        #[test]
        fn equalish_is_true_for_diff_below_epsilon() {
            let point_a = Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            let point_b = Point {
                x: 1.000001,
                y: 1.000001,
                z: 1.000001,
            };

            assert!(point_a.equalish_to(&point_b));
        }

        #[test]
        fn equalish_is_false_for_diff_above_epsilon() {
            let point_a = Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            let point_b = Point {
                x: 1.00002,
                y: 1.00002,
                z: 1.00002,
            };

            assert!(!point_a.equalish_to(&point_b));
        }

        #[test]
        fn add_vector_sums_each_pair_of_values() {
            let a = Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let b = Vector {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            };

            let expected = Point {
                x: 3.0,
                y: 5.0,
                z: 7.0,
            };

            assert!((a + b).equalish_to(&expected));
        }

        #[test]
        fn subtract_vector_subtracts_latter_from_former_for_each_pair_of_values() {
            let a = Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let b = Vector {
                x: 1.0,
                y: 1.0,
                z: 4.0,
            };

            let expected = Point {
                x: 0.0,
                y: 1.0,
                z: -1.0,
            };

            assert!((a - b).equalish_to(&expected));
        }

        #[test]
        fn subtract_point_subtracts_latter_from_former_for_each_pair_of_values() {
            let a = Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let b = Point {
                x: 1.0,
                y: 1.0,
                z: 4.0,
            };

            let expected = Vector {
                x: 0.0,
                y: 1.0,
                z: -1.0,
            };

            assert!((a - b).equalish_to(&expected));
        }
    }

    mod vector_tests {
        use super::*;

        #[test]
        fn equalish_is_true_for_diff_below_epsilon() {
            let vector_a = Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            let vector_b = Vector {
                x: 1.000001,
                y: 1.000001,
                z: 1.000001,
            };

            assert!(vector_a.equalish_to(&vector_b));
        }

        #[test]
        fn equalish_is_false_for_diff_above_epsilon() {
            let vector_a = Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };

            let vector_b = Vector {
                x: 1.00002,
                y: 1.00002,
                z: 1.00002,
            };

            assert!(!vector_a.equalish_to(&vector_b));
        }

        #[test]
        fn add_vector_sums_each_pair_of_values() {
            let a = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let b = Vector {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            };

            let expected = Vector {
                x: 3.0,
                y: 5.0,
                z: 7.0,
            };

            assert!((a + b).equalish_to(&expected));
        }

        #[test]
        fn add_point_sums_each_pair_of_values() {
            let a = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let b = Point {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            };

            let expected = Point {
                x: 3.0,
                y: 5.0,
                z: 7.0,
            };

            assert!((a + b).equalish_to(&expected));
        }

        #[test]
        fn subtract_vector_subtracts_second_value_from_for_for_each_pair() {
            let a = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let b = Vector {
                x: 1.0,
                y: 3.0,
                z: 2.0,
            };

            let expected = Vector {
                x: 0.0,
                y: -1.0,
                z: 1.0,
            };

            assert!((a - b).equalish_to(&expected));
        }

        #[test]
        fn negating_vector_inverts_each_value() {
            let v = Vector {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            };

            let expected = Vector {
                x: -1.0,
                y: 1.0,
                z: 0.0,
            };

            assert!((-v).equalish_to(&expected));
        }

        #[test]
        fn multiply_vector_multiplies_each_value() {
            let a = Vector {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            };
            let b = 5.0;

            let expected = Vector {
                x: 5.0,
                y: -5.0,
                z: 0.0,
            };

            assert!((a * b).equalish_to(&expected));
        }

        #[test]
        fn divide_vector_divides_each_value() {
            let a = Vector {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            };
            let b = 2.0;

            let expected = Vector {
                x: 0.5,
                y: -0.5,
                z: 0.0,
            };

            let result = a / b;

            assert!(
                result.equalish_to(&expected),
                "Expected {:?} but got {:?}",
                expected,
                result
            );
        }

        #[test]
        fn magnitudes_of_vectors() {
            let v = Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
            let expected = 1.0;
            let result = v.mag();
            assert!(
                equalish(expected, result),
                "Expected {} but got {}",
                expected,
                result
            );

            let v = Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            };
            let expected = 1.0;
            let result = v.mag();
            assert!(
                equalish(expected, result),
                "Expected {} but got {}",
                expected,
                result
            );

            let v = Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            };
            let expected = 1.0;
            let result = v.mag();
            assert!(
                equalish(expected, result),
                "Expected {} but got {}",
                expected,
                result
            );

            let v = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let expected = (14.0 as f32).sqrt();
            let result = v.mag();
            assert!(
                equalish(expected, result),
                "Expected {} but got {}",
                expected,
                result
            );

            let v = Vector {
                x: -1.0,
                y: -2.0,
                z: -3.0,
            };
            let expected = (14.0 as f32).sqrt();
            let result = v.mag();
            assert!(
                equalish(expected, result),
                "Expected {} but got {}",
                expected,
                result
            );
        }

        #[test]
        fn normalizing_vectors() {
            let v = Vector {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            };
            let expected = Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
            let result = v.norm();
            assert!(
                expected.equalish_to(&result),
                "Expected {:?} but got {:?}",
                expected,
                result
            );

            let v = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let x = 1.0 / (14.0 as f32).sqrt();
            let y = 2.0 / (14.0 as f32).sqrt();
            let z = 3.0 / (14.0 as f32).sqrt();
            let expected = Vector { x, y, z };
            let result = v.norm();
            assert!(
                expected.equalish_to(&result),
                "Expected {:?} but got {:?}",
                expected,
                result
            );
        }

        #[test]
        fn dot_products_of_vectors() {
            let a = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let b = Vector {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            };
            let expected = 20.0;
            let result = a.dot(b);
            assert!(
                equalish(expected, result),
                "Expected {:?} but got {:?}",
                expected,
                result
            );
        }

        #[test]
        fn cross_products_of_vectors() {
            let a = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let b = Vector {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            };
            let expected = Vector {
                x: -1.0,
                y: 2.0,
                z: -1.0,
            };
            let result = a.cross(b);
            assert!(
                expected.equalish_to(&result),
                "Expected {:?} but got {:?}",
                expected,
                result
            );
        }
    }
}
