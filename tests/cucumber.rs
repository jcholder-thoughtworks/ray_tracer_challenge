// Derived from example at https://github.com/bbqsrc/cucumber-rust/blob/master/README.md

#![recursion_limit = "256"]

use std::rc::Rc;

use cucumber::{after, before, cucumber};

use ray_tracer_challenge::canvas::*;
use ray_tracer_challenge::color::*;
use ray_tracer_challenge::light::*;
use ray_tracer_challenge::material::*;
use ray_tracer_challenge::math::*;
use ray_tracer_challenge::math::transforms::*;
use ray_tracer_challenge::objects::*;
use ray_tracer_challenge::*;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    rw: RaytracerWorld,
    colors: Vec<Color>,
    matrix: AnyMatrix,
    matrix_a: Rc<AnyMatrix>,
    matrix_b: Rc<AnyMatrix>,
    matrix_c: Rc<AnyMatrix>,
    matrix_t: Rc<AnyMatrix>,
    half_quarter: Rc<TransformationMatrix>,
    full_quarter: Rc<TransformationMatrix>,
    transform: Rc<TransformationMatrix>,
    m: Rc<TransformationMatrix>,
    t: Rc<TransformationMatrix>,
    p: Point,
    p2: Point,
    p3: Point,
    p4: Point,
    origin: Point,
    v: Vector,
    direction: Vector,
    n: Vector,
    inv: TransformationMatrix,
    tuple: (f32, f32, f32, f32),
    r: Ray,
    rv: Vector,
    r2: Ray,
    s: RaytracerObject,
    s1: RaytracerObject,
    s2: RaytracerObject,
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
    shape: Rc<RaytracerObject>,
    comps: Option<PrecomputedHit>,
    c: Color,
    from: Point,
    to: Point,
    up: Vector,
    hsize: f32,
    vsize: f32,
    field_of_view: f32,
    camera: Camera,
    image: Canvas,
    in_shadow: bool,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        let mut rw = RaytracerWorld::default();
        let s = rw.new_sphere(CENTER_ORIGIN);
        let s1 = rw.new_sphere(CENTER_ORIGIN);
        let s2 = rw.new_sphere(CENTER_ORIGIN);
        let shape = Rc::new(rw.new_sphere(CENTER_ORIGIN));

        // This function is called every time a new scenario is started
        MyWorld {
            rw,
            colors: vec![BLACK; 3],
            // TODO: Consider using Option here to make it more obvious when they haven't been set
            matrix: AnyMatrix::M4x4(TransformationMatrix::default()),
            matrix_a: Rc::new(AnyMatrix::M4x4(TransformationMatrix::default())),
            matrix_b: Rc::new(AnyMatrix::M4x4(TransformationMatrix::default())),
            matrix_c: Rc::new(AnyMatrix::M4x4(TransformationMatrix::default())),
            matrix_t: Rc::new(AnyMatrix::M4x4(TransformationMatrix::default())),
            half_quarter: Rc::new(TransformationMatrix::default()),
            full_quarter: Rc::new(TransformationMatrix::default()),
            transform: Rc::new(TransformationMatrix::default()),
            m: Rc::new(TransformationMatrix::default()),
            t: Rc::new(TransformationMatrix::default()),
            p: CENTER_ORIGIN,
            p2: CENTER_ORIGIN,
            p3: CENTER_ORIGIN,
            p4: CENTER_ORIGIN,
            origin: CENTER_ORIGIN,
            v: STATIONARY,
            direction: STATIONARY,
            n: STATIONARY,
            inv: TransformationMatrix::default(),
            tuple: (0.0, 0.0, 0.0, 0.0),
            r: Ray::new(CENTER_ORIGIN, STATIONARY),
            rv: STATIONARY,
            r2: Ray::new(CENTER_ORIGIN, STATIONARY),
            s,
            s1,
            s2,
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
            shape,
            comps: None,
            c: BLACK,
            from: CENTER_ORIGIN,
            to: CENTER_ORIGIN,
            up: STATIONARY,
            hsize: 0.0,
            vsize: 0.0,
            field_of_view: 0.0,
            camera: Camera::new(0.0, 0.0, 0.0),
            image: Canvas::new(0, 0),
            in_shadow: false,
        }
    }
}

mod example_steps {
    use std::f32::consts::PI;
    use std::rc::Rc;

    use regex::Regex;

    use cucumber::steps;

    use gherkin;

    use ray_tracer_challenge::color::*;
    use ray_tracer_challenge::light::*;
    use ray_tracer_challenge::material::*;
    use ray_tracer_challenge::math::transforms::*;
    use ray_tracer_challenge::math::*;
    use ray_tracer_challenge::objects::*;
    use ray_tracer_challenge::*;

    #[allow(dead_code)]
    fn m2x2(any: &Rc<AnyMatrix>) -> Matrix2x2 {
        match **any {
            AnyMatrix::M2x2(m) => m,
            _ => panic!("Expected matrix to be a Matrix2x2"),
        }.clone()
    }

    fn m3x3(any: &Rc<AnyMatrix>) -> Matrix3x3 {
        match **any {
            AnyMatrix::M3x3(m) => m,
            _ => panic!("Expected matrix to be a Matrix3x3"),
        }.clone()
    }

    fn m4x4(any: &Rc<AnyMatrix>) -> Matrix4x4 {
        match **any {
            AnyMatrix::M4x4(m) => m,
            _ => panic!("Expected matrix to be a Matrix4x4"),
        }.clone()
    }

    fn table_to_matrix2x2(table: gherkin::Table) -> Matrix2x2 {
        let mut matrix = Matrix2x2::default();

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

    fn table_to_matrix3x3(table: gherkin::Table) -> Matrix3x3 {
        let mut matrix = Matrix3x3::default();

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

    fn table_to_matrix4x4(table: gherkin::Table) -> Matrix4x4 {
        let mut matrix = Matrix4x4::default();

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
        given "the following 2x2 matrix M:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut matrix = Matrix2x2::default();

            for (c, value) in table.header.iter().enumerate() {
                matrix[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    matrix[[r + 1,c]] = value.parse().unwrap();
                }
            }

            world.matrix = AnyMatrix::M2x2(matrix);
        };

        given "the following 3x3 matrix M:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut matrix = Matrix3x3::default();

            for (c, value) in table.header.iter().enumerate() {
                matrix[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    matrix[[r + 1,c]] = value.parse().unwrap();
                }
            }

            world.matrix = AnyMatrix::M3x3(matrix);
        };

        given "the following 4x4 matrix M:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut matrix = Matrix4x4::default();

            for (c, value) in table.header.iter().enumerate() {
                matrix[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    matrix[[r + 1,c]] = value.parse().unwrap();
                }
            }

            world.matrix = AnyMatrix::M4x4(matrix);
        };

        given "the following 2x2 matrix A:" |world, step| {
            let table = step.table().unwrap().clone();

            world.matrix_a = Rc::new(AnyMatrix::M2x2(table_to_matrix2x2(table)));
        };

        given "the following 3x3 matrix A:" |world, step| {
            let table = step.table().unwrap().clone();

            world.matrix_a = Rc::new(AnyMatrix::M3x3(table_to_matrix3x3(table)));
        };

        given "the following 4x4 matrix A:" |world, step| {
            let table = step.table().unwrap().clone();

            world.matrix_a = Rc::new(AnyMatrix::M4x4(table_to_matrix4x4(table)));
        };

        given "the following 4x4 matrix B:" |world, step| {
            let table = step.table().unwrap().clone();

            let matrix = table_to_matrix4x4(table);

            world.matrix_b = Rc::new(AnyMatrix::M4x4(matrix));
        };

        given "the following matrix B:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut array = Matrix4x4::default();

            for (c, value) in table.header.iter().enumerate() {
                array[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    array[[r + 1,c]] = value.parse().unwrap();
                }
            }

            world.matrix_b = Rc::new(AnyMatrix::M4x4(array));
        };

        given "the following matrix A:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut array = Matrix4x4::default();

            for (c, value) in table.header.iter().enumerate() {
                array[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    array[[r + 1,c]] = value.parse().unwrap();
                }
            }

            world.matrix_a = Rc::new(AnyMatrix::M4x4(array));
        };

        given "C ← A * B" |world, _step| {
            let ma = match *world.matrix_a {
                AnyMatrix::M4x4(m) => m,
                _ => panic!("Expected world.matrix_a to be a Matrix4x4"),
            };
            let mb = match *world.matrix_b {
                AnyMatrix::M4x4(m) => m,
                _ => panic!("Expected world.matrix_b to be a Matrix4x4"),
            };
            world.matrix_c = Rc::new(AnyMatrix::M4x4(ma * mb));
        };

        then regex r"^M\[(.*),(.*)\] = (.*)$" |world, matches, _step| {
            let r: usize = matches[1].parse().unwrap();
            let c: usize = matches[2].parse().unwrap();

            let expected: f32 = matches[3].parse().unwrap();

            let actual = match world.matrix {
                AnyMatrix::M2x2(m) => m[[r, c]],
                AnyMatrix::M3x3(m) => m[[r, c]],
                AnyMatrix::M4x4(m) => m[[r, c]],
                AnyMatrix::M4x1(_m) => unimplemented!("Did not expect a 4x1 matrix"),
            };

            assert_eq!(expected, actual);
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

            let ma = m3x3(&world.matrix_a);
            let sma = ma.submatrix(row_i, col_i);

            world.matrix_b = Rc::new(AnyMatrix::M2x2(sma));
        };

        given "B ← inverse(A)" |world, _step| {
            world.matrix_b = Rc::new(AnyMatrix::M4x4(m4x4(&world.matrix_a).inverse()));
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

            world.m = Rc::new(translation(t1, t2, t3).into());
        };

        given regex r"^m ← scaling\(([0-9.]*), ([0-9.]*), ([0-9.]*)\)$" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();

            world.m = Rc::new(scaling(t1, t2, t3).into());
        };

        given regex r"^m ← scaling\(([0-9.]*), ([0-9.]*), ([0-9.]*)\) \* rotation_z\(π/5\)$" |world, matches, _step| {
            let t1: f32 = matches[1].parse().unwrap();
            let t2: f32 = matches[2].parse().unwrap();
            let t3: f32 = matches[3].parse().unwrap();

            let st = scaling(t1, t2, t3);
            let rzt = rotation_z(PI / 5.0);

            world.m = Rc::new((st * rzt).into());
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

            world.matrix_a = Rc::new(AnyMatrix::M4x4(rotation_x(PI / denominator)));
        };

        given regex r"^B ← scaling\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.matrix_b = Rc::new(AnyMatrix::M4x4(scaling(x, y, z)));
        };

        given regex r"^C ← translation\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.matrix_c = Rc::new(AnyMatrix::M4x4(translation(x, y, z)));
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

        given "s1 ← sphere()" |world, _step| {
            world.s1 = world.rw.new_sphere(CENTER_ORIGIN);
        };

        given "s1 is added to w" |world, _step| {
            world.rw.add_object(world.s1);
        };

        given "s2 is added to w" |world, _step| {
            world.rw.add_object(world.s2);
        };

        given regex r"^i1 ← intersection\((.*), s\)$" |world, matches, _step| {
            let time: Time = matches[1].parse().unwrap();

            let object = Rc::new(world.s.clone());

            world.i1 = Some(Rc::new(Intersection { time, object }));
        };

        given regex r"^i ← intersection\((.*), s2\)$" |world, matches, _step| {
            let time: Time = matches[1].parse().unwrap();

            let object = Rc::new(world.s2.clone());

            world.i = Some(Rc::new(Intersection { time, object }));
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
            world.s.transform = *world.m;
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

        given regex r"^light ← point_light\(point\((.*), (.*), (.*)\), color\((.*), (.*), (.*)\)\)$" |world, matches, _step| {
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

        given regex r"^w\.light ← point_light\(point\((.*), (.*), (.*)\), color\((.*), (.*), (.*)\)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let position = Point::new(x, y, z);

            let red: f32 = matches[4].parse().unwrap();
            let green: f32 = matches[5].parse().unwrap();
            let blue: f32 = matches[6].parse().unwrap();

            let intensity = Color::new(red, green, blue);

            world.rw.light = Some(Light::new(position, intensity));
        };

        given "w ← world()" |world, _step| {
            world.rw = RaytracerWorld::new();
        };

        given "s1 ← sphere() with:" |world, step| {
            let table = step.table().unwrap().clone();

            let color_regex = Regex::new(r"^\((.*), (.*), (.*)\)$").unwrap();
            let color_captures = color_regex.captures(&table.header[1]).unwrap();
            let r = color_captures.get(1).unwrap().as_str().parse().unwrap();
            let g = color_captures.get(2).unwrap().as_str().parse().unwrap();
            let b = color_captures.get(3).unwrap().as_str().parse().unwrap();

            let color = Color::new(r, g, b);

            let diffuse: f32 = table.rows[0][1].parse().unwrap();
            let specular: f32 = table.rows[1][1].parse().unwrap();

            world.s1.material.color = color;
            world.s1.material.diffuse = diffuse;
            world.s1.material.specular = specular;
        };

        given "s2 ← sphere() with:" |world, step| {
            let table = step.table().unwrap().clone();

            let regex = Regex::new(r"^(translation|scaling)\((.*), (.*), (.*)\)$").unwrap();
            let captures = regex.captures(&table.rows[0][1]).unwrap();
            let transformation_type: &str = captures.get(1).unwrap().as_str();
            let x: f32 = captures.get(2).unwrap().as_str().parse().unwrap();
            let y: f32 = captures.get(3).unwrap().as_str().parse().unwrap();
            let z: f32 = captures.get(4).unwrap().as_str().parse().unwrap();

            let transform = match transformation_type {
                "scaling" => scaling(x, y, z),
                "translation" => translation(x, y, z),
                _ => unimplemented!("Missing support for transformation of type {}", transformation_type),
            };

            world.s2.transform = transform;
        };

        given "shape ← sphere() with:" |world, step| {
            let table = step.table().unwrap().clone();

            let regex = Regex::new(r"^(translation|scaling)\((.*), (.*), (.*)\)$").unwrap();
            let captures = regex.captures(&table.rows[0][1]).unwrap();
            let transformation_type: &str = captures.get(1).unwrap().as_str();
            let x: f32 = captures.get(2).unwrap().as_str().parse().unwrap();
            let y: f32 = captures.get(3).unwrap().as_str().parse().unwrap();
            let z: f32 = captures.get(4).unwrap().as_str().parse().unwrap();

            let transform = match transformation_type {
                "scaling" => scaling(x, y, z),
                "translation" => translation(x, y, z),
                _ => unimplemented!("Missing support for transformation of type {}", transformation_type),
            };

            let mut shape = world.rw.new_sphere(CENTER_ORIGIN);
            shape.transform = transform;

            world.shape = Rc::new(shape);
        };

        given "w ← default_world()" |world, _step| {
            world.rw = RaytracerWorld::default();
        };

        given "shape ← sphere()" |world, _step| {
            world.shape = Rc::new(world.rw.new_sphere(CENTER_ORIGIN));
        };

        given regex r"^i ← intersection\((.*), shape\)$" |world, matches, _step| {
            let time: Time = matches[1].parse().unwrap();

            let object = Rc::clone(&world.shape);

            world.i = Some(Rc::new(Intersection { time, object }));
        };

        given "shape ← the first object in w" |world, _step| {
            world.shape = Rc::clone(&world.rw.objects().first().unwrap());
        };

        given "shape ← the second object in w" |world, _step| {
            world.shape = Rc::clone(&world.rw.objects().get(1).unwrap());
        };

        given "outer ← the first object in w" |_world, _step| {
            // Seemed like too much effort to make this assignment approach 
            // work with Rust's ownership system so skipping it
        };

        given "inner ← the second object in w" |_world, _step| {
            // Seemed like too much effort to make this assignment approach 
            // work with Rust's ownership system so skipping it
        };

        given "outer.material.ambient ← 1" |world, _step| {
            world.rw.get_object_mut(0).material.ambient = 1.0;
        };

        given "inner.material.ambient ← 1" |world, _step| {
            world.rw.get_object_mut(1).material.ambient = 1.0;
        };

        given regex r"^from ← point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.from = Point::new(x, y, z);
        };

        given regex r"^to ← point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.to = Point::new(x, y, z);
        };

        given regex r"^up ← vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            world.up = Vector::new(x, y, z);
        };

        given "hsize ← 160" |world, _step| {
            world.hsize = 160.0;
        };

        given "vsize ← 120" |world, _step| {
            world.vsize = 120.0;
        };

        given "field_of_view ← π/2" |world, _step| {
            world.field_of_view = PI / 2.0;
        };

        given regex r"^c ← camera\((.*), (.*), π/2\)$" |world, matches, _step| {
            let hsize: f32 = matches[1].parse().unwrap();
            let vsize: f32 = matches[2].parse().unwrap();
            let field_of_view: f32 = PI / 2.0;

            world.camera = Camera::new(hsize, vsize, field_of_view);
        };

        given "c.transform ← view_transform(from, to, up)" |world, _step| {
            world.camera.transform = view_transform(&world.from, &world.to, &world.up);
        };

        given "in_shadow ← true" |world, _step| {
            world.in_shadow = true;
        };

        when "p2 ← A * p" |world, _step| {
            world.p2 = m4x4(&world.matrix_a) * world.p;
        };

        when "p3 ← B * p2" |world, _step| {
            world.p3 = m4x4(&world.matrix_b) * world.p2;
        };

        when "p4 ← C * p3" |world, _step| {
            world.p4 = m4x4(&world.matrix_c) * world.p3;
        };

        when "T ← C * B * A" |world, _step| {
            let product = m4x4(&world.matrix_c) * m4x4(&world.matrix_b) * m4x4(&world.matrix_a);

            world.matrix_t = Rc::new(AnyMatrix::M4x4(product));
        };

        when "r ← ray(origin, direction)" |world, _step| {
            world.r = Ray::new(world.origin, world.direction);
        };

        when "xs ← intersect(s, r)" |world, _step| {
            let sphere_rc: Rc<RaytracerObject> = Rc::new(world.s.clone());

            world.xs = intersect(sphere_rc, &world.r);
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
            let m: TransformationMatrix = (*world.m).into();
            world.r2 = world.r.transform(&m);
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
            world.result = world.mt.lighting(world.light, world.position, world.eyev, world.normalv, world.in_shadow);
        };

        when "result ← lighting(m, light, position, eyev, normalv, in_shadow)" |world, _step| {
            world.result = world.mt.lighting(world.light, world.position, world.eyev, world.normalv, world.in_shadow);
        };

        when "w ← default_world()" |world, _step| {
            world.rw = RaytracerWorld::default();
        };

        when "xs ← intersect_world(w, r)" |world, _step| {
            world.xs = world.rw.intersect(&world.r);
        };

        when "comps ← prepare_computations(i, r)" |world, _step| {
            world.comps = Some(world.i.as_ref().unwrap().prepare_computations(&world.r));
        };

        when "c ← shade_hit(w, comps)" |world, _step| {
            world.c = world.rw.shade_hit(world.comps.as_ref().unwrap());
        };

        when "c ← color_at(w, r)" |world, _step| {
            world.c = world.rw.color_at(&world.r);
        };

        when "t ← view_transform(from, to, up)" |world, _step| {
            world.t = Rc::new(view_transform(&world.from, &world.to, &world.up));
        };

        when "c ← camera(hsize, vsize, field_of_view)" |world, _step| {
            world.camera = Camera::new(world.hsize, world.vsize, world.field_of_view);
        };

        when regex r"^r ← ray_for_pixel\(c, (.*), (.*)\)$" |world, matches, _step| {
            let x: usize = matches[1].parse().unwrap();
            let y: usize = matches[2].parse().unwrap();

            world.r = world.camera.ray_for_pixel(x, y);
        };

        when "c.transform ← rotation_y(π/4) * translation(0, -2, 5)" |world, _step| {
            world.camera.transform = rotation_y(PI/4.0) * translation(0.0, -2.0, 5.0);
        };

        when "image ← render(c, w)" |world, _step| {
            world.image = world.camera.render(&world.rw);
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

            let tuple_matrix = Matrix4x1::new([
                world.tuple.0,
                world.tuple.1,
                world.tuple.2,
                world.tuple.3,
            ]);

            let multiplied_matrix = m4x4(&world.matrix_a) * tuple_matrix;

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

            let mut expected_matrix = Matrix4x4::default();

            for (c, value) in table.header.iter().enumerate() {
                expected_matrix[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    expected_matrix[[r + 1,c]] = value.parse().unwrap();
                }
            }

            let actual_matrix = m4x4(&world.matrix_a) * m4x4(&world.matrix_b);

            assert_eq!(expected_matrix, actual_matrix);
        };

        then "A * identity_matrix = A" |world, _step| {
            let expected = m4x4(&world.matrix_a) * Matrix4x4::identity();

            // TODO: Not an ideal cloning. How to make this more efficient?
            let actual = m4x4(&world.matrix_a);

            assert_eq!(expected, actual);
        };

        then "transpose(A) is the following matrix:" |world, step| {
            let table = step.table().unwrap().clone();

            let expected: Matrix4x4 = table_to_matrix4x4(table);

            let actual = m4x4(&world.matrix_a).transposed();

            assert_eq!(expected, actual);
        };

        then regex r"^determinant\(A\) = (.*)$" |world, matches, _step| {
            let expected: f32 = matches[1].parse().unwrap();

            let actual = match *world.matrix_a {
                AnyMatrix::M2x2(m) => m.determinant(),
                AnyMatrix::M3x3(m) => m.determinant(),
                AnyMatrix::M4x4(m) => m.determinant(),
                AnyMatrix::M4x1(_m) => unimplemented!("need Matrix4x1#cofactor"),
            };

            assert_eq!(expected, actual);
        };

        then regex r"^determinant\(B\) = (.*)$" |world, matches, _step| {
            let expected: f32 = matches[1].parse().unwrap();

            let actual = match *world.matrix_b {
                AnyMatrix::M2x2(m) => m.determinant(),
                AnyMatrix::M3x3(m) => m.determinant(),
                AnyMatrix::M4x4(m) => m.determinant(),
                AnyMatrix::M4x1(_m) => unimplemented!("need Matrix4x1#cofactor"),
            };

            assert_eq!(expected, actual);
        };

        then regex r"^minor\(A, (.*), (.*)\) = (.*)$" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let expected: f32 = matches[3].parse().unwrap();

            let actual = match *world.matrix_a {
                AnyMatrix::M2x2(_m) => unimplemented!("need Matrix2x2#cofactor"),
                AnyMatrix::M3x3(m) => m.minor(row_i, col_i),
                AnyMatrix::M4x4(m) => m.minor(row_i, col_i),
                AnyMatrix::M4x1(_m) => unimplemented!("need Matrix4x1#cofactor"),
            };

            assert_eq!(expected, actual);
        };

        then regex r"^cofactor\(A, (.*), (.*)\) = (.*)$" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let expected: f32 = matches[3].parse().unwrap();

            let actual = match *world.matrix_a {
                AnyMatrix::M2x2(_m) => unimplemented!("need Matrix2x2#cofactor"),
                AnyMatrix::M3x3(m) => m.cofactor(row_i, col_i),
                AnyMatrix::M4x4(m) => m.cofactor(row_i, col_i),
                AnyMatrix::M4x1(_m) => unimplemented!("need Matrix4x1#cofactor"),
            };

            assert_eq!(expected, actual);
        };

        then regex r"^submatrix\(A, (.*), (.*)\) is the following 2x2 matrix:$" |world, matches, step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let expected: Matrix2x2 = table_to_matrix2x2(step.table().unwrap().clone());

            let actual = m3x3(&world.matrix_a).submatrix(row_i, col_i);

            assert_eq!(expected, actual);
        };

        then regex r"^submatrix\(A, (.*), (.*)\) is the following 3x3 matrix:$" |world, matches, step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let expected: Matrix3x3 = table_to_matrix3x3(step.table().unwrap().clone());

            let actual = m4x4(&world.matrix_a).submatrix(row_i, col_i);

            assert_eq!(expected, actual);
        };

        then "A is invertible" |world, _step| {
            assert!(m4x4(&world.matrix_a).invertible());
        };

        then "A is not invertible" |world, _step| {
            assert!(!m4x4(&world.matrix_a).invertible());
        };

        then regex r"^B\[(.*),(.*)\] = (.*)/(.*)$" |world, matches, _step| {
            let row_i: usize = matches[1].parse().unwrap();
            let col_i: usize = matches[2].parse().unwrap();

            let numerator: f32 = matches[3].parse().unwrap();
            let denominator: f32 = matches[4].parse().unwrap();

            let mb = match *world.matrix_b {
                AnyMatrix::M4x4(m) => m,
                _ => panic!("Expected world.matrix_b to be a Matrix4x4"),
            };

            let expected: f32 = (numerator as f32) / (denominator as f32);

            let actual = mb[[row_i, col_i]] as f32;

            assert_eq!(expected, actual);
        };

        then "B is the following 4x4 matrix:" |world, step| {
            let table = step.table().unwrap().clone();

            let mb = match *world.matrix_b {
                AnyMatrix::M4x4(m) => m,
                _ => panic!("Expected world.matrix_b to be a Matrix4x4"),
            };

            let expected: Matrix4x4 = table_to_matrix4x4(table);

            assert_eq!(expected, mb.rounded());
        };

        then "inverse(A) is the following 4x4 matrix:" |world, step| {
            let table = step.table().unwrap().clone();

            let ma = match *world.matrix_a {
                AnyMatrix::M4x4(m) => m,
                _ => panic!("Expected world.matrix_a to be a Matrix4x4"),
            };

            let expected: Matrix4x4 = table_to_matrix4x4(table);

            assert_eq!(expected, ma.inverse().rounded());
        };

        then "C * inverse(B) = A" |world, _step| {
            let ma = match *world.matrix_a {
                AnyMatrix::M4x4(m) => m,
                _ => panic!("Expected world.matrix_a to be a Matrix4x4"),
            };
            let mb = match *world.matrix_b {
                AnyMatrix::M4x4(m) => m,
                _ => panic!("Expected world.matrix_b to be a Matrix4x4"),
            };
            let mc = match *world.matrix_c {
                AnyMatrix::M4x4(m) => m,
                _ => panic!("Expected world.matrix_b to be a Matrix4x4"),
            };

            let expected = ma;

            let actual = (mc * mb.inverse()).rounded();

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

            let mt = m4x4(&world.matrix_t);

            let actual = mt * world.p;

            assert_eq!(expected, actual.rounded());
        };

        then "r.origin = origin" |world, _step| {
            assert_eq!(world.origin, world.r.origin);
        };

        then regex r"^r.origin = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            assert_eq!(expected.rounded(), world.r.origin.rounded());
        };

        then regex r"^r.direction = vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
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

            assert_eq!(expected.rounded(), world.r.direction.rounded());
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
            let expected = TransformationMatrix::identity();

            let actual = world.s.transform;

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

        then "w contains no objects" |world, _step| {
            let expected: Vec<Rc<RaytracerObject>> = vec![];
            let actual = world.rw.objects();

            assert_eq!(expected, actual);
        };

        then "w has no light source" |world, _step| {
            let expected: Option<Light> = None;
            let actual = world.rw.light;

            assert_eq!(expected, actual);
        };

        then "w.light = light" |world, _step| {
            let expected = Some(world.light);
            let actual = world.rw.light;

            assert_eq!(expected, actual);
        };

        then regex r"^w contains (.*)$" |world, matches, _step| {
            let expected_obj_name = matches[1].as_str();

            let expected = match expected_obj_name {
                "s1" => world.s1.clone(),
                "s2" => world.s2.clone(),
                _ => panic!("Unrecognized object name: {}", expected_obj_name),
            };

            let equivalent_obj = |o: &&Rc<RaytracerObject>| -> bool {
                let obj: RaytracerObject = o.as_ref().clone();

                obj.obj_type == expected.obj_type
                    && obj.origin == expected.origin
                    && obj.transform == expected.transform
                    && obj.material == expected.material
            };

            let found_match = world.rw.objects().iter().find(equivalent_obj).is_some();

            assert!(found_match, "Expected to find {:?} in {:?} but didn't", expected, world.rw.objects());
        };

        then "comps.t = i.t" |world, _step| {
            let expected = world.comps.as_ref().unwrap().time;
            let actual = world.i.as_ref().unwrap().time;

            assert_eq!(expected, actual);
        };

        then "comps.object = i.object" |world, _step| {
            let expected = world.i.as_ref().unwrap().object.as_ref().clone();
            let actual = world.comps.as_ref().unwrap().object.as_ref().clone();

            assert_eq!(expected, actual);
        };

        then regex r"^comps.point = point\((.*), (.*), (.*)\)$" |world, matches, _step| {
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
            let actual = world.comps.as_ref().unwrap().point;

            assert_eq!(expected, actual);
        };

        then regex r"^comps.eyev = vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
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
            let actual = world.comps.as_ref().unwrap().eyev;

            assert_eq!(expected, actual);
        };

        then regex r"^comps.normalv = vector\((.*), (.*), (.*)\)$" |world, matches, _step| {
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
            let actual = world.comps.as_ref().unwrap().normalv;

            assert_eq!(expected, actual);
        };

        then regex r"^comps.inside = (.*)$" |world, matches, _step| {
            let expected: bool = matches[1].parse().unwrap();

            let actual = world.comps.as_ref().unwrap().inside;

            assert_eq!(expected, actual);
        };

        then "comps.over_point.z < -EPSILON/2" |world, _step| {
            let expected: f32 = -(EPSILON/2.0);

            let actual = world.comps.as_ref().unwrap().over_point.z();

            assert_eq!(round(expected), round(actual));
        };

        then "comps.point.z > comps.over_point.z" |world, _step| {
            let comps = world.comps.as_ref().unwrap();

            assert!(comps.point.z() > comps.over_point.z(), "Expected comps.point.z ({}) to be greater than comps._over_point.z ({})", comps.point.z(), comps.over_point.z());
        };

        then regex r"^c = color\((.*), (.*), (.*)\)$" |world, matches, _step| {
            let r: f32 = matches[1].parse().unwrap();
            let g: f32 = matches[2].parse().unwrap();
            let b: f32 = matches[3].parse().unwrap();

            let expected = Color::new(r, g, b);

            let actual = world.c;

            assert_eq!(expected.rounded(), actual.rounded());
        };

        then "c = inner.material.color" |world, _step| {
            let expected = world.rw.objects().get(1).unwrap().material.color;
            let actual = world.c;

            assert_eq!(expected, actual);
        };

        then "t = identity_matrix" |world, _step| {
            let expected = TransformationMatrix::identity();

            let actual = *world.t;

            assert_eq!(expected, actual);
        };

        then "t = scaling(-1, 1, -1)" |world, _step| {
            let expected = scaling(-1.0, 1.0, -1.0);

            let actual: TransformationMatrix = *world.t;

            assert_eq!(expected, actual);
        };

        then "t = translation(0, 0, -8)" |world, _step| {
            let expected = translation(0.0, 0.0, -8.0);

            let actual: TransformationMatrix = *world.t;

            assert_eq!(expected, actual);
        };

        then "t is the following 4x4 matrix:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut expected = Matrix4x4::default();

            for (c, value) in table.header.iter().enumerate() {
                expected[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    expected[[r + 1,c]] = value.parse().unwrap();
                }
            }

            let actual = *world.t;

            assert_eq!(expected.rounded(), actual.rounded());
        };

        then "c.hsize = 160" |world, _step| {
            assert_eq!(160.0, world.camera.hsize);
        };

        then "c.vsize = 120" |world, _step| {
            assert_eq!(120.0, world.camera.vsize);
        };

        then "c.field_of_view = π/2" |world, _step| {
            assert_eq!(PI / 2.0, world.camera.field_of_view);
        };

        then "c.transform = identity_matrix" |world, _step| {
            let expected = &TransformationMatrix::identity();

            let actual = &world.camera.transform;

            assert_eq!(expected, actual);
        };

        then "c.pixel_size = 0.01" |world, _step| {
            assert_eq!(0.01_f32, world.camera.pixel_size);
        };

        then "pixel_at(image, 5, 5) = color(0.38066, 0.47583, 0.2855)" |world, _step| {
            let expected = Color::new(0.38066, 0.47583, 0.2855);

            let actual = world.image.pixel_at(5, 5);

            assert_eq!(expected.rounded(), actual.rounded());
        };

        then "is_shadowed(w, p) is true" |world, _step| {
            assert!(world.rw.is_shadowed(world.p));
        };

        then "is_shadowed(w, p) is false" |world, _step| {
            assert!(!world.rw.is_shadowed(world.p));
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
