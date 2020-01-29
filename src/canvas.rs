use super::color::{Color, BLACK};

const PPM_VERSION: &str = "P3";
const MAX_COLOR_VALUE: f32 = 1.0;
const MIN_COLOR_VALUE: f32 = 0.0;
const PPM_MAX_COLOR_VALUE: u32 = 255;

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
        self.verify_valid_coords(x, y);

        let i = x + (y * self.width);
        self.pixels[i as usize] = color;
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        self.verify_valid_coords(x, y);

        let i = x + (y * self.width);
        self.pixels[i as usize]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        let headers = format!("{}\n{} {}\n{}\n", PPM_VERSION, self.width, self.height, PPM_MAX_COLOR_VALUE);
        ppm.push_str(&headers);

        for (i, pixel) in self.pixels.iter().enumerate() {
            let red = within_bounds(pixel.red);
            let green = within_bounds(pixel.green);
            let blue = within_bounds(pixel.blue);

            let pixel_str = format!("{} {} {} ", red, green, blue);
            ppm.push_str(&pixel_str);

            if i > 0 && ((i + 1) % (self.width as usize) == 0) {
                ppm = ppm.trim().to_string();
                ppm.push_str(&"\n");
            }
        }

        ppm
    }

    fn verify_valid_coords(&self, x: u32, y: u32) {
        assert!(x < self.width, "x ({}) must be less than canvas width ({}).", x, self.width);
        assert!(y < self.height, "y ({}) must be less than canvas height ({}).", y, self.height);
    }
}

fn within_bounds(color_value: f32) -> u32 {
    let bounded = color_value.max(MIN_COLOR_VALUE).min(MAX_COLOR_VALUE) * (PPM_MAX_COLOR_VALUE as f32);
    let bounded = bounded.round();

    bounded as u32
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
        assert!(canvas.pixels[32].equalish_to(&red), "Actual: {:?}", canvas.pixels[32]);
    }

    #[test]
    fn ppm_headers() {
        let canvas = Canvas::new(5, 3);

        let ppm = canvas.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();

        assert!(lines[0].trim() == "P3", "Actual: {}", lines[0].trim());
        assert!(lines[1].trim() == "5 3", "Actual: {}", lines[1].trim());
        assert!(lines[2].trim() == "255", "Actual: {}", lines[2].trim());
    }

    #[test]
    fn ppm_content() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);

        let ppm = canvas.to_ppm();
        let lines: Vec<&str> = ppm.split("\n").collect();

        assert!(lines[3].trim() == "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", "Actual: {}", lines[3].trim());
        assert!(lines[4].trim() == "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", "Actual: {}", lines[4].trim());
        assert!(lines[5].trim() == "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", "Actual: {}", lines[5].trim());
    }
}
