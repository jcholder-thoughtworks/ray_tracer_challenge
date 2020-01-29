use std::fs::File;
use std::io::prelude::*;

use ray_tracer_challenge::*;
use ray_tracer_challenge::physics::*;
use ray_tracer_challenge::color::*;
use ray_tracer_challenge::canvas::*;

fn main() -> std::io::Result<()> {
    let mut projectile = Projectile {
        position: Point { x: 1.0, y: 1.0, z: 1.0 },
        velocity: Vector { x: 1.0, y: 1.0, z: 1.0 },
    };
    let environment = Environment {
        gravity: Vector { x: 0.0, y: -0.1, z: 0.0 },
        wind: Vector { x: 0.1, y: 0.0, z: 0.0 },
    };

    println!("It's a projectile! {:?}", projectile);
    println!("It's an environment! {:?}", environment);

    while projectile.position.y > 0.0 {
        projectile = projectile.tick(&environment);
        println!("Updated projectile! {:?}", projectile);
    }

    println!("And we've landed!");

    println!("Here's a color: {:?}.", Color { red: 0.1, green: 0.2, blue: 0.3 });

    let mut canvas = Canvas::new(5, 3);
    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);

    canvas.write_pixel(0, 0, c1);
    canvas.write_pixel(2, 1, c2);
    canvas.write_pixel(4, 2, c3);

    println!("Here's a canvas: {:?}.", canvas);

    let ppm = canvas.to_ppm();

    println!("Here's the canvas as a PPM file: {}", ppm);

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&ppm.as_bytes())?;

    Ok(())
}
