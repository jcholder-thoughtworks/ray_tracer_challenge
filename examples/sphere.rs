use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;

use ray_tracer_challenge::*;
use ray_tracer_challenge::{Interceptable};
use ray_tracer_challenge::math::transforms::*;
use ray_tracer_challenge::color::*;
use ray_tracer_challenge::canvas::*;

const CANVAS_WIDTH: u32 = 50;
const CANVAS_HEIGHT: u32 = 50;

fn main() -> std::io::Result<()> {
    let red: Color = Color::new(1.0, 0.0, 0.0);

    let mut canvas = Canvas::of_color(CANVAS_WIDTH, CANVAS_HEIGHT, BLACK);

    println!("Drawing the sphere ...");

    let mut world = RaytracerWorld::new();

    let mut sphere = world.new_sphere(CENTER_ORIGIN);
    sphere.transform = scaling(9.0, 10.0, 5.0).dot(&translation(3.0, 3.0, 0.0));

    let sphere_rc: Rc<dyn Interceptable> = Rc::new(sphere.clone());

    for x in 0..CANVAS_WIDTH {
        for y in 0..CANVAS_HEIGHT {
            let origin = Point::new(x as f32, y as f32, -10.0);
            let direction = Vector::new(0.0, 0.0, 1.0);

            let ray = Ray::new(origin, direction);

            let intersections = intersect(&sphere_rc, &ray);

            let hit = intersections.hit();

            if let Some(_) = hit {
                println!("Hit on {}, {}!", x, y);
                canvas.write_pixel(x, y, red);
            } else {
                println!("Miss on {}, {}", x, y);
            }
        }
    }

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
