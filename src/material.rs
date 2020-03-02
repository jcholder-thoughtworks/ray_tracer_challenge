use super::color::{Color, BLACK};
use super::light::Light;
use super::{Point, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        &self,
        light: Light,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        let material = self;

        let effective_color: Color = material.color * light.intensity;

        let lightv: Vector = (light.position - point).norm();

        let ambient: Color = effective_color * material.ambient;

        let light_dot_normal: f32 = lightv.dot(normalv);

        let mut diffuse: Color = BLACK;
        let mut specular: Color = BLACK;

        if light_dot_normal >= 0.0 {
            diffuse = effective_color * material.diffuse * light_dot_normal;
            let reflectv: Vector = (lightv * -1.0).reflect(&normalv);
            let reflect_dot_eye: f32 = reflectv.dot(eyev);

            if reflect_dot_eye > 0.0 {
                let factor: f32 = reflect_dot_eye.powf(material.shininess);
                specular = light.intensity * material.specular * factor;
            }
        }

        if in_shadow {
            ambient
        } else {
            ambient + diffuse + specular
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}
