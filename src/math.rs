use std::ops;
use std::convert::From;

use ndarray::*;

use super::{Point, Vector, round};

pub mod transforms;

pub trait RaytracerMatrix: Clone {
    type Unit;

    fn determinant(&self) -> Self::Unit;

    fn submatrix(&self, row: usize, col: usize) -> Self;

    fn minor(&self, row: usize, col: usize) -> Self::Unit {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> Self::Unit {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 0 { // if even
            minor
        } else {
            Self::negate(minor)
        }
    }

    fn invertible(&self) -> bool;

    fn negate(number: Self::Unit) -> Self::Unit;

    fn inverse(&self) -> Self;

    fn rounded(&self) -> Self;

    fn transposed(&self) -> Self;
}

impl RaytracerMatrix for Array<i32, Ix2> {
    type Unit = i32;

    fn determinant(&self) -> Self::Unit {
        match self.dim() {
            (2, 2) => determinant_i32_2x2(self),
            _ => determinant_i32_n_x_n(self),
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let rows_to_keep: Vec<usize> = (0..(self.nrows())).filter(|n| *n != row).collect();
        let cols_to_keep: Vec<usize> = (0..(self.ncols())).filter(|n| *n != col).collect();

        self.select(Axis(0), &rows_to_keep).select(Axis(1), &cols_to_keep)
    }

    fn invertible(&self) -> bool {
        self.determinant() != 0
    }

    fn negate(number: Self::Unit) -> Self::Unit {
        -number
    }

    fn inverse(&self) -> Self {
        // TODO: Put this safety check behind a debug flag so that we can disable it for
        // optimization
        if ! self.invertible() {
            panic!("Cannot invert a matrix with a determinant of zero");
        }

        let mut inverted = Array2::zeros(self.dim());

        for row_i in 0..self.nrows() {
            for col_i in 0..self.ncols() {
                let cofactor = self.cofactor(row_i, col_i);

                inverted[[col_i, row_i]] = cofactor / self.determinant();
            }
        }

        inverted
    }

    fn rounded(&self) -> Self {
        self.clone()
    }

    // TODO: Figure out how to avoid this manual copying.
    // Might need to change these implementations to target ArrayBase instead of Array
    fn transposed(&self) -> Self {
        let transposed_view = self.t();
        let mut transposed_matrix = Array2::zeros(self.dim());

        for r in 0..(self.nrows()) {
            for c in 0..(self.ncols()) {
                transposed_matrix[[r, c]] = transposed_view[[r, c]];
            }
        }

        transposed_matrix
    }
}

impl RaytracerMatrix for Array<f32, Ix2> {
    type Unit = f32;

    fn determinant(&self) -> Self::Unit {
        match self.dim() {
            (2, 2) => determinant_f32_2x2(self),
            _ => determinant_f32_n_x_n(self),
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let rows_to_keep: Vec<usize> = (0..(self.nrows())).filter(|n| *n != row).collect();
        let cols_to_keep: Vec<usize> = (0..(self.ncols())).filter(|n| *n != col).collect();

        self.select(Axis(0), &rows_to_keep).select(Axis(1), &cols_to_keep)
    }

    fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn negate(number: Self::Unit) -> Self::Unit {
        -number
    }

    fn inverse(&self) -> Self {
        // TODO: Put this safety check behind a debug flag so that we can disable it for
        // optimization
        if ! self.invertible() {
            panic!("Cannot invert a matrix with a determinant of zero");
        }

        let mut inverted = Array2::zeros(self.dim());

        for row_i in 0..self.nrows() {
            for col_i in 0..self.ncols() {
                let cofactor = self.cofactor(row_i, col_i);

                inverted[[col_i, row_i]] = cofactor / self.determinant();
            }
        }

        inverted
    }

    fn rounded(&self) -> Self {
        let mut rounded_matrix = self.clone();

        for el in rounded_matrix.iter_mut() {
            *el = round(*el);
        }

        rounded_matrix
    }

    // TODO: Figure out how to avoid this manual copying.
    // Might need to change these implementations to target ArrayBase instead of Array
    fn transposed(&self) -> Self {
        let transposed_view = self.t();
        let mut transposed_matrix = Array2::zeros(self.dim());

        for r in 0..(self.nrows()) {
            for c in 0..(self.ncols()) {
                transposed_matrix[[r, c]] = transposed_view[[r, c]];
            }
        }

        transposed_matrix
    }
}

fn determinant_i32_2x2(matrix: &Array<i32, Ix2>) -> i32 {
        let a = matrix[[0,0]];
        let b = matrix[[0,1]];
        let c = matrix[[1,0]];
        let d = matrix[[1,1]];

        (a*d) - (b*c)
}

fn determinant_i32_n_x_n(matrix: &Array<i32, Ix2>) -> i32 {
    let mut determinant: i32 = 0;

    for c in 0..matrix.ncols() {
        let element = matrix[[0,c]];
        let cofactor = matrix.cofactor(0, c);

        determinant += cofactor * element;
    }

    determinant
}

fn determinant_f32_2x2(matrix: &Array<f32, Ix2>) -> f32 {
        let a = matrix[[0,0]];
        let b = matrix[[0,1]];
        let c = matrix[[1,0]];
        let d = matrix[[1,1]];

        (a*d) - (b*c)
}

fn determinant_f32_n_x_n(matrix: &Array<f32, Ix2>) -> f32 {
    let mut determinant: f32 = 0.0;

    for c in 0..matrix.ncols() {
        let element = matrix[[0,c]];
        let cofactor = matrix.cofactor(0, c);

        determinant += cofactor * element;
    }

    determinant
}

// TODO: Double-check all these magic numbers. Knowledge of p(1) vs v(1) belongs alongside
// those structs rather than here

impl From<Point> for Array<f32, Ix1> {
    fn from(item: Point) -> Self {
        arr1(&[item.x, item.y, item.z, 1.0])
    }
}

impl From<Array<f32, Ix1>> for Point {
    fn from(item: Array<f32, Ix1>) -> Self {
        Point::new(item[[0]], item[[1]], item[[2]])
    }
}

impl From<Vector> for Array<f32, Ix1> {
    fn from(item: Vector) -> Self {
        arr1(&[item.x, item.y, item.z, 0.0])
    }
}

impl From<Array<f32, Ix1>> for Vector {
    fn from(item: Array<f32, Ix1>) -> Self {
        Vector::new(item[[0]], item[[1]], item[[2]])
    }
}

// TODO: Feels like we should be able to use a `where` clause here
impl ops::Mul<Array<f32, Ix2>> for Point {
    type Output = Self;

    fn mul(self, rhs: Array<f32, Ix2>) -> Self::Output {
        rhs.dot(&arr1(&[self.x, self.y, self.z, 1.0])).into()
    }
}

// TODO: Feels like we should be able to use a `where` clause here
impl ops::Mul<Point> for Array<f32, Ix2> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        self.dot(&arr1(&[rhs.x, rhs.y, rhs.z, 1.0])).into()
    }
}

// TODO: Feels like we should be able to use a `where` clause here
impl ops::Mul<Array<f32, Ix2>> for Vector {
    type Output = Self;

    fn mul(self, rhs: Array<f32, Ix2>) -> Self::Output {
        rhs.dot(&arr1(&[self.x, self.y, self.z, 0.0])).into()
    }
}

// TODO: I could probably consolidate some of these operations with default trait implementations.
// Both structs can convert to 1D 4x arrays, after all

// TODO: Feels like we should be able to use a `where` clause here
impl ops::Mul<Vector> for Array<f32, Ix2> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.dot(&arr1(&[rhs.x, rhs.y, rhs.z, 0.0])).into()
    }
}
