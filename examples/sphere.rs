use core::f32::consts::PI;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use ray_tracer_challenge::color::*;
use ray_tracer_challenge::light::*;
use ray_tracer_challenge::material::*;
use ray_tracer_challenge::math::transforms::*;
use ray_tracer_challenge::*;

fn parse_arg(arg: &str) -> f32 {
    match arg.parse() {
        Ok(t) => t,
        Err(_e) => panic!("Invalid argument: {}. Expected a float value (e.g. 100.0)", arg),
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some(a) = args.get(1) {
        if a == "--help" {
            let cmd_name = args.get(0).unwrap();
            println!("Usage: {} [width:float] [height:float] [field-of-view:float]", cmd_name);
            println!("e.g. {} 200.0, 100.0, 1.047", cmd_name);
            return Ok(());
        }
    }

    let canvas_width: f32 = match args.get(1) {
        Some(a) => parse_arg(a),
        None => 200.0,
    };

    let canvas_height: f32 = match args.get(2) {
        Some(a) => parse_arg(a),
        None => 100.0,
    };

    let field_of_view: f32 = match args.get(3) {
        Some(a) => parse_arg(a),
        None => PI / 3.0,
    };

    println!("Settings: width({}), height({}), field of view({})", canvas_width, canvas_height, field_of_view);
    println!("Rendering the scene ...");

    let mut world = RaytracerWorld::new();

    let light = Light::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    world.light = Some(light);

    let mut floor = world.new_sphere(CENTER_ORIGIN);
    floor.transform = scaling(10.0, 0.01, 10.0);
    floor.material = Material::default();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let floor_material = floor.material;

    world.add_object(floor);

    let mut left_wall = world.new_sphere(CENTER_ORIGIN);
    // TODO: Operation order might be reversed
    left_wall.transform = translation(0.0, 0.0, 5.0) * rotation_y(-PI/4.0) * rotation_x(PI/2.0) * scaling(10.0, 0.01, 10.0);
    left_wall.material = floor_material;
    world.add_object(left_wall);

    let mut right_wall = world.new_sphere(CENTER_ORIGIN);
    // TODO: Operation order might be reversed
    right_wall.transform = translation(0.0, 0.0, 5.0) * rotation_y(PI/4.0) * rotation_x(PI/2.0) * scaling(10.0, 0.01, 10.0);
    right_wall.material = floor_material;
    world.add_object(right_wall);

    let mut middle = world.new_sphere(CENTER_ORIGIN);
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.add_object(middle);

    let mut right = world.new_sphere(CENTER_ORIGIN);
    // TODO: Operation order might be reversed
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.add_object(right);

    let mut left = world.new_sphere(CENTER_ORIGIN);
    // TODO: Operation order might be reversed
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.add_object(left);

    let mut camera = Camera::new(canvas_width, canvas_height, field_of_view);
    camera.transform = view_transform(&Point::new(0.0, 1.5, -5.0), &Point::new(0.0, 1.0, 0.0), &Vector::new(0.0, 1.0, 0.0));

    let canvas = camera.render(&world);

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
