// Derived from example at https://github.com/bbqsrc/cucumber-rust/blob/master/README.md

#[macro_use]
extern crate ndarray;

use ndarray::*;
use ndarray::prelude::*;

use cucumber::{cucumber, before, after};

use ray_tracer_challenge::color::*;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    colors: Vec<Color>,
    matrix: Array<f32, Ix2>,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld { 
            colors: vec![BLACK; 3],
            matrix: Array::from_elem((4, 4), 0.0),
        }
    }
}

mod example_steps {
    use cucumber::steps;

    use ndarray::*;

    use ray_tracer_challenge::color::*;
    
    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
        given "the following 4x4 matrix M:" |world, step| {
            let table = step.table().unwrap().clone();

            world.matrix = Array::from_elem((4, 4), 0.0);

            for (x, value) in table.header.iter().enumerate() {
                world.matrix[[0,x]] = value.parse().unwrap();
            }

            for (y, row) in table.rows.iter().enumerate() {
                for (x, value) in row.iter().enumerate() {
                    world.matrix[[y + 1,x]] = value.parse().unwrap();
                }
            }
        };

        then regex r"M\[(.*),(.*)\] = (.*)" |world, matches, step| {
            let x: usize = matches[1].parse().unwrap();
            let y: usize = matches[2].parse().unwrap();
            let expected: f32 = matches[3].parse().unwrap();

            assert_eq!(expected, world.matrix[[x, y]]);
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
