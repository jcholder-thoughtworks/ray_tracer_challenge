use ndarray::*;

pub fn translation(x: f32, y: f32, z: f32) -> Array<f32, Ix2> {
    let array: Array<f32, Ix2> = arr2(&[
                                      [1.0, 0.0, 0.0, x],
                                      [0.0, 1.0, 0.0, y],
                                      [0.0, 0.0, 1.0, z],
                                      [0.0, 0.0, 0.0, 1.0],
    ]);

    array
}

pub fn scaling(x: f32, y: f32, z: f32) -> Array<f32, Ix2> {
    let array: Array<f32, Ix2> = arr2(&[
                                      [x, 0.0, 0.0, 0.0],
                                      [0.0, y, 0.0, 0.0],
                                      [0.0, 0.0, z, 0.0],
                                      [0.0, 0.0, 0.0, 1.0],
    ]);

    array
}

pub fn rotation_x(radians: f32) -> Array<f32, Ix2> {
    let cr = radians.cos();
    let sr = radians.sin();

    let r1: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
    let r2: [f32; 4] = [0.0, cr, -sr, 0.0];
    let r3: [f32; 4] = [0.0, sr, cr, 0.0];
    let r4: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

    arr2(&[r1, r2, r3, r4])
}
