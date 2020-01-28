const EPSILON: f32 = 0.00001;

struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn equalish_to(&self, other: &Point) -> bool {
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
    }
}
