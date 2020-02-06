use std::rc::Rc;

use ndarray::*;

pub type TransformationMatrix = Array<f32, Ix2>;


#[derive(Debug)]
pub enum TransformationType {
    Translation,
    Scaling,
    RotationX,
    RotationY,
    RotationZ,
    Shearing,
}

#[derive(Debug)]
pub struct Transformation {
    pub ttype: TransformationType, // ttype instead of type to avoid keyword
    pub matrix: Rc<TransformationMatrix>,
}

pub fn translation(x: f32, y: f32, z: f32) -> Transformation {
    let array: Array<f32, Ix2> = arr2(&[
                                      [1.0, 0.0, 0.0, x],
                                      [0.0, 1.0, 0.0, y],
                                      [0.0, 0.0, 1.0, z],
                                      [0.0, 0.0, 0.0, 1.0],
    ]);

    Transformation {
        ttype: TransformationType::Translation,
        matrix: Rc::new(array),
    }
}

pub fn scaling(x: f32, y: f32, z: f32) -> Transformation {
    let array: Array<f32, Ix2> = arr2(&[
                                      [x, 0.0, 0.0, 0.0],
                                      [0.0, y, 0.0, 0.0],
                                      [0.0, 0.0, z, 0.0],
                                      [0.0, 0.0, 0.0, 1.0],
    ]);

    Transformation {
        ttype: TransformationType::Scaling,
        matrix: Rc::new(array),
    }
}

pub fn rotation_x(radians: f32) -> Transformation {
    let cr = radians.cos();
    let sr = radians.sin();

    let r1: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
    let r2: [f32; 4] = [0.0, cr, -sr, 0.0];
    let r3: [f32; 4] = [0.0, sr, cr, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    Transformation {
        ttype: TransformationType::RotationX,
        matrix: Rc::new(arr2(&[r1, r2, r3, r4])),
    }
}

pub fn rotation_y(radians: f32) -> Transformation {
    let cr = radians.cos();
    let sr = radians.sin();

    let r1: [f32; 4] = [cr, 0.0, sr, 0.0];
    let r2: [f32; 4] = [0.0, 1.0, 0.0, 0.0];
    let r3: [f32; 4] = [-sr, 0.0, cr, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    Transformation {
        ttype: TransformationType::RotationY,
        matrix: Rc::new(arr2(&[r1, r2, r3, r4])),
    }
}

pub fn rotation_z(radians: f32) -> Transformation {
    let cr = radians.cos();
    let sr = radians.sin();

    let r1: [f32; 4] = [cr, -sr, 0.0, 0.0];
    let r2: [f32; 4] = [sr, cr, 0.0, 0.0];
    let r3: [f32; 4] = [0.0, 0.0, 1.0, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    Transformation {
        ttype: TransformationType::RotationZ,
        matrix: Rc::new(arr2(&[r1, r2, r3, r4])),
    }
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Transformation {
    let r1: [f32; 4] = [1.0, xy, xz, 0.0];
    let r2: [f32; 4] = [yx, 1.0, yz, 0.0];
    let r3: [f32; 4] = [zx, zy, 1.0, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    Transformation {
        ttype: TransformationType::Shearing,
        matrix: Rc::new(arr2(&[r1, r2, r3, r4])),
    }
}
