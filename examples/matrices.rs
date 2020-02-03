use ndarray::*;

use ray_tracer_challenge::math::*;

fn main() {
    invert_identity_matrix();

    println!();

    multiply_by_inverse();

    println!();

    compare_inverse_transpose();

    println!();

    tuples_and_identity_matrices();
}

fn invert_identity_matrix() {
    println!("Invert the identity matrix!\n");

    let identity_matrix: Array<f32, Ix2> = Array::eye(4);

    println!("{:?}\n", identity_matrix);

    let inverted = identity_matrix.inverse();

    println!("{:?}\n", inverted);
}

fn multiply_by_inverse() {
    println!("Multiply a matrix by its inverse!\n");

    let orig: Array<f32, Ix2> = arr2(&[[0.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

    println!("{:?}\n", orig);

    let inverted = orig.inverse();

    println!("{:?}\n", inverted.rounded());

    let product = orig.dot(&inverted);

    println!("{:?}\n", product.rounded());
}

fn compare_inverse_transpose() {
    println!("How does the inverse of a transpose compare to the transpose of an inverse?\n");

    let orig: Array<f32, Ix2> = arr2(&[[0.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

    println!("{:?}\n", orig);

    let it = orig.transposed().inverse();

    println!("Inverse of a transpose:");
    println!("{:?}\n", it.rounded());

    let ti = orig.inverse().transposed();

    println!("Transpose of an inverse:");
    println!("{:?}\n", ti.rounded());
}

fn tuples_and_identity_matrices() {
    println!("Multiplying tuples by identity matrices!");

    let identity_matrix: Array<f32, Ix2> = Array::eye(4);

    let tuple = (5, 6, 7, 8);

    println!("{:?}\n", tuple);

    let tuple_matrix = arr1(&[5.0, 6.0, 7.0, 8.0]);

    println!("{:?}\n", identity_matrix.dot(&tuple_matrix));

    let mut modified_identity_matrix = identity_matrix.clone();
    modified_identity_matrix[[0, 0]] = 4.0;

    println!("But what if we change the matrix to {:?}?\n", modified_identity_matrix);

    println!("{:?}\n", modified_identity_matrix.dot(&tuple_matrix));

    let mut modified_identity_matrix = identity_matrix.clone();
    modified_identity_matrix[[0, 1]] = 4.0;

    println!("But what if we change the matrix to {:?}?\n", modified_identity_matrix);

    println!("{:?}\n", modified_identity_matrix.dot(&tuple_matrix));
}
