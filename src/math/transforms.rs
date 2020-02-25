use super::{Matrix4x4, Point, Vector};

pub type TransformationMatrix = Matrix4x4;

pub fn translation(x: f32, y: f32, z: f32) -> TransformationMatrix {
    TransformationMatrix::from_2d_array([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scaling(x: f32, y: f32, z: f32) -> TransformationMatrix {
    TransformationMatrix::from_2d_array([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_x(radians: f32) -> TransformationMatrix {
    let cr = radians.cos();
    let sr = radians.sin();

    let r1: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
    let r2: [f32; 4] = [0.0, cr, -sr, 0.0];
    let r3: [f32; 4] = [0.0, sr, cr, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    TransformationMatrix::from_2d_array([r1, r2, r3, r4])
}

pub fn rotation_y(radians: f32) -> TransformationMatrix {
    let cr = radians.cos();
    let sr = radians.sin();

    let r1: [f32; 4] = [cr, 0.0, sr, 0.0];
    let r2: [f32; 4] = [0.0, 1.0, 0.0, 0.0];
    let r3: [f32; 4] = [-sr, 0.0, cr, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    TransformationMatrix::from_2d_array([r1, r2, r3, r4])
}

pub fn rotation_z(radians: f32) -> TransformationMatrix {
    let cr = radians.cos();
    let sr = radians.sin();

    let r1: [f32; 4] = [cr, -sr, 0.0, 0.0];
    let r2: [f32; 4] = [sr, cr, 0.0, 0.0];
    let r3: [f32; 4] = [0.0, 0.0, 1.0, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    TransformationMatrix::from_2d_array([r1, r2, r3, r4])
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> TransformationMatrix {
    let r1: [f32; 4] = [1.0, xy, xz, 0.0];
    let r2: [f32; 4] = [yx, 1.0, yz, 0.0];
    let r3: [f32; 4] = [zx, zy, 1.0, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    TransformationMatrix::from_2d_array([r1, r2, r3, r4])
}

pub fn view_transform(from: &Point, to: &Point, up: &Vector) -> TransformationMatrix {
    // TODO: Implement #norm for Array<f32, Ix1> so that we don't have to convert types
    let forward: Vector = to.clone() - from.clone();
    let forward = forward.norm();

    let left = forward.cross(up.norm());

    let true_up = left.cross(forward.clone());

    let r1: [f32; 4] = [left.x, left.y, left.z, 0.0];
    let r2: [f32; 4] = [true_up.x, true_up.y, true_up.z, 0.0];
    let r3: [f32; 4] = [-forward.x, -forward.y, -forward.z, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    let orientation = TransformationMatrix::from_2d_array([r1, r2, r3, r4]);

    orientation * translation(-from.x, -from.y, -from.z)
}
