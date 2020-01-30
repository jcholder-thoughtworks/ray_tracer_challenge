// Derived from example at https://github.com/bbqsrc/cucumber-rust/blob/master/README.md

use ndarray::*;

use cucumber::{cucumber, before, after};

use ray_tracer_challenge::color::*;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    colors: Vec<Color>,
    matrix: Array<f32, Ix2>,
    matrix_a: Array<i32, Ix2>,
    matrix_b: Array<i32, Ix2>,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld { 
            colors: vec![BLACK; 3],
            matrix: Array::from_elem((4, 4), 0.0),
            matrix_a: Array::from_elem((4, 4), 0),
            matrix_b: Array::from_elem((4, 4), 0),
        }
    }
}

mod example_steps {
    use cucumber::steps;

    use ndarray::*;

    use ray_tracer_challenge::color::*;
    
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

        given "the following matrix B:" |world, step| {
            let table = step.table().unwrap().clone();

            world.matrix_b = Array::from_elem((4, 4), 0);

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

            world.matrix_a = Array::from_elem((4, 4), 0);

            for (c, value) in table.header.iter().enumerate() {
                world.matrix_a[[0,c]] = value.parse().unwrap();
            }

            for (r, row) in table.rows.iter().enumerate() {
                for (c, value) in row.iter().enumerate() {
                    world.matrix_a[[r + 1,c]] = value.parse().unwrap();
                }
            }
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

        then "A * B is the following 4x4 matrix:" |world, step| {
            let table = step.table().unwrap().clone();

            let mut expected_matrix: Array<i32, Ix2> = Array::from_elem((4, 4), 0);

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
            let identity_matrix: Array<i32, Ix2> = Array::eye(4);

            assert_eq!(world.matrix_a.dot(&identity_matrix), world.matrix_a)
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
