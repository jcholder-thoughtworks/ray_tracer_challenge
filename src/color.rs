use std::ops;

use super::equalish;

#[derive(Clone,Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Color { red, green, blue }
    }

    pub fn equalish_to(&self, other: &Self) -> bool {
        equalish(self.red, other.red) &&
            equalish(self.green, other.green) &&
            equalish(self.blue, other.blue)
    }

    fn hadamard_schur_product(&self, other: &Self) -> Self {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.hadamard_schur_product(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let expected = Color::new(1.6, 0.7, 1.0);
        let actual = c1 + c2;

        assert!(actual.equalish_to(&expected), "Expected {:?} but got {:?}", expected, actual);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let expected = Color::new(0.2, 0.5, 0.5);
        let actual = c1 - c2;

        assert!(actual.equalish_to(&expected), "Expected {:?} but got {:?}", expected, actual);
    }

    #[test]
    fn multiplying_colors_by_scalars() {
        let c = Color::new(0.2, 0.3, 0.4);

        let expected = Color::new(0.4, 0.6, 0.8);
        let actual = c * 2.0;

        assert!(actual.equalish_to(&expected), "Expected {:?} but got {:?}", expected, actual);
    }

    #[test]
    fn multiplying_colors_by_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let expected = Color::new(0.9, 0.2, 0.04);
        let actual = c1 * c2;

        assert!(actual.equalish_to(&expected), "Expected {:?} but got {:?}", expected, actual);
    }
}
