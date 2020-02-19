use core::f32::consts::PI;

use std::fs::File;
use std::io::prelude::*;

use ray_tracer_challenge::color::*;
use ray_tracer_challenge::material::*;
use ray_tracer_challenge::math::transforms::*;
use ray_tracer_challenge::*;

const CANVAS_WIDTH: f32 = 350.0;
const CANVAS_HEIGHT: f32 = 350.0;
const FIELD_OF_VIEW: f32 = PI / 2.0;

fn main() -> std::io::Result<()> {
    println!("Drawing the sphere ...");

    let mut world = RaytracerWorld::default();

    let mut light = world.light.unwrap();
    light.position.z = 2.0;
    world.light = Some(light);

    let mut sphere = world.new_sphere(CENTER_ORIGIN);
    sphere.material = Material::default();
    sphere.material.color = Color::new(1.0, 0.0, 0.8);
    sphere.transform = scaling(4.0, 4.0, 4.0);

    world.add_object(sphere);

    let mut camera = Camera::new(CANVAS_WIDTH, CANVAS_HEIGHT, FIELD_OF_VIEW);
    camera.transform = translation(0.0, 0.0, -3.0);

    let canvas = camera.render(&world);

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
