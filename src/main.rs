use ray_tracer_challenge::*;
use ray_tracer_challenge::physics::*;
use ray_tracer_challenge::color::*;

fn main() {
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
}
