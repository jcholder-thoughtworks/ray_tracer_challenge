use ray_tracer_challenge::*;

fn main() {
    let point = Point { x: 1.0, y: 1.0, z: 1.0 };
    let vector = Vector { x: 1.0, y: 1.0, z: 1.0 };

    println!("It's a point! {:?}", point);
    println!("It's a vector! {:?}", vector);
    println!("Let's add them together! {:?}", point + vector);
}
