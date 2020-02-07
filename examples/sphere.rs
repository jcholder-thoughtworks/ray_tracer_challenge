use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;

use ray_tracer_challenge::*;
use ray_tracer_challenge::{Interceptable};
use ray_tracer_challenge::math::transforms::*;
use ray_tracer_challenge::color::*;
use ray_tracer_challenge::canvas::*;

const CANVAS_WIDTH: u32 = 100;
const CANVAS_HEIGHT: u32 = 100;
const ZOOM: f32 = 0.01;

fn main() -> std::io::Result<()> {
    let red: Color = Color::new(1.0, 0.0, 0.0);

    let mut canvas = Canvas::of_color(CANVAS_WIDTH, CANVAS_HEIGHT, BLACK);

    println!("Drawing the sphere ...");

    let mut world = RaytracerWorld::new();

    let mut sphere = world.new_sphere(CENTER_ORIGIN);
    //sphere.transform = scaling(12.0, 15.0, 5.0).dot(&translation(3.0, 3.0, 0.0));

    let sphere_rc: Rc<dyn Interceptable> = Rc::new(sphere.clone());

    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 0.1);

    let ray = Ray::new(origin, direction);

    let w = CANVAS_WIDTH as i32;
    let h = CANVAS_HEIGHT as i32;

    for x in 0..CANVAS_WIDTH {
        for y in 0..CANVAS_HEIGHT {
            let rot_x = (x as i32 - (w / 2)) as f32 * ZOOM;
            let rot_y = (y as i32 - (h / 2)) as f32 * ZOOM;

            let rx = rotation_x(rot_x);
            let ry = rotation_y(rot_y);

            let new_direction = ray.direction * rx.dot(&ry);

            let pointed_ray = Ray::new(ray.origin, new_direction);

            let intersections = intersect(&sphere_rc, &pointed_ray);

            let hit = intersections.hit();

            if let Some(_) = hit {
                canvas.write_pixel(x, y, red);
            }
        }
    }

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
