// Derived from example at https://github.com/bbqsrc/cucumber-rust/blob/master/README.md

use std::rc::Rc;

use ndarray::*;

use cucumber::{cucumber, before, after};

use ray_tracer_challenge::*;
use ray_tracer_challenge::color::*;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    rw: RaytracerWorld,
    colors: Vec<Color>,
    matrix: Array<f32, Ix2>,
    matrix_a: Array<f32, Ix2>,
    matrix_b: Array<f32, Ix2>,
    matrix_c: Array<f32, Ix2>,
    matrix_t: Array<f32, Ix2>,
    half_quarter: Array<f32, Ix2>,
    full_quarter: Array<f32, Ix2>,
    transform: Array<f32, Ix2>,
    p: Point,
    p2: Point,
    p3: Point,
    p4: Point,
    origin: Point,
    v: Vector,
    direction: Vector,
    inv: Array<f32, Ix2>,
    tuple: (f32, f32, f32, f32),
    r: Ray,
    s: Sphere,
    xs: Intersections,
    i: Option<Rc<Intersection>>,
    i1: Option<Rc<Intersection>>,
    i2: Option<Rc<Intersection>>,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        let mut rw = RaytracerWorld::new();
        let s = rw.new_sphere(CENTER_ORIGIN);

        // This function is called every time a new scenario is started
        MyWorld {
            rw,
            colors: vec![BLACK; 3],
            matrix: Array::from_elem((4, 4), 0.0),
            matrix_a: Array::from_elem((4, 4), 0.0),
            matrix_b: Array::from_elem((4, 4), 0.0),
            matrix_c: Array::from_elem((4, 4), 0.0),
            matrix_t: Array::from_elem((4, 4), 0.0),
            half_quarter: Array::from_elem((4, 4), 0.0),
            full_quarter: Array::from_elem((4, 4), 0.0),
            transform: Array::from_elem((4, 4), 0.0),
            p: CENTER_ORIGIN,
            p2: CENTER_ORIGIN,
            p3: CENTER_ORIGIN,
            p4: CENTER_ORIGIN,
            origin: CENTER_ORIGIN,
            v: STATIONARY,
            direction: STATIONARY,
            inv: Array::from_elem((4, 4), 0.0),
            tuple: (0.0, 0.0, 0.0, 0.0),
            r: Ray::new(CENTER_ORIGIN, STATIONARY),
            s,
            xs: vec![],
            i: None,
            i1: None,
            i2: None,
        }
    }
}

mod example_steps {
    use std::rc::Rc;
    use std::f32::consts::PI;

    use cucumber::steps;

    use ndarray::*;
    use gherkin;

    use ray_tracer_challenge::*;
    use ray_tracer_challenge::color::*;
    use ray_tracer_challenge::math::*;
    use ray_tracer_challenge::math::transforms::*;

    fn table_to_matrix(table: gherkin::Table, size: (Ix, Ix)) -> Array<f32, Ix2> {
        let mut matrix = Array::from_elem(size, 0.0);

        for (c, value) in table.header.iter().enumerate() {
            matrix[[0,c]] = value.parse().unwrap();
        }

        for (r, row) in table.rows.iter().enumerate() {
            for (c, value) in row.iter().enumerate() {
                matrix[[r + 1,c]] = value.parse().unwrap();
            }
        }

        matrix
    }

    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
        given regex r"the following (.*)x(.*) matrix M:" |world, matches, step| {
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

        given regex r"the following (.*)x(.*) matrix A:" |world, matches, step| {
            let width = matches[1].parse().unwrap();
            let height = matches[2].parse().unwrap();

            let table = step.table().unwrap().clone();

            world.matrix_a = table_to_matrix(table, (width, height));
        };

        given regex r"the following (.*)x(.*) matrix B:" |world, matches, step| {
            let width = matches[1].parse().unwrap();
            let height = matches[2].parse().unwrap();

            let table = step.table().unwrap().clone();

            world.matrix_b = table_to_matrix(table, (width, height));
        };

        given "the following matrix B:" |world, step| {
            let table = step.table().unwrap().clone();

            world.matrix_b = Array::from_elem((4, 4), 0.0);

            for (c, value) in table.header.iter().enumerate() {
                world.matrix_b[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    world.matrix_b[[r + 1,c]] = value.parse().unwrap();
                }
            }
        };

        given "the following matrix A:" |world, step| {
            let table = step.table().unwrap().clone();

            world.matrix_a = Array::from_elem((4, 4), 0.0);

            for (c, value) in table.header.iter().enumerate() {
                world.matrix_a[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    world.matrix_a[[r + 1,c]] = value.parse().unwrap();
                }
            }
        };

        given "C ← A * B" |world, _step| {
            world.matrix_c = world.matrix_a.dot(&world.matrix_b);
        };

        then regex r"M\[(.*),(.*)\] = (.*)" |world, matches, _step| {
            let r: usize = matches[1].parse().unwrap();
            let c: usize = matches[2].parse().unwrap();
            let expected: f32 = matches[3].parse().unwrap();

            assert_eq!(expected, world.matrix[[r, c]]);
        };

        given regex r"c(.*) = color\((.*), (.*), (.*)\)" |world, matches, _step| {
            let color_i: usize = matches[1].parse().unwrap();

            let red: f32 = matches[2].parse().unwrap();
            let green: f32 = matches[3].parse().unwrap();
            let blue: f32 = matches[4].parse().unwrap();

            let color = Color::new(red, green, blue);

            world.colors[color_i] = color;
        };

        given regex r"b ← tuple\((.*), (.*), (.*), (.*)\)" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();
            let t4: f32 = matches[4].parse().unwrap();

            world.tuple = (t1, t2, t3, t4);
        };

        given regex r"B ← submatrix\(A, (.*), (.*)\)" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            world.matrix_b = world.matrix_a.submatrix(row_i, col_i);
        };

        given "B ← inverse(A)" |world, _step| {
            world.matrix_b = world.matrix_a.inverse();
        };

        given regex r"transform ← translation\((.*), (.*), (.*)\)" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();

            world.transform = translation(t1, t2, t3);
        };

        given regex r"transform ← scaling\((.*), (.*), (.*)\)" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();

            world.transform = scaling(t1, t2, t3);
        };

        given regex r"p ← point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.p = Point::new(x, y, z);
        };

        given regex r"v ← vector\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.v = Vector::new(x, y, z);
        };

        given "inv ← inverse(transform)" |world, _step| {
            world.inv = world.transform.inverse();
        };

        given regex r"half_quarter ← rotation_x\(π / (.*)\)" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.half_quarter = rotation_x(PI / denominator);
        };

        given regex r"half_quarter ← rotation_y\(π / (.*)\)" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.half_quarter = rotation_y(PI / denominator);
        };

        given regex r"half_quarter ← rotation_z\(π / (.*)\)" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.half_quarter = rotation_z(PI / denominator);
        };

        given regex r"full_quarter ← rotation_x\(π / (.*)\)" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.full_quarter = rotation_x(PI / denominator);
        };

        given regex r"full_quarter ← rotation_y\(π / (.*)\)" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.full_quarter = rotation_y(PI / denominator);
        };

        given regex r"full_quarter ← rotation_z\(π / (.*)\)" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.full_quarter = rotation_z(PI / denominator);
        };

        given "inv ← inverse(half_quarter)" |world, _step| {
            world.inv = world.half_quarter.inverse();
        };

        given regex r"transform ← shearing\((.*), (.*), (.*), (.*), (.*), (.*)\)" |world, matches, _step| {
            let xy: f32 = matches[1].parse().unwrap();
            let xz: f32 = matches[2].parse().unwrap();
            let yx: f32 = matches[3].parse().unwrap();
            let yz: f32 = matches[4].parse().unwrap();
            let zx: f32 = matches[5].parse().unwrap();
            let zy: f32 = matches[6].parse().unwrap();

            world.transform = shearing(xy, xz, yx, yz, zx, zy);
        };

        given regex r"A ← rotation_x\(π / (.*)\)" |world, matches, _step| {
            let denominator: f32 = matches[1].parse().unwrap();

            world.matrix_a = rotation_x(PI / denominator);
        };

        given regex r"B ← scaling\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.matrix_b = scaling(x, y, z);
        };

        given regex r"C ← translation\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.matrix_c = translation(x, y, z);
        };

        given regex r"origin ← point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.origin = Point::new(x, y, z);
        };

        given regex r"direction ← vector\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.direction = Vector::new(x, y, z);
        };

        given regex r"r ← ray\(point\((.*), (.*), (.*)\), vector\((.*), (.*), (.*)\)\)" |world, matches, _step| {
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

        given regex r"i1 ← intersection\((.*), s\)" |world, matches, _step| {
            let time: f32 = matches[1].parse().unwrap();

            world.i1 = Some(Rc::new(Intersection { time, object: Rc::new(world.s) }));
        };

        given regex r"i2 ← intersection\((.*), s\)" |world, matches, _step| {
            let time: f32 = matches[1].parse().unwrap();

            world.i2 = Some(Rc::new(Intersection { time, object: Rc::new(world.s) }));
        };

        when "p2 ← A * p" |world, _step| {
            world.p2 = &world.matrix_a * world.p;
        };

        when "p3 ← B * p2" |world, _step| {
            world.p3 = &world.matrix_b * world.p2;
        };

        when "p4 ← C * p3" |world, _step| {
            world.p4 = &world.matrix_c * world.p3;
        };

        when "T ← C * B * A" |world, _step| {
            world.matrix_t = world.matrix_c.dot(&world.matrix_b.dot(&world.matrix_a));
        };

        when "r ← ray(origin, direction)" |world, _step| {
            world.r = Ray::new(world.origin, world.direction);
        };

        when "xs ← intersect(s, r)" |world, _step| {
            world.xs = world.s.intersections_with(world.r);
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

        // TODO: Move this to up with the other given rules
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

        when regex r"i ← intersection\((.*), s\)" |world, matches, _step| {
            let time: f32 = matches[1].parse().unwrap();

            world.i = Some(Rc::new(Intersection { time, object: Rc::new(world.s) }));
        };

        when "i ← hit(xs)" |world, _step| {
            world.i = world.xs.hit();
        };

        then regex r"c(.*) \+ c(.*) = color\((.*), (.*), (.*)\)" |world, matches, _step| {
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

        then regex r"A \* b = tuple\((.*), (.*), (.*), (.*)\)" |world, matches, _step| {
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

            let actual_matrix = world.matrix_a.dot(&world.matrix_b);

            assert_eq!(expected_matrix, actual_matrix);
        };

        then "A * identity_matrix = A" |world, _step| {
            let identity_matrix: Array<f32, Ix2> = Array::eye(4);

            assert_eq!(world.matrix_a.dot(&identity_matrix), world.matrix_a)
        };

        then "transpose(A) is the following matrix:" |world, step| {
            let table = step.table().unwrap().clone();

            let transposed = world.matrix_a.t();

            let expected = table_to_matrix(table, (4, 4));

            assert_eq!(expected, transposed);
        };

        then regex r"determinant\(A\) = (.*)" |world, matches, _step| {
            let expected: f32 = matches[1].parse().unwrap();

            let actual = world.matrix_a.determinant();

            assert_eq!(expected, actual);
        };

        then regex r"determinant\(B\) = (.*)" |world, matches, _step| {
            let expected: f32 = matches[1].parse().unwrap();

            let actual = world.matrix_b.determinant();

            assert_eq!(expected, actual);
        };

        then regex r"minor\(A, (.*), (.*)\) = (.*)" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let expected: f32 = matches[3].parse().unwrap();

            let actual = world.matrix_a.minor(row_i, col_i);

            assert_eq!(expected, actual);
        };

        then regex r"cofactor\(A, (.*), (.*)\) = (.*)" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let expected: f32 = matches[3].parse().unwrap();

            let actual = world.matrix_a.cofactor(row_i, col_i);

            assert_eq!(expected, actual);
        };

        then regex r"submatrix\(A, (.*), (.*)\) is the following (.*)x(.*) matrix" |world, matches, step| {
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

        then regex r"B\[(.*),(.*)\] = (.*)/(.*)" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let numerator: f32 = matches[3].parse().unwrap();
            let denominator: f32 = matches[4].parse().unwrap();

            let expected: f32 = (numerator as f32) / (denominator as f32);

            let actual = world.matrix_b[[row_i, col_i]] as f32;

            assert_eq!(expected, actual);
        };

        then regex r"B is the following (.*)x(.*) matrix" |world, matches, step| {
            let width = matches[1].parse().unwrap();
            let height = matches[2].parse().unwrap();

            let table = step.table().unwrap().clone();

            let expected = table_to_matrix(table, (width, height));

            assert_eq!(expected, world.matrix_b.rounded());
        };

        then regex r"inverse\(A\) is the following (.*)x(.*) matrix" |world, matches, step| {
            let width = matches[1].parse().unwrap();
            let height = matches[2].parse().unwrap();

            let table = step.table().unwrap().clone();

            let expected = table_to_matrix(table, (width, height));

            assert_eq!(expected, world.matrix_a.inverse().rounded());
        };

        then "C * inverse(B) = A" |world, _step| {
            assert_eq!(world.matrix_a, world.matrix_c.dot(&world.matrix_b.inverse()).rounded());
        };

        then regex r"transform \* p = point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = &world.transform * world.p;

            assert_eq!(expected, actual);
        };

        then regex r"inv \* p = point\((.*), (.*), (.*)\)" |world, matches, _step| {
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

            let actual = &world.transform * world.v;

            assert_eq!(expected, actual);
        };

        then regex r"transform \* v = vector\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Vector::new(x, y, z);

            let actual = &world.transform * world.v;

            assert_eq!(expected, actual);
        };

        then regex r"inv \* v = vector\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Vector::new(x, y, z);

            let actual = &world.inv * world.v;

            assert_eq!(expected, actual);
        };

        then regex r"half_quarter \* p = point\((.*), (.*), (.*)\)" |world, matches, _step| {
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

            let actual = &world.half_quarter * world.p;

            assert_eq!(expected, actual);
        };

        then regex r"full_quarter \* p = point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = &world.full_quarter * world.p;

            assert_eq!(expected, actual.rounded());
        };

        then regex r"p2 = point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.p2;

            assert_eq!(expected, actual.rounded());
        };

        then regex r"p3 = point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.p3;

            assert_eq!(expected, actual.rounded());
        };

        then regex r"p4 = point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.p4;

            assert_eq!(expected, actual.rounded());
        };

        then regex r"T \* p = point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = &world.matrix_t * world.p;

            assert_eq!(expected, actual.rounded());
        };

        then "r.origin = origin" |world, _step| {
            assert_eq!(world.origin, world.r.origin);
        };

        then "r.direction = direction" |world, _step| {
            assert_eq!(world.direction, world.r.direction);
        };

        then regex r"position\(r, (.*)\) = point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let t: f32 = matches[1].parse().unwrap();
            let x: f32 = matches[2].parse().unwrap();
            let y: f32 = matches[3].parse().unwrap();
            let z: f32 = matches[4].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.r.position(t);

            assert_eq!(expected, actual.rounded());
        };

        then regex r"xs.count = (.*)" |world, matches, _step| {
            let count: usize = matches[1].parse().unwrap();

            let expected = count;

            let actual = world.xs.len();

            assert_eq!(expected, actual);
        };

        then regex r"xs\[(.*)\].t = (.*)" |world, matches, _step| {
            let index: usize = matches[1].parse().unwrap();
            let time: f32 = matches[2].parse().unwrap();

            let expected = time;

            let actual = world.xs[index].time;

            assert_eq!(expected, actual);
        };

        then regex r"xs\[(.*)\].object = s" |world, matches, _step| {
            let index: usize = matches[1].parse().unwrap();

            let expected = world.s.id();

            let actual = world.xs[index].object.id();

            assert_eq!(expected, actual);
        };

        then regex r"i.t = (.*)" |world, matches, _step| {
            let time: f32 = matches[1].parse().unwrap();

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

        then "i is nothing" |world, _step| {
            match world.i.as_ref() {
                Some(i) => panic!("world.i should have been None but it was {:?}", i),
                _ => assert!(true),
            };
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
fn setup() {
}

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
