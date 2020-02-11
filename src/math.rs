use std::convert::From;
use std::ops;

use ndarray::*;

use super::{round, Point, Vector};

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

        if (row + col) % 2 == 0 {
            // if even
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

impl RaytracerMatrix for Array<f32, Ix2> {
    type Unit = f32;

    fn determinant(&self) -> Self::Unit {
        match self.dim() {
            (2, 2) => determinant_f32_2x2(self),
            _ => determinant_f32_n_x_n(self),
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let orig_rows = self.nrows();
        let orig_cols = self.ncols();

        let mut sub = Self::default((orig_rows - 1, orig_cols - 1));

        for r in 0..orig_rows {
            if r == row {
                continue;
            }

            for c in 0..orig_cols {
                if c == col {
                    continue;
                }

                let ri = if r > row { r - 1 } else { r };

                let ci = if c > col { c - 1 } else { c };

                sub[[ri, ci]] = self[[r, c]];
            }
        }

        sub
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
        if !self.invertible() {
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

fn determinant_f32_2x2(matrix: &Array<f32, Ix2>) -> f32 {
    let a = matrix[[0, 0]];
    let b = matrix[[0, 1]];
    let c = matrix[[1, 0]];
    let d = matrix[[1, 1]];

    (a * d) - (b * c)
}

fn determinant_f32_n_x_n(matrix: &Array<f32, Ix2>) -> f32 {
    let mut determinant: f32 = 0.0;

    for c in 0..matrix.ncols() {
        let element = matrix[[0, c]];
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

// TODO: Feels like we should be able to use a `where` clause here
impl ops::Mul<Array<f32, Ix2>> for Point {
    type Output = Self;

    fn mul(self, rhs: Array<f32, Ix2>) -> Self::Output {
        rhs.dot(&arr1(&[self.x, self.y, self.z, 1.0])).into()
    }
}

// TODO: Feels like we should be able to use a `where` clause here
impl ops::Mul<Point> for &Array<f32, Ix2> {
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
impl ops::Mul<Vector> for &Array<f32, Ix2> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.dot(&arr1(&[rhs.x, rhs.y, rhs.z, 0.0])).into()
    }
}
