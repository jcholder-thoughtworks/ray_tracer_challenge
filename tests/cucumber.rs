// Derived from example at https://github.com/bbqsrc/cucumber-rust/blob/master/README.md

use cucumber::{cucumber, before, after};

use ray_tracer_challenge::color::*;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    colors: Vec<Color>,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld { 
            colors: vec![BLACK; 3],
        }
    }
}

mod example_steps {
    use cucumber::steps;

    use ray_tracer_challenge::color::*;
    
    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
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