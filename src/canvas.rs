use super::color::{Color, BLACK};

#[derive(Clone,Debug)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![BLACK; (width * height) as usize],
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        let i = x + (y * self.height);
        self.pixels[i as usize] = color;
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        let i = x + (y * self.height);
        self.pixels[i as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        let canvas = Canvas::new(10, 20);

        assert!(canvas.width == 10);
        assert!(canvas.height == 20);

        for i in 0..(10 * 20) {
            let pixel = &canvas.pixels[i];

            assert!(pixel.red == BLACK.red);
            assert!(pixel.green == BLACK.green);
            assert!(pixel.blue == BLACK.blue);
        }
    }

    #[test]
    fn writing_and_reading_pixels() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        canvas.write_pixel(2, 3, red);

        let pixel = canvas.pixel_at(2, 3);

        assert!(pixel.equalish_to(&red));
    }
}
