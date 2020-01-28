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

impl Vector {
    pub fn equalish_to(&self, other: &Self) -> bool {
        equalish(self.x, other.x) &&
            equalish(self.y, other.y) &&
            equalish(self.z, other.z)
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
    }
}
