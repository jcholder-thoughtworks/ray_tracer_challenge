use core::f32::consts::PI;

use std::fs::File;
use std::io::prelude::*;

use ray_tracer_challenge::color::*;
use ray_tracer_challenge::light::*;
use ray_tracer_challenge::material::*;
use ray_tracer_challenge::math::transforms::*;
use ray_tracer_challenge::*;

const CANVAS_WIDTH: f32 = 200.0;
const CANVAS_HEIGHT: f32 = 100.0;
const FIELD_OF_VIEW: f32 = PI / 3.0;

fn main() -> std::io::Result<()> {
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
    left_wall.transform = translation(0.0, 0.0, 5.0).dot(&rotation_y(-PI/4.0).dot(&rotation_x(PI/2.0).dot(&scaling(10.0, 0.01, 10.0))));
    left_wall.material = floor_material;
    world.add_object(left_wall);

    let mut right_wall = world.new_sphere(CENTER_ORIGIN);
    right_wall.transform = translation(0.0, 0.0, 5.0).dot(&rotation_y(PI/4.0).dot(&rotation_x(PI/2.0).dot(&scaling(10.0, 0.01, 10.0))));
    right_wall.material = floor_material;
    world.add_object(right_wall);

    let mut middle = world.new_sphere(CENTER_ORIGIN);
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.add_object(middle);

    let mut right = world.new_sphere(CENTER_ORIGIN);
    right.transform = translation(1.5, 0.5, -0.5).dot(&scaling(0.5, 0.5, 0.5));
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.add_object(right);

    let mut left = world.new_sphere(CENTER_ORIGIN);
    left.transform = translation(-1.5, 0.33, -0.75).dot(&scaling(0.33, 0.33, 0.33));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.add_object(left);

    let mut camera = Camera::new(CANVAS_WIDTH, CANVAS_HEIGHT, FIELD_OF_VIEW);
    camera.transform = view_transform(&Point::new(0.0, 1.5, -5.0), &Point::new(0.0, 1.0, 0.0), &Vector::new(0.0, 1.0, 0.0));

    let canvas = camera.render(&world);

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
