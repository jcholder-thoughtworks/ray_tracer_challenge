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
}
