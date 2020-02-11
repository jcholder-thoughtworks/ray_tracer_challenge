// Derived from example at https://github.com/bbqsrc/cucumber-rust/blob/master/README.md

#![recursion_limit = "256"]

use std::rc::Rc;

use ndarray::*;

use cucumber::{after, before, cucumber};

use ray_tracer_challenge::color::*;
use ray_tracer_challenge::light::*;
use ray_tracer_challenge::material::*;
use ray_tracer_challenge::objects::*;
use ray_tracer_challenge::*;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    rw: RaytracerWorld,
    colors: Vec<Color>,
    matrix: Array<f32, Ix2>,
    matrix_a: Rc<Array<f32, Ix2>>,
    matrix_b: Rc<Array<f32, Ix2>>,
    matrix_c: Rc<Array<f32, Ix2>>,
    matrix_t: Rc<Array<f32, Ix2>>,
    half_quarter: Rc<Array<f32, Ix2>>,
    full_quarter: Rc<Array<f32, Ix2>>,
    transform: Rc<Array<f32, Ix2>>,
    m: Rc<Array<f32, Ix2>>,
    t: Rc<Array<f32, Ix2>>,
    p: Point,
    p2: Point,
    p3: Point,
    p4: Point,
    origin: Point,
    v: Vector,
    direction: Vector,
    n: Vector,
    inv: Array<f32, Ix2>,
    tuple: (f32, f32, f32, f32),
    r: Ray,
    rv: Vector,
    r2: Ray,
    s: Sphere,
    xs: Intersections,
    i: Option<Rc<Intersection>>,
    i1: Option<Rc<Intersection>>,
    i2: Option<Rc<Intersection>>,
    i3: Option<Rc<Intersection>>,
    i4: Option<Rc<Intersection>>,
    intensity: Color,
    position: Point,
    light: Light,
    mt: Material,
    eyev: Vector,
    normalv: Vector,
    result: Color,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        let mut rw = RaytracerWorld::default();
        let s = rw.new_sphere(CENTER_ORIGIN);

        // This function is called every time a new scenario is started
        MyWorld {
            rw,
            colors: vec![BLACK; 3],
            // TODO: Consider using Option here to make it more obvious when they haven't been set
            matrix: Array::from_elem((4, 4), 0.0),
            matrix_a: Rc::new(Array::from_elem((4, 4), 4.0)),
            matrix_b: Rc::new(Array::from_elem((4, 4), 0.0)),
            matrix_c: Rc::new(Array::from_elem((4, 4), 0.0)),
            matrix_t: Rc::new(Array::from_elem((4, 4), 0.0)),
            half_quarter: Rc::new(Array::from_elem((4, 4), 0.0)),
            full_quarter: Rc::new(Array::from_elem((4, 4), 0.0)),
            transform: Rc::new(Array::from_elem((4, 4), 0.0)),
            m: Rc::new(Array::from_elem((4, 4), 0.0)),
            t: Rc::new(Array::from_elem((4, 4), 0.0)),
            p: CENTER_ORIGIN,
            p2: CENTER_ORIGIN,
            p3: CENTER_ORIGIN,
            p4: CENTER_ORIGIN,
            origin: CENTER_ORIGIN,
            v: STATIONARY,
            direction: STATIONARY,
            n: STATIONARY,
            inv: Array::from_elem((4, 4), 0.0),
            tuple: (0.0, 0.0, 0.0, 0.0),
            r: Ray::new(CENTER_ORIGIN, STATIONARY),
            rv: STATIONARY,
            r2: Ray::new(CENTER_ORIGIN, STATIONARY),
            s,
            xs: vec![],
            i: None,
            i1: None,
            i2: None,
            i3: None,
            i4: None,
            intensity: BLACK,
            position: CENTER_ORIGIN,
            light: Light::new(CENTER_ORIGIN, BLACK),
            mt: Material::default(),
            eyev: STATIONARY,
            normalv: STATIONARY,
            result: BLACK,
        }
    }
}

mod example_steps {
    use std::f32::consts::PI;
    use std::rc::Rc;

    use cucumber::steps;

    use gherkin;
    use ndarray::*;

    use ray_tracer_challenge::color::*;
    use ray_tracer_challenge::light::*;
    use ray_tracer_challenge::material::*;
    use ray_tracer_challenge::math::transforms::*;
    use ray_tracer_challenge::math::*;
    use ray_tracer_challenge::*;

    fn table_to_matrix(table: gherkin::Table, size: (Ix, Ix)) -> Array<f32, Ix2> {
        let mut matrix = Array::from_elem(size, 0.0);

        for (c, value) in table.header.iter().enumerate() {
            matrix[[0, c]] = value.parse().unwrap();
        }

        for (r, row) in table.rows.iter().enumerate() {
            for (c, value) in row.iter().enumerate() {
                matrix[[r + 1, c]] = value.parse().unwrap();
            }
        }

        matrix
    }

    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
        given regex r"^the following (.*)x(.*) matrix M:$" |world, matches, step| {
            let width = matches[1].parse().unwrap();
            let height = matches[2].parse().unwrap();

            let table = step.table().unwrap().clone();

            world.matrix = Array::from_elem((width, height), 0.0);

            for (c, value) in table.header.iter().enumerate() {
                world.matrix[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    world.matrix[[r + 1,c]] = value.parse().unwrap();
                }
            }
        };

        given regex r"^the following (.*)x(.*) matrix A:$" |world, matches, step| {
            let width = matches[1].parse().unwrap();
            let height = matches[2].parse().unwrap();

            let table = step.table().unwrap().clone();

            world.matrix_a = Rc::new(table_to_matrix(table, (width, height)));
        };

        given regex r"^the following (.*)x(.*) matrix B:$" |world, matches, step| {
            let width = matches[1].parse().unwrap();
            let height = matches[2].parse().unwrap();

            let table = step.table().unwrap().clone();

            world.matrix_b = Rc::new(table_to_matrix(table, (width, height)));
        };

        given "the following matrix B:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut array = Array::from_elem((4, 4), 0.0);

            for (c, value) in table.header.iter().enumerate() {
                array[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    array[[r + 1,c]] = value.parse().unwrap();
                }
            }

            world.matrix_b = Rc::new(array);
        };

        given "the following matrix A:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut array = Array::from_elem((4, 4), 0.0);

            for (c, value) in table.header.iter().enumerate() {
                array[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    array[[r + 1,c]] = value.parse().unwrap();
                }
            }

            world.matrix_a = Rc::new(array);
        };

        given "C ← A * B" |world, _step| {
            world.matrix_c = Rc::new(world.matrix_a.dot(world.matrix_b.as_ref()));
        };

        then regex r"^M\[(.*),(.*)\] = (.*)$" |world, matches, _step| {
            let r: usize = matches[1].parse().unwrap();
            let c: usize = matches[2].parse().unwrap();
            let expected: f32 = matches[3].parse().unwrap();

            assert_eq!(expected, world.matrix[[r, c]]);
        };

        given regex r"^c(.*) = color\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let color_i: usize = matches[1].parse().unwrap();

            let red: f32 = matches[2].parse().unwrap();
            let green: f32 = matches[3].parse().unwrap();
            let blue: f32 = matches[4].parse().unwrap();

            let color = Color::new(red, green, blue);

            world.colors[color_i] = color;
        };

        given regex r"^b ← tuple\((.*), (.*), (.*), (.*)\)$" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();
            let t4: f32 = matches[4].parse().unwrap();

            world.tuple = (t1, t2, t3, t4);
        };

        given regex r"^B ← submatrix\(A, (.*), (.*)\)$" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            world.matrix_b = Rc::new(world.matrix_a.submatrix(row_i, col_i));
        };

        given "B ← inverse(A)" |world, _step| {
            world.matrix_b = Rc::new(world.matrix_a.inverse());
        };

        given regex r"^transform ← translation\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();

            world.transform = Rc::new(translation(t1, t2, t3));
        };

        given regex r"^m ← translation\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();

            world.m = Rc::new(translation(t1, t2, t3));
        };

        given regex r"^m ← scaling\(([0-9.]*), ([0-9.]*), ([0-9.]*)\)$" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();

            world.m = Rc::new(scaling(t1, t2, t3));
        };

        given regex r"^m ← scaling\(([0-9.]*), ([0-9.]*), ([0-9.]*)\) \* rotation_z\(π/5\)$" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();

            let st = scaling(t1, t2, t3);
            let rzt = rotation_z(PI / 5.0);

            world.m = Rc::new(st.dot(&rzt));
        };

        given regex r"^transform ← scaling\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();

            world.transform = Rc::new(scaling(t1, t2, t3));
        };

        given regex r"^p ← point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.p = Point::new(x, y, z);
        };

        given regex r"^v ← vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.v = Vector::new(x, y, z);
        };

        given "inv ← inverse(transform)" |world, _step| {
            world.inv = world.transform.inverse();
        };

        given regex r"^half_quarter ← rotation_x\(π / (.*)\)$" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.half_quarter = Rc::new(rotation_x(PI / denominator));
        };

        given regex r"^half_quarter ← rotation_y\(π / (.*)\)$" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.half_quarter = Rc::new(rotation_y(PI / denominator));
        };

        given regex r"^half_quarter ← rotation_z\(π / (.*)\)$" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.half_quarter = Rc::new(rotation_z(PI / denominator));
        };

        given regex r"^full_quarter ← rotation_x\(π / (.*)\)$" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.full_quarter = Rc::new(rotation_x(PI / denominator));
        };

        given regex r"^full_quarter ← rotation_y\(π / (.*)\)$" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.full_quarter = Rc::new(rotation_y(PI / denominator));
        };

        given regex r"^full_quarter ← rotation_z\(π / (.*)\)$" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.full_quarter = Rc::new(rotation_z(PI / denominator));
        };

        given "inv ← inverse(half_quarter)" |world, _step| {
            world.inv = world.half_quarter.inverse();
        };

        given regex r"^transform ← shearing\((.*), (.*), (.*), (.*), (.*), (.*)\)$" |world, matches, _step| {
            let xy: f32 = matches[1].parse().unwrap();
            let xz: f32 = matches[2].parse().unwrap();
            let yx: f32 = matches[3].parse().unwrap();
            let yz: f32 = matches[4].parse().unwrap();
            let zx: f32 = matches[5].parse().unwrap();
            let zy: f32 = matches[6].parse().unwrap();

            world.transform = Rc::new(shearing(xy, xz, yx, yz, zx, zy));
        };

        given regex r"^A ← rotation_x\(π / (.*)\)$" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.matrix_a = Rc::new(rotation_x(PI / denominator));
        };

        given regex r"^B ← scaling\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.matrix_b = Rc::new(scaling(x, y, z));
        };

        given regex r"^C ← translation\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.matrix_c = Rc::new(translation(x, y, z));
        };

        given regex r"^t ← translation\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.t = Rc::new(translation(x, y, z));
        };

        given regex r"^origin ← point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.origin = Point::new(x, y, z);
        };

        given regex r"^direction ← vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.direction = Vector::new(x, y, z);
        };

        given regex r"^r ← ray\(point\((.*), (.*), (.*)\), vector\((.*), (.*), (.*)\)\)$" |world, matches, _step| {
            let px: f32 = matches[1].parse().unwrap();
            let py: f32 = matches[2].parse().unwrap();
            let pz: f32 = matches[3].parse().unwrap();

            let vx: f32 = matches[4].parse().unwrap();
            let vy: f32 = matches[5].parse().unwrap();
            let vz: f32 = matches[6].parse().unwrap();

            world.r = Ray::new(Point::new(px, py, pz), Vector::new(vx, vy, vz));
        };

        given "s ← sphere()" |world, _step| {
            world.s = world.rw.new_sphere(CENTER_ORIGIN);
        };

        given regex r"^i1 ← intersection\((.*), s\)$" |world, matches, _step| {
            let time: Time = matches[1].parse().unwrap();

            let object = Rc::new(world.s.clone());

            world.i1 = Some(Rc::new(Intersection { time, object }));
        };

        given regex r"^i2 ← intersection\((.*), s\)$" |world, matches, _step| {
            let time: Time = matches[1].parse().unwrap();

            let object = Rc::new(world.s.clone());

            world.i2 = Some(Rc::new(Intersection { time, object }));
        };

        given regex r"^i3 ← intersection\((.*), s\)$" |world, matches, _step| {
            let time: Time = matches[1].parse().unwrap();

            let object = Rc::new(world.s.clone());

            world.i3 = Some(Rc::new(Intersection { time, object }));
        };

        given regex r"^i4 ← intersection\((.*), s\)$" |world, matches, _step| {
            let time: Time = matches[1].parse().unwrap();

            let object = Rc::new(world.s.clone());

            world.i4 = Some(Rc::new(Intersection { time, object }));
        };

        given "xs ← intersections(i2, i1)" |world, _step| {
            let i1 = match &world.i1 {
                Some(i) => i.clone(),
                None => panic!("world.i1 was not assigned"),
            };

            let i2 = match &world.i2 {
                Some(i) => i.clone(),
                None => panic!("world.i2 was not assigned"),
            };

            world.xs = vec![i2, i1];
        };

        given "xs ← intersections(i1, i2, i3, i4)" |world, _step| {
            let i1 = match &world.i1 {
                Some(i) => i.clone(),
                None => panic!("world.i1 was not assigned"),
            };

            let i2 = match &world.i2 {
                Some(i) => i.clone(),
                None => panic!("world.i2 was not assigned"),
            };

            let i3 = match &world.i3 {
                Some(i) => i.clone(),
                None => panic!("world.i3 was not assigned"),
            };

            let i4 = match &world.i4 {
                Some(i) => i.clone(),
                None => panic!("world.i4 was not assigned"),
            };

            world.xs = vec![i1, i2, i3, i4];
        };

        given regex r"^set_transform\(s, translation\((.*), (.*), (.*)\)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.s.transform = translation(x, y, z);
        };

        given "set_transform(s, m)" |world, _step| {
            world.s.transform = world.m.as_ref().clone();
        };

        given regex r"^n ← vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = match matches[1].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[1].parse().unwrap(),
            };
            let y: f32 = match matches[2].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[2].parse().unwrap(),
            };
            let z: f32 = match matches[3].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[3].parse().unwrap(),
            };

            world.n = Vector::new(x, y, z);
        };

        given "intensity ← color(1, 1, 1)" |world, _step| {
            world.intensity = Color::new(1.0, 1.0, 1.0);
        };

        given "position ← point(0, 0, 0)" |world, _step| {
            world.position = Point::new(0.0, 0.0, 0.0);
        };

        given "m ← material()" |world, _step| {
            world.mt = Material::default();
        };

        given regex r"^m.ambient ← (.*)$" |world, matches, _step| {
            let new_value : f32 = matches[1].parse().unwrap();

            world.mt.ambient = new_value;
        };

        given regex r"^eyev ← vector\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = match matches[1].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[1].parse().unwrap(),
            };
            let y: f32 = match matches[2].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[2].parse().unwrap(),
            };
            let z: f32 = match matches[3].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[3].parse().unwrap(),
            };

            world.eyev = Vector::new(x, y, z);
        };

        given regex r"^normalv ← vector\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.normalv = Vector::new(x, y, z);
        };

        given regex r"light ← point_light\(point\((.*), (.*), (.*)\), color\((.*), (.*), (.*)\)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let position = Point::new(x, y, z);

            let red: f32 = matches[4].parse().unwrap();
            let green: f32 = matches[5].parse().unwrap();
            let blue: f32 = matches[6].parse().unwrap();

            let intensity = Color::new(red, green, blue);

            world.light = Light::new(position, intensity);
        };

        when "p2 ← A * p" |world, _step| {
            world.p2 = world.matrix_a.as_ref() * world.p;
        };

        when "p3 ← B * p2" |world, _step| {
            world.p3 = world.matrix_b.as_ref() * world.p2;
        };

        when "p4 ← C * p3" |world, _step| {
            world.p4 = world.matrix_c.as_ref() * world.p3;
        };

        when "T ← C * B * A" |world, _step| {
            world.matrix_t = Rc::new(world.matrix_c.dot(&world.matrix_b.dot(world.matrix_a.as_ref())));
        };

        when "r ← ray(origin, direction)" |world, _step| {
            world.r = Ray::new(world.origin, world.direction);
        };

        when "xs ← intersect(s, r)" |world, _step| {
            let sphere_rc: Rc<dyn Interceptable> = Rc::new(world.s.clone());

            world.xs = intersect(&sphere_rc, &world.r);
        };

        when "xs ← intersections(i1, i2)" |world, _step| {
            let i1 = match &world.i1 {
                Some(i) => i.clone(),
                None => panic!("world.i1 was not assigned"),
            };

            let i2 = match &world.i2 {
                Some(i) => i.clone(),
                None => panic!("world.i2 was not assigned"),
            };

            world.xs = vec![i1, i2];
        };

        when regex r"^i ← intersection\((.*), s\)$" |world, matches, _step| {
            let time: Time = matches[1].parse().unwrap();

            let object = Rc::new(world.s.clone());

            world.i = Some(Rc::new(Intersection { time, object }));
        };

        when "i ← hit(xs)" |world, _step| {
            world.i = world.xs.hit();
        };

        when "r2 ← transform(r, m)" |world, _step| {
            world.r2 = world.r.transform(&world.m);
        };

        when "set_transform(s, t)" |world, _step| {
            world.s.transform = world.t.as_ref().clone();
        };

        when regex r"^set_transform\(s, scaling\((.*), (.*), (.*)\)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.s.transform = scaling(x, y, z);
        };

        when regex r"^set_transform\(s, translation\((.*), (.*), (.*)\)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.s.transform = translation(x, y, z);
        };

        when regex r"^n ← normal_at\(s, point\((.*), (.*), (.*)\)\)$" |world, matches, _step| {
            let x: f32 = match matches[1].as_str() {
                "√3/3" => 3.0_f32.sqrt() / 3.0,
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[1].parse().unwrap(),
            };
            let y: f32 = match matches[2].as_str() {
                "√3/3" => 3.0_f32.sqrt() / 3.0,
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[2].parse().unwrap(),
            };
            let z: f32 = match matches[3].as_str() {
                "√3/3" => 3.0_f32.sqrt() / 3.0,
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[3].parse().unwrap(),
            };

            world.n = world.s.normal_at(Point::new(x, y, z));
        };

        when "r ← reflect(v, n)" |world, _step| {
            world.rv = world.v.reflect(&world.n);
        };

        when "light ← point_light(position, intensity)" |world, _step| {
            world.light = Light::new(world.position, world.intensity);
        };

        when "m ← s.material" |world, _step| {
            world.mt = world.s.material;
        };

        when "s.material ← m" |world, _step| {
            world.s.material = world.mt;
        };

        when "result ← lighting(m, light, position, eyev, normalv)" |world, _step| {
            world.result = world.mt.lighting(world.light, world.position, world.eyev, world.normalv);
        };

        then regex r"^c(.*) \+ c(.*) = color\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let color_i1: usize = matches[1].parse().unwrap();
            let color1 = world.colors[color_i1];

            let color_i2: usize = matches[2].parse().unwrap();
            let color2 = world.colors[color_i2];

            let red: f32 = matches[3].parse().unwrap();
            let green: f32 = matches[4].parse().unwrap();
            let blue: f32 = matches[5].parse().unwrap();

            let expected = Color::new(red, green, blue);

            assert!((color1 + color2).equalish_to(&expected));
        };

        then "A = B" |world, _step| {
            assert_eq!(world.matrix_a, world.matrix_b);
        };

        then "A != B" |world, _step| {
            assert_ne!(world.matrix_a, world.matrix_b);
        };

        then regex r"^A \* b = tuple\((.*), (.*), (.*), (.*)\)$" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();
            let t4: f32 = matches[4].parse().unwrap();

            let mut tuple_matrix: Array<f32, Ix1> = Array::from_elem(4, 0.0);
            tuple_matrix[[0]] = world.tuple.0;
            tuple_matrix[[1]] = world.tuple.1;
            tuple_matrix[[2]] = world.tuple.2;
            tuple_matrix[[3]] = world.tuple.3;

            let multiplied_matrix = world.matrix_a.dot(&tuple_matrix);

            let matrix_as_tuple = (
                multiplied_matrix[0],
                multiplied_matrix[1],
                multiplied_matrix[2],
                multiplied_matrix[3],
            );

            let expected_tuple = (t1, t2, t3, t4);

            assert_eq!(expected_tuple, matrix_as_tuple);
        };

        then "A * B is the following 4x4 matrix:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut expected_matrix: Array<f32, Ix2> = Array::from_elem((4, 4), 0.0);

            for (c, value) in table.header.iter().enumerate() {
                expected_matrix[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    expected_matrix[[r + 1,c]] = value.parse().unwrap();
                }
            }

            let actual_matrix = world.matrix_a.dot(world.matrix_b.as_ref());

            assert_eq!(expected_matrix, actual_matrix);
        };

        then "A * identity_matrix = A" |world, _step| {
            let identity_matrix: Array<f32, Ix2> = Array::eye(4);

            let expected = world.matrix_a.dot(&identity_matrix);

            // TODO: Not an ideal cloning. How to make this more efficient?
            let actual = (*world.matrix_a).clone();

            assert_eq!(expected, actual);
        };

        then "transpose(A) is the following matrix:" |world, step| {
            let table = step.table().unwrap().clone();

            let transposed = world.matrix_a.t();

            let expected = table_to_matrix(table, (4, 4));

            assert_eq!(expected, transposed);
        };

        then regex r"^determinant\(A\) = (.*)$" |world, matches, _step| {
            let expected: f32 = matches[1].parse().unwrap();

            let actual = world.matrix_a.determinant();

            assert_eq!(expected, actual);
        };

        then regex r"^determinant\(B\) = (.*)$" |world, matches, _step| {
            let expected: f32 = matches[1].parse().unwrap();

            let actual = world.matrix_b.determinant();

            assert_eq!(expected, actual);
        };

        then regex r"^minor\(A, (.*), (.*)\) = (.*)$" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let expected: f32 = matches[3].parse().unwrap();

            let actual = world.matrix_a.minor(row_i, col_i);

            assert_eq!(expected, actual);
        };

        then regex r"^cofactor\(A, (.*), (.*)\) = (.*)$" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let expected: f32 = matches[3].parse().unwrap();

            let actual = world.matrix_a.cofactor(row_i, col_i);

            assert_eq!(expected, actual);
        };

        then regex r"^submatrix\(A, (.*), (.*)\) is the following (.*)x(.*) matrix:$" |world, matches, step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let row_count: usize = matches[3].parse().unwrap();
            let col_count: usize = matches[4].parse().unwrap();

            let actual = world.matrix_a.submatrix(row_i, col_i);

            let expected = table_to_matrix(step.table().unwrap().clone(), (row_count, col_count));

            assert_eq!(expected, actual);
        };

        then "A is invertible" |world, _step| {
            assert!(world.matrix_a.invertible());
        };

        then "A is not invertible" |world, _step| {
            assert!(!world.matrix_a.invertible());
        };

        then regex r"^B\[(.*),(.*)\] = (.*)/(.*)$" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let numerator: f32 = matches[3].parse().unwrap();
            let denominator: f32 = matches[4].parse().unwrap();

            let expected: f32 = (numerator as f32) / (denominator as f32);

            let actual = world.matrix_b[[row_i, col_i]] as f32;

            assert_eq!(expected, actual);
        };

        then regex r"^B is the following (.*)x(.*) matrix:$" |world, matches, step| {
            let width = matches[1].parse().unwrap();
            let height = matches[2].parse().unwrap();

            let table = step.table().unwrap().clone();

            let expected = table_to_matrix(table, (width, height));

            assert_eq!(expected, world.matrix_b.rounded());
        };

        then regex r"^inverse\(A\) is the following (.*)x(.*) matrix:$" |world, matches, step| {
            let width = matches[1].parse().unwrap();
            let height = matches[2].parse().unwrap();

            let table = step.table().unwrap().clone();

            let expected = table_to_matrix(table, (width, height));

            assert_eq!(expected, world.matrix_a.inverse().rounded());
        };

        then "C * inverse(B) = A" |world, _step| {
            let expected = (*world.matrix_a).clone();

            let actual = world.matrix_c.dot(&world.matrix_b.inverse()).rounded();

            assert_eq!(expected, actual);
        };

        then regex r"^transform \* p = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.transform.as_ref() * world.p;

            assert_eq!(expected, actual);
        };

        then regex r"^inv \* p = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = match matches[1].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[1].parse().unwrap(),
            };
            let y: f32 = match matches[2].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[2].parse().unwrap(),
            };
            let z: f32 = match matches[3].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[3].parse().unwrap(),
            };

            let expected = Point::new(x, y, z);

            let actual = &world.inv * world.p;

            assert_eq!(expected.rounded(), actual.rounded());
        };

        then "transform * v = v" |world, _step| {
            let expected = world.v;

            let actual = world.transform.as_ref() * world.v;

            assert_eq!(expected, actual);
        };

        then regex r"^transform \* v = vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Vector::new(x, y, z);

            let actual = world.transform.as_ref() * world.v;

            assert_eq!(expected, actual);
        };

        then regex r"^inv \* v = vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Vector::new(x, y, z);

            let actual = &world.inv * world.v;

            assert_eq!(expected, actual);
        };

        then regex r"^half_quarter \* p = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = match matches[1].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[1].parse().unwrap(),
            };
            let y: f32 = match matches[2].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[2].parse().unwrap(),
            };
            let z: f32 = match matches[3].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[3].parse().unwrap(),
            };

            let expected = Point::new(x, y, z);

            let actual = world.half_quarter.as_ref() * world.p;

            assert_eq!(expected, actual);
        };

        then regex r"^full_quarter \* p = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.full_quarter.as_ref() * world.p;

            assert_eq!(expected, actual.rounded());
        };

        then regex r"^p2 = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.p2;

            assert_eq!(expected, actual.rounded());
        };

        then regex r"^p3 = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.p3;

            assert_eq!(expected, actual.rounded());
        };

        then regex r"^p4 = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.p4;

            assert_eq!(expected, actual.rounded());
        };

        then regex r"^T \* p = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.matrix_t.as_ref() * world.p;

            assert_eq!(expected, actual.rounded());
        };

        then "r.origin = origin" |world, _step| {
            assert_eq!(world.origin, world.r.origin);
        };

        then regex r"^r2.origin = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.r2.origin;

            assert_eq!(expected, actual);
        };

        then "r.direction = direction" |world, _step| {
            assert_eq!(world.direction, world.r.direction);
        };

        then regex r"^r2.direction = vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Vector::new(x, y, z);

            let actual = world.r2.direction;

            assert_eq!(expected, actual);
        };

        then regex r"^position\(r, (.*)\) = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let t: f32 = matches[1].parse().unwrap();
            let x: f32 = matches[2].parse().unwrap();
            let y: f32 = matches[3].parse().unwrap();
            let z: f32 = matches[4].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.r.position(t);

            assert_eq!(expected, actual.rounded());
        };

        then regex r"^xs.count = (.*)$" |world, matches, _step| {
            let count: usize = matches[1].parse().unwrap();

            let expected = count;

            let actual = world.xs.len();

            assert_eq!(expected, actual);
        };

        then regex r"^xs\[(.*)\].t = (.*)$" |world, matches, _step| {
            let index: usize = matches[1].parse().unwrap();
            let time: Time = matches[2].parse().unwrap();

            let expected = time;

            let actual = world.xs[index].time;

            assert_eq!(expected, actual);
        };

        then regex r"^xs\[(.*)\].object = s$" |world, matches, _step| {
            let index: usize = matches[1].parse().unwrap();

            let expected = world.s.id();

            let actual = world.xs[index].object.id();

            assert_eq!(expected, actual);
        };

        then regex r"^i.t = (.*)$" |world, matches, _step| {
            let time: Time = matches[1].parse().unwrap();

            let expected = time;

            let actual = match world.i.as_ref() {
                Some(interception) => interception.time,
                None => panic!("world.i was not assigned"),
            };

            assert_eq!(expected, actual);
        };

        then "i.object = s" |world, _step| {
            let expected = world.s.id();

            let actual = match world.i.as_ref() {
                Some(interception) => interception.object.id(),
                None => panic!("world.i was not assigned"),
            };

            assert_eq!(expected, actual);
        };

        then "i = i1" |world, _step| {
            let expected = match world.i.as_ref() {
                Some(interception) => interception.object.id(),
                None => panic!("world.i was not assigned"),
            };

            let actual = match world.i1.as_ref() {
                Some(interception) => interception.object.id(),
                None => panic!("world.i1 was not assigned"),
            };

            assert_eq!(expected, actual);
        };

        then "i = i2" |world, _step| {
            let expected = match world.i.as_ref() {
                Some(interception) => interception.object.id(),
                None => panic!("world.i was not assigned"),
            };

            let actual = match world.i2.as_ref() {
                Some(interception) => interception.object.id(),
                None => panic!("world.i2 was not assigned"),
            };

            assert_eq!(expected, actual);
        };

        then "i = i4" |world, _step| {
            let expected = match world.i.as_ref() {
                Some(interception) => interception.object.id(),
                None => panic!("world.i was not assigned"),
            };

            let actual = match world.i4.as_ref() {
                Some(interception) => interception.object.id(),
                None => panic!("world.i4 was not assigned"),
            };

            assert_eq!(expected, actual);
        };

        then "i is nothing" |world, _step| {
            match world.i.as_ref() {
                Some(i) => panic!("world.i should have been None but it was {:?}", i),
                _ => assert!(true),
            };
        };

        then "s.transform = identity_matrix" |world, _step| {
            let expected: &Array<f32, Ix2> = &Array::eye(4);

            let actual = &world.s.transform;

            assert_eq!(expected, actual);
        };

        then "s.transform = t" |world, _step| {
            let expected = world.t.as_ref().clone();

            let actual = world.s.transform.clone();

            assert_eq!(expected, actual);
        };

        then regex r"^n = vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = match matches[1].as_str() {
                "√3/3" => 3.0_f32.sqrt() / 3.0,
                _ => matches[1].parse().unwrap(),
            };
            let y: f32 = match matches[2].as_str() {
                "√3/3" => 3.0_f32.sqrt() / 3.0,
                _ => matches[2].parse().unwrap(),
            };
            let z: f32 = match matches[3].as_str() {
                "√3/3" => 3.0_f32.sqrt() / 3.0,
                _ => matches[3].parse().unwrap(),
            };

            let expected = Vector::new(x, y, z);

            let actual = world.n;

            assert_eq!(expected.rounded(), actual.rounded());
        };

        then "n = normalize(n)" |world, _step| {
            let expected = world.n;

            let actual = world.n.norm();

            assert_eq!(expected.rounded(), actual.rounded());
        };

        then regex r"^r = vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = match matches[1].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[1].parse().unwrap(),
            };
            let y: f32 = match matches[2].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[2].parse().unwrap(),
            };
            let z: f32 = match matches[3].as_str() {
                "√2/2" => 2.0_f32.sqrt() / 2.0,
                "-√2/2" => -(2.0_f32.sqrt() / 2.0),
                _ => matches[3].parse().unwrap(),
            };

            let expected = Vector::new(x, y, z);

            let actual = world.rv;

            assert_eq!(expected.rounded(), actual.rounded());
        };

        then "light.position = position" |world, _step| {
            assert_eq!(world.position, world.light.position);
        };

        then "light.intensity = intensity" |world, _step| {
            assert_eq!(world.intensity, world.light.intensity);
        };

        then regex r"^m.color = color\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let r: f32 = matches[1].parse().unwrap();
            let g: f32 = matches[2].parse().unwrap();
            let b: f32 = matches[3].parse().unwrap();

            let expected = Color::new(r, g, b);

            let actual = world.mt.color;

            assert_eq!(expected, actual);
        };

        then regex r"^m.ambient = (.*)$" |world, matches, _step| {
            let expected : f32 = matches[1].parse().unwrap();

            let actual = world.mt.ambient;

            assert_eq!(expected, actual);
        };

        then regex r"^m.diffuse = (.*)$" |world, matches, _step| {
            let expected : f32 = matches[1].parse().unwrap();

            let actual = world.mt.diffuse;

            assert_eq!(expected, actual);
        };

        then regex r"^m.specular = (.*)$" |world, matches, _step| {
            let expected : f32 = matches[1].parse().unwrap();

            let actual = world.mt.specular;

            assert_eq!(expected, actual);
        };

        then regex r"^m.shininess = (.*)$" |world, matches, _step| {
            let expected : f32 = matches[1].parse().unwrap();

            let actual = world.mt.shininess;

            assert_eq!(expected, actual);
        };

        then "m = material()" |world, _step| {
            let expected = Material::default();

            let actual = world.s.material;

            assert_eq!(expected, actual);
        };

        then "s.material = m" |world, _step| {
            let expected = world.s.material;

            let actual = world.mt;

            assert_eq!(expected, actual);
        };

        then regex r"^result = color\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let r: f32 = matches[1].parse().unwrap();
            let g: f32 = matches[2].parse().unwrap();
            let b: f32 = matches[3].parse().unwrap();

            let expected = Color::new(r, g, b);

            let actual = world.result;

            assert_eq!(expected.rounded(), actual.rounded());
        };
    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {}

cucumber! {
    features: "./features", // Path to our feature files
    world: crate::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        example_steps::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ],
    after: &[
        an_after_fn // Optional; called after each scenario
    ]
}
