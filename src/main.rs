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
}
