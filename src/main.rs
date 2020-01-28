const EPSILON: f32 = 0.00001;

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

    pub fn add_vector(&self, other: &Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract_point(&self, other: &Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn subtract_vector(&self, other: &Vector) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn equalish_to(&self, other: &Self) -> bool {
        equalish(self.x, other.x) &&
            equalish(self.y, other.y) &&
            equalish(self.z, other.z)
    }

    pub fn add_vector(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn add_point(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract_vector(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
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

            assert!(a.add_vector(&b).equalish_to(&expected));
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

            assert!(a.subtract_vector(&b).equalish_to(&expected));
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

            assert!(a.subtract_point(&b).equalish_to(&expected));
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

            assert!(a.add_vector(&b).equalish_to(&expected));
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

            assert!(a.add_point(&b).equalish_to(&expected));
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

            assert!(a.subtract_vector(&b).equalish_to(&expected));
        }
    }
}
