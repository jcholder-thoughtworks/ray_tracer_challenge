extern crate clap;
use clap::{Arg, App};

use core::f32::consts::PI;

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
    let matches = App::new("Example: Sphere")
        .version("0.0.1")
        .author("JC Holder")
        .arg(Arg::with_name("width")
             .help("Width of the image to render in pixels. Default: 200")
             .required(true)
             .index(1))
        .arg(Arg::with_name("height")
             .help("Height of the image to render in pixels: Default: 100")
             .required(true)
             .index(2))
        .arg(Arg::with_name("field-of-view")
             .help("Field of view for the camera. Default: PI / 3")
             .required(false)
             .index(3))
        .arg(Arg::with_name("threaded")
             .help("Splits the rendering work across multiple threads. (Default: single threaded.)")
             .short("t")
             .long("threaded"))
        .get_matches();

    let threaded = matches.is_present("threaded");
    let canvas_width = parse_arg(matches.value_of("width").unwrap_or("200.0"));
    let canvas_height = parse_arg(matches.value_of("height").unwrap_or("200.0"));
    let field_of_view = parse_arg(matches.value_of("field-of-view").unwrap_or(&(PI / 3.0).to_string()));

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

    let canvas = if threaded {
        render_threaded(world, camera)
    } else {
        camera.render(&world)
    };

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
