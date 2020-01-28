use super::*;

#[derive(Clone,Debug)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

impl Projectile {
    pub fn tick(self, environment: &Environment) -> Self {
        let position = self.position + self.velocity.clone();
        let velocity = self.velocity + environment.gravity + environment.wind;

        Self { position, velocity }
    }
}

#[derive(Clone,Debug)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}
