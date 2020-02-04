use std::fs::File;
use std::io::prelude::*;
use std::f32::consts::PI;

use ray_tracer_challenge::*;
use ray_tracer_challenge::math::transforms::*;
use ray_tracer_challenge::color::*;
use ray_tracer_challenge::canvas::*;

const CANVAS_WIDTH: u32 = 400;
const CANVAS_HEIGHT: u32 = 400;

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::of_color(CANVAS_WIDTH, CANVAS_HEIGHT, BLACK);

    println!("Drawing the clock ...");

    let mut hand = Point::new(0.0, 1.0, 0.0);

    let hour_turn = rotation_z(-(2.0 * PI) / 12.0);

    for hour in 0..12 {
        println!("{:?}", hand);

        let x: i32 = ((CANVAS_WIDTH as f32 * 0.4) * hand.x).round() as i32;
        let y: i32 = ((CANVAS_HEIGHT as f32 * 0.4) * hand.y).round() as i32;

        println!("Starting here: {}, {}", x, y);

        let x = (x + CANVAS_WIDTH as i32 / 2) as u32;
        let y = (y + CANVAS_HEIGHT as i32 / 2) as u32;

        println!("Drawing dot at {}, {}", x, y);

        canvas.write_pixel(x, y, WHITE);

        hand = &hour_turn * hand;
    }

    let ppm = canvas.to_ppm();

    println!("Saving the PPM to a file ...");

    let mut file = File::create("output.ppm")?;
    file.write_all(&ppm.as_bytes())?;

    Ok(())
}
