// Derived from example at https://github.com/bbqsrc/cucumber-rust/blob/master/README.md

use ndarray::*;

use cucumber::{cucumber, before, after};

use ray_tracer_challenge::*;
use ray_tracer_challenge::color::*;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    colors: Vec<Color>,
    matrix: Array<f32, Ix2>,
    matrix_a: Array<f32, Ix2>,
    matrix_b: Array<f32, Ix2>,
    matrix_c: Array<f32, Ix2>,
    transform: Array<f32, Ix2>,
    p: Point,
    v: Vector,
    inv: Array<f32, Ix2>,
    tuple: (f32, f32, f32, f32),
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld {
            colors: vec![BLACK; 3],
            matrix: Array::from_elem((4, 4), 0.0),
            matrix_a: Array::from_elem((4, 4), 0.0),
            matrix_b: Array::from_elem((4, 4), 0.0),
            matrix_c: Array::from_elem((4, 4), 0.0),
            transform: Array::from_elem((4, 4), 0.0),
            p: Point::new(0.0, 0.0, 0.0),
            v: Vector::new(0.0, 0.0, 0.0),
            inv: Array::from_elem((4, 4), 0.0),
            tuple: (0.0, 0.0, 0.0, 0.0),
        }
    }
}

mod example_steps {
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

            let actual = world.transform.clone() * world.p;

            assert_eq!(expected, actual);
        };

        then regex r"inv \* p = point\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Point::new(x, y, z);

            let actual = world.inv.clone() * world.p;

            assert_eq!(expected, actual);
        };

        then "transform * v = v" |world, _step| {
            let expected = world.v;

            let actual = world.transform.clone() * world.v;

            assert_eq!(expected, actual);
        };

        then regex r"transform \* v = vector\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Vector::new(x, y, z);

            let actual = world.transform.clone() * world.v;

            assert_eq!(expected, actual);
        };

        then regex r"inv \* v = vector\((.*), (.*), (.*)\)" |world, matches, _step| {
            let x: f32 = matches[1].parse().unwrap();
            let y: f32 = matches[2].parse().unwrap();
            let z: f32 = matches[3].parse().unwrap();

            let expected = Vector::new(x, y, z);

            let actual = world.inv.clone() * world.v;

            assert_eq!(expected, actual);
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
