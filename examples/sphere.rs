use std::fs::File;
use std::io::prelude::*;

use ray_tracer_challenge::canvas::*;
use ray_tracer_challenge::color::*;
use ray_tracer_challenge::material::*;
use ray_tracer_challenge::math::transforms::*;
use ray_tracer_challenge::*;

const CANVAS_WIDTH: u32 = 150;
const CANVAS_HEIGHT: u32 = 150;
const ZOOM: f32 = 0.01;

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::of_color(CANVAS_WIDTH, CANVAS_HEIGHT, BLACK);

    println!("Drawing the sphere ...");

    let mut world = RaytracerWorld::default();

    let mut sphere = world.new_sphere(CENTER_ORIGIN);
    sphere.material = Material::default();
    sphere.material.color = Color::new(1.0, 0.2, 1.0);
    sphere.transform = scaling(3.0, 3.0, 3.0);

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = Light::new(light_position, light_color);

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

            let new_direction = (ray.direction * rx.dot(&ry)).norm();

            let pointed_ray = Ray::new(ray.origin, new_direction);

            let hit = sphere.hit_on_intersect(&pointed_ray);

            if let Some(h) = hit {
                let point = pointed_ray.position(h.time);
                let normal = h.object.normal_at(point);
                let eye = pointed_ray.direction;
                let color = h.object.material().lighting(light, point, eye, normal);

                canvas.write_pixel(x, y, color);
            }
        }
    }

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&canvas.to_ppm().as_bytes())?;

    Ok(())
}
