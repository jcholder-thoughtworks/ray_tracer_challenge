use std::ops;

const EPSILON: f32 = 0.00001;

#[derive(Clone,Debug)]
struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn equalish_to(&self, other: &Self) -> bool {
        equalish(self.x, other.x) &&
            equalish(self.y, other.y) &&
            equalish(self.z, other.z)
    }
}

impl ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Clone,Debug)]
struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// TODO: Template for float type
impl ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// TODO: Template for float type
impl ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vector {
    pub fn equalish_to(&self, other: &Self) -> bool {
        equalish(self.x, other.x) &&
            equalish(self.y, other.y) &&
            equalish(self.z, other.z)
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn norm(self) -> Self {
        let mag = self.mag();

        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) +
            (self.y * rhs.y) +
            (self.z * rhs.z)
    }
}

fn equalish(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equalish_is_true_for_diff_below_epsilon() {
        assert!(equalish(1.0, 1.000001));
    }

    #[test]
    fn equalish_is_false_for_diff_above_epsilon() {
        assert!(! equalish(1.0, 1.00002));
    }

    mod point_tests {
        use super::*;

        #[test]
        fn equalish_is_true_for_diff_below_epsilon() {
            let point_a = Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };

            let point_b = Point {
                x: 1.000001,
                y: 1.000001,
                z: 1.000001,
            };

            assert!(point_a.equalish_to(&point_b));
        }

        #[test]
        fn equalish_is_false_for_diff_above_epsilon() {
            let point_a = Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };

            let point_b = Point {
                x: 1.00002,
                y: 1.00002,
                z: 1.00002,
            };

            assert!(! point_a.equalish_to(&point_b));
        }

        #[test]
        fn add_vector_sums_each_pair_of_values() {
            let a = Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let b = Vector {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            };

            let expected = Point {
                x: 3.0,
                y: 5.0,
                z: 7.0,
            };

            assert!((a + b).equalish_to(&expected));
        }

        #[test]
        fn subtract_vector_subtracts_latter_from_former_for_each_pair_of_values() {
            let a = Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let b = Vector {
                x: 1.0,
                y: 1.0,
                z: 4.0,
            };

            let expected = Point {
                x: 0.0,
                y: 1.0,
                z: -1.0,
            };

            assert!((a - b).equalish_to(&expected));
        }

        #[test]
        fn subtract_point_subtracts_latter_from_former_for_each_pair_of_values() {
            let a = Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let b = Point {
                x: 1.0,
                y: 1.0,
                z: 4.0,
            };

            let expected = Vector {
                x: 0.0,
                y: 1.0,
                z: -1.0,
            };

            assert!((a - b).equalish_to(&expected));
        }
    }

    mod vector_tests {
        use super::*;

        #[test]
        fn equalish_is_true_for_diff_below_epsilon() {
            let vector_a = Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };

            let vector_b = Vector {
                x: 1.000001,
                y: 1.000001,
                z: 1.000001,
            };

            assert!(vector_a.equalish_to(&vector_b));
        }

        #[test]
        fn equalish_is_false_for_diff_above_epsilon() {
            let vector_a = Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };

            let vector_b = Vector {
                x: 1.00002,
                y: 1.00002,
                z: 1.00002,
            };

            assert!(! vector_a.equalish_to(&vector_b));
        }

        #[test]
        fn add_vector_sums_each_pair_of_values() {
            let a = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let b = Vector {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            };

            let expected = Vector {
                x: 3.0,
                y: 5.0,
                z: 7.0,
            };

            assert!((a + b).equalish_to(&expected));
        }

        #[test]
        fn add_point_sums_each_pair_of_values() {
            let a = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let b = Point {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            };

            let expected = Point {
                x: 3.0,
                y: 5.0,
                z: 7.0,
            };

            assert!((a + b).equalish_to(&expected));
        }

        #[test]
        fn subtract_vector_subtracts_second_value_from_for_for_each_pair() {
            let a = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let b = Vector {
                x: 1.0,
                y: 3.0,
                z: 2.0,
            };

            let expected = Vector {
                x: 0.0,
                y: -1.0,
                z: 1.0,
            };

            assert!((a - b).equalish_to(&expected));
        }

        #[test]
        fn negating_vector_inverts_each_value() {
            let v = Vector {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            };

            let expected = Vector {
                x: -1.0,
                y: 1.0,
                z: 0.0,
            };

            assert!((-v).equalish_to(&expected));
        }

        #[test]
        fn multiply_vector_multiplies_each_value() {
            let a = Vector {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            };

            let b = 5.0;

            let expected = Vector {
                x: 5.0,
                y: -5.0,
                z: 0.0,
            };

            assert!((a * b).equalish_to(&expected));
        }

        #[test]
        fn divide_vector_divides_each_value() {
            let a = Vector {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            };

            let b = 2.0;

            let expected = Vector {
                x: 0.5,
                y: -0.5,
                z: 0.0,
            };

            let result = a / b;

            assert!(result.equalish_to(&expected), "Expected {:?} but got {:?}", expected, result);
        }

        #[test]
        fn magnitudes_of_vectors() {
            let v = Vector { x: 1.0, y: 0.0, z: 0.0 };
            let expected = 1.0;
            let result = v.mag();
            assert!(equalish(expected, result), "Expected {} but got {}", expected, result);

            let v = Vector { x: 0.0, y: 1.0, z: 0.0 };
            let expected = 1.0;
            let result = v.mag();
            assert!(equalish(expected, result), "Expected {} but got {}", expected, result);

            let v = Vector { x: 0.0, y: 0.0, z: 1.0 };
            let expected = 1.0;
            let result = v.mag();
            assert!(equalish(expected, result), "Expected {} but got {}", expected, result);

            let v = Vector { x: 1.0, y: 2.0, z: 3.0 };
            let expected = (14.0 as f32).sqrt();
            let result = v.mag();
            assert!(equalish(expected, result), "Expected {} but got {}", expected, result);

            let v = Vector { x: -1.0, y: -2.0, z: -3.0 };
            let expected = (14.0 as f32).sqrt();
            let result = v.mag();
            assert!(equalish(expected, result), "Expected {} but got {}", expected, result);
        }

        #[test]
        fn normalizing_vectors() {
            let v = Vector { x: 4.0, y: 0.0, z: 0.0 };
            let expected = Vector { x: 1.0, y: 0.0, z: 0.0 };
            let result = v.norm();
            assert!(expected.equalish_to(&result), "Expected {:?} but got {:?}", expected, result);

            let v = Vector { x: 1.0, y: 2.0, z: 3.0 };
            let x = 1.0 / (14.0 as f32).sqrt();
            let y = 2.0 / (14.0 as f32).sqrt();
            let z = 3.0 / (14.0 as f32).sqrt();
            let expected = Vector { x, y, z };
            let result = v.norm();
            assert!(expected.equalish_to(&result), "Expected {:?} but got {:?}", expected, result);
        }

        #[test]
        fn dot_products_of_vectors() {
            let a = Vector { x: 1.0, y: 2.0, z: 3.0 };
            let b = Vector { x: 2.0, y: 3.0, z: 4.0 };
            let expected = 20.0;
            let result = a.dot(b);
            assert!(equalish(expected, result), "Expected {:?} but got {:?}", expected, result);
        }
    }
}
