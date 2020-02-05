use std::fs::File;
use std::io::prelude::*;

use ray_tracer_challenge::*;
use ray_tracer_challenge::physics::*;
use ray_tracer_challenge::color::*;
use ray_tracer_challenge::canvas::*;

const CANVAS_WIDTH: u32 = 400;
const CANVAS_HEIGHT: u32 = 200;

fn main() -> std::io::Result<()> {
    let mut projectile = Projectile {
        position: Point { x: 1.0, y: 1.0, z: 1.0 },
        velocity: Vector { x: 2.0, y: 5.0, z: 1.0 },
    };
    let environment = Environment {
        gravity: Vector { x: 0.0, y: -0.1, z: -0.1 },
        wind: Vector { x: 0.02, y: 0.0, z: 0.0 },
    };

    let mut canvas = Canvas::of_color(CANVAS_WIDTH, CANVAS_HEIGHT, BLACK);

    println!("It's a projectile! {:?}", projectile);
    println!("It's an environment! {:?}", environment);

    while projectile.position.y > 0.0 {
        projectile = projectile.tick(&environment);

        println!("Updated projectile! {:?}", projectile);

        let x = projectile.position.x.round() as i32;
        let y = CANVAS_HEIGHT as i32 - projectile.position.y.round() as i32;
        let z = 100.0 / projectile.position.z.abs();
        let color = WHITE * z;

        println!("{} {:?}", z, color);

        if x < 0 && y < 0 {
            continue;
        }

        let x = x as u32;
        let y = y as u32;

        if x < CANVAS_WIDTH && y < CANVAS_HEIGHT {
            canvas.write_pixel(x, y, color);
        }
    }

    println!("And we've landed!");

    let ppm = canvas.to_ppm();

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&ppm.as_bytes())?;

    Ok(())
}