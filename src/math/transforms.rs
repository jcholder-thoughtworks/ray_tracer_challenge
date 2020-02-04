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
