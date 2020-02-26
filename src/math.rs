use std::ops;

use ndarray::*;

use super::{round, Point, Vector};

pub mod transforms;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AnyMatrix {
    M2x2(Matrix2x2),
    M3x3(Matrix3x3),
    M4x4(Matrix4x4),
    M4x1(Matrix4x1),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix2x2 {
    values: [f32; 4],
}

impl Matrix2x2 {
    pub fn new(values: [f32; 4]) -> Self {
        Self { values }
    }

    pub fn default() -> Self {
        Self::new([0.0; 4])
    }

    pub fn determinant(&self) -> f32 {
        let a = self[[0, 0]];
        let b = self[[0, 1]];
        let c = self[[1, 0]];
        let d = self[[1, 1]];

        (a * d) - (b * c)
    }

    pub fn minor(&self) -> f32 {
        self.determinant()
    }

    pub fn cofactor(&self) -> f32 {
        self.minor()
    }
}

impl ops::Index<[usize; 2]> for Matrix2x2 {
    type Output = f32;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let row = index[0];
        let col = index[1];

        &self.values[row * 2 + col]
    }
}

impl ops::IndexMut<[usize; 2]> for Matrix2x2 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let row = index[0];
        let col = index[1];

        &mut self.values[row * 2 + col]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3x3 {
    values: [f32; 9],
}

impl Matrix3x3 {
    pub fn new(values: [f32; 9]) -> Self {
        Self { values }
    }

    pub fn default() -> Self {
        Self::new([0.0; 9])
    }

    pub fn determinant(&self) -> f32 {
        let mut determinant: f32 = 0.0;

        for c in 0..3 {
            let element = self[[0, c]];
            let cofactor = self.cofactor(0, c);

            determinant += cofactor * element;
        }

        determinant
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let mut sub = Matrix2x2::default();

        for r in 0..3 {
            if r == row {
                continue;
            }

            for c in 0..3 {
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

    // Credit to https://www.mathsisfun.com/algebra/matrix-determinant.html for this optimization
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        let mut i = 0;
        let mut m: [f32; 4] = [0.0; 4];

        for r in 0..=2 {
            if r == row {
                continue;
            }

            for c in 0..=2 {
                if c == col {
                    continue;
                }

                m[i] = self[[r, c]];
                i = i + 1;
            }
        }

        (m[0] * m[3]) - (m[1] * m[2])
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 0 {
            // if even
            minor
        } else {
            -minor
        }
    }
}

impl ops::Index<[usize; 2]> for Matrix3x3 {
    type Output = f32;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let row = index[0];
        let col = index[1];

        &self.values[row * 3 + col]
    }
}

impl ops::IndexMut<[usize; 2]> for Matrix3x3 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let row = index[0];
        let col = index[1];

        &mut self.values[row * 3 + col]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix4x1 {
    values: [f32; 4],
}

impl Matrix4x1 {
    pub fn new(values: [f32; 4]) -> Self {
        Self { values }
    }

    pub fn default() -> Self {
        Self { values: [0.0, 0.0, 0.0, 0.0] }
    }
}

impl ops::Index<usize> for Matrix4x1 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl ops::Mul<Matrix4x1> for Matrix4x1 {
    type Output = f32;

    fn mul(self, rhs: Matrix4x1) -> Self::Output {
        let v1 = self.values;
        let v2 = rhs.values;

        (v1[0] * v2[0]) + (v1[1] * v2[1]) + (v1[2] * v2[2]) + (v1[3] * v2[3])
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix4x4 {
    values: [f32; 16],
}

impl Matrix4x4 {
    pub fn new(values: [f32; 16]) -> Self {
        Self { values }
    }

    pub fn from_2d_array(values: [[f32; 4]; 4]) -> Self {
        Self::new([
                  values[0][0],
                  values[0][1],
                  values[0][2],
                  values[0][3],
                  values[1][0],
                  values[1][1],
                  values[1][2],
                  values[1][3],
                  values[2][0],
                  values[2][1],
                  values[2][2],
                  values[2][3],
                  values[3][0],
                  values[3][1],
                  values[3][2],
                  values[3][3],
        ])
    }

    pub fn default() -> Self {
        Self::new([0.0; 16])
    }

    pub fn row(&self, r: usize) -> Matrix4x1 {
        assert!(r < 4, "Invalid row index: {}", r);

        let mut v = [0.0; 4];

        for index in 0..4 {
            v[index] = self.values[index + 4 * r];
        }

        Matrix4x1::new(v)
    }

    pub fn col(&self, c: usize) -> Matrix4x1 {
        assert!(c < 4, "Invalid column index: {}", c);

        let v = [
            self.values[0 + c],
            self.values[4 + c],
            self.values[8 + c],
            self.values[12 + c],
        ];

        Matrix4x1::new(v)
    }

    pub fn identity() -> Self {
        Self::new([
                  1.0, 0.0, 0.0, 0.0,
                  0.0, 1.0, 0.0, 0.0,
                  0.0, 0.0, 1.0, 0.0,
                  0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn determinant(&self) -> f32 {
        let mut determinant: f32 = 0.0;

        for c in 0..4 {
            let element = self[[0, c]];
            let cofactor = self.cofactor(0, c);

            determinant += cofactor * element;
        }

        determinant
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);

        if (row + col) % 2 == 0 {
            // if even
            minor
        } else {
            -minor
        }
    }

    // Credit to https://www.mathsisfun.com/algebra/matrix-determinant.html for this optimization
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        let mut index = 0;
        let mut m: [f32; 9] = [0.0; 9];

        for r in 0..=3 {
            if r == row {
                continue;
            }

            for c in 0..=3 {
                if c == col {
                    continue;
                }

                m[index] = self[[r, c]];
                index = index + 1;
            }
        }

        m[0] * (m[4] * m[8] - m[5] * m[7])
            - m[1] * (m[3] * m[8] - m[5] * m[6])
            + m[2] * (m[3] * m[7] - m[4] * m[6])
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        let mut sub = Matrix3x3::default();

        for r in 0..4 {
            if r == row {
                continue;
            }

            for c in 0..4 {
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

    pub fn inverse(&self) -> Self {
        if !self.invertible() {
            panic!("Cannot invert a matrix with a determinant of zero");
        }

        let mut inverted = Matrix4x4::default();

        for row_i in 0..4 {
            for col_i in 0..4 {
                let cofactor = self.cofactor(row_i, col_i);

                inverted[[col_i, row_i]] = cofactor / self.determinant();
            }
        }

        inverted
    }

    pub fn transposed(&self) -> Self {
        let v = self.values;

        Self::new([
                  v[0], v[4], v[8], v[12],
                  v[1], v[5], v[9], v[13],
                  v[2], v[6], v[10], v[14],
                  v[3], v[7], v[11], v[15],
        ])
    }

    pub fn rounded(&self) -> Self {
        let mut values = [0.0; 16];

        for (index, value) in self.values.iter().enumerate() {
            values[index] = round(*value);
        }

        Self::new(values)
    }
}

impl ops::Index<[usize; 2]> for Matrix4x4 {
    type Output = f32;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let row = index[0];
        let col = index[1];

        &self.values[row * 4 + col]
    }
}

impl ops::IndexMut<[usize; 2]> for Matrix4x4 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let row = index[0];
        let col = index[1];

        &mut self.values[row * 4 + col]
    }
}

impl ops::Mul<Point> for &Matrix4x4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let rhs: [f32; 4] = [rhs.x(), rhs.y(), rhs.z(), 1.0]; // TODO: replace with Into<Matrix4x1>
        // TODO: These indices for Matrix4x4 may be wrong
        let v = self.values;
        let values: [f32; 4] = [
            (v[0] * rhs[0]) + (v[1] * rhs[1]) + (v[2] * rhs[2]) + (v[3] * rhs[3]),
            (v[4] * rhs[0]) + (v[5] * rhs[1]) + (v[6] * rhs[2]) + (v[7] * rhs[3]),
            (v[8] * rhs[0]) + (v[9] * rhs[1]) + (v[10] * rhs[2]) + (v[11] * rhs[3]),
            (v[12] * rhs[0]) + (v[13] * rhs[1]) + (v[14] * rhs[2]) + (v[15] * rhs[3]),
        ];

        Self::Output::new(values[0], values[1], values[2])
    }
}

impl ops::Mul<Vector> for &Matrix4x4 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let rhs: [f32; 4] = [rhs.x(), rhs.y(), rhs.z(), 0.0]; // TODO: replace with Into<Matrix4x1>
        let v = self.values;
        let values: [f32; 4] = [
            (v[0] * rhs[0]) + (v[1] * rhs[1]) + (v[2] * rhs[2]) + (v[3] * rhs[3]),
            (v[4] * rhs[0]) + (v[5] * rhs[1]) + (v[6] * rhs[2]) + (v[7] * rhs[3]),
            (v[8] * rhs[0]) + (v[9] * rhs[1]) + (v[10] * rhs[2]) + (v[11] * rhs[3]),
            (v[12] * rhs[0]) + (v[13] * rhs[1]) + (v[14] * rhs[2]) + (v[15] * rhs[3]),
        ];

        Self::Output::new(values[0], values[1], values[2])
    }
}

impl ops::Mul<Point> for Matrix4x4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        let rhs: [f32; 4] = [rhs.x(), rhs.y(), rhs.z(), 1.0]; // TODO: replace with Into<Matrix4x1>
        let v = self.values;
        let values: [f32; 4] = [
            (v[0] * rhs[0]) + (v[1] * rhs[1]) + (v[2] * rhs[2]) + (v[3] * rhs[3]),
            (v[4] * rhs[0]) + (v[5] * rhs[1]) + (v[6] * rhs[2]) + (v[7] * rhs[3]),
            (v[8] * rhs[0]) + (v[9] * rhs[1]) + (v[10] * rhs[2]) + (v[11] * rhs[3]),
            (v[12] * rhs[0]) + (v[13] * rhs[1]) + (v[14] * rhs[2]) + (v[15] * rhs[3]),
        ];

        Self::Output::new(values[0], values[1], values[2])
    }
}

impl ops::Mul<Array<f32, Ix1>> for &Matrix4x4 {
    type Output = Array<f32, Ix1>;

    fn mul(self, rhs: Array<f32, Ix1>) -> Self::Output {
        let rhs: [f32; 4] = [rhs[[0]], rhs[[1]], rhs[[2]], rhs[[3]]]; // TODO: replace with Into<Matrix4x1>
        let v = self.values;
        let values: [f32; 4] = [
            (v[0] * rhs[0]) + (v[1] * rhs[1]) + (v[2] * rhs[2]) + (v[3] * rhs[3]),
            (v[4] * rhs[0]) + (v[5] * rhs[1]) + (v[6] * rhs[2]) + (v[7] * rhs[3]),
            (v[8] * rhs[0]) + (v[9] * rhs[1]) + (v[10] * rhs[2]) + (v[11] * rhs[3]),
            (v[12] * rhs[0]) + (v[13] * rhs[1]) + (v[14] * rhs[2]) + (v[15] * rhs[3]),
        ];

        arr1(&values)
    }
}

impl ops::Mul<Matrix4x1> for Matrix4x4 {
    type Output = Matrix4x1;

    fn mul(self, rhs: Matrix4x1) -> Self::Output {
        let v = self.values;
        let values: [f32; 4] = [
            (v[0] * rhs[0]) + (v[1] * rhs[1]) + (v[2] * rhs[2]) + (v[3] * rhs[3]),
            (v[4] * rhs[0]) + (v[5] * rhs[1]) + (v[6] * rhs[2]) + (v[7] * rhs[3]),
            (v[8] * rhs[0]) + (v[9] * rhs[1]) + (v[10] * rhs[2]) + (v[11] * rhs[3]),
            (v[12] * rhs[0]) + (v[13] * rhs[1]) + (v[14] * rhs[2]) + (v[15] * rhs[3]),
        ];

        Self::Output::new(values)
    }
}

impl ops::Mul<Array<f32, Ix1>> for Matrix4x4 {
    type Output = Array<f32, Ix1>;

    fn mul(self, rhs: Array<f32, Ix1>) -> Self::Output {
        let rhs: [f32; 4] = [rhs[[0]], rhs[[1]], rhs[[2]], rhs[[3]]]; // TODO: replace with Into<Matrix4x1>
        // TODO: These indices for Matrix4x4 may be wrong
        let v = self.values;
        let values: [f32; 4] = [
            (v[0] * rhs[0]) + (v[1] * rhs[1]) + (v[2] * rhs[2]) + (v[3] * rhs[3]),
            (v[4] * rhs[0]) + (v[5] * rhs[1]) + (v[6] * rhs[2]) + (v[7] * rhs[3]),
            (v[8] * rhs[0]) + (v[9] * rhs[1]) + (v[10] * rhs[2]) + (v[11] * rhs[3]),
            (v[12] * rhs[0]) + (v[13] * rhs[1]) + (v[14] * rhs[2]) + (v[15] * rhs[3]),
        ];

        arr1(&values)
    }
}

impl ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut values: [f32; 16] = [0.0; 16];

        for ri in 0..4 {
            let r = self.row(ri);

            for ci in 0..4 {
                let c = rhs.col(ci);

                values[ri * 4 + ci] = r * c;
            }
        }

        Self::Output::new(values)
    }
}

pub trait RaytracerMatrix: Clone {
    type Unit;

    fn determinant(&self) -> Self::Unit;

    fn submatrix(&self, row: usize, col: usize) -> Self;

    fn minor(&self, row: usize, col: usize) -> Self::Unit;

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

    fn minor(&self, row: usize, col: usize) -> Self::Unit {
        match self.dim() {
            (3, 3) => minor_3x3(self, row, col),
            (4, 4) => minor_4x4(self, row, col),
            _ => panic!("Calculation of minors for matrix with dimensions of {:?} are not supported", self.dim()),
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

// Credit to https://www.mathsisfun.com/algebra/matrix-determinant.html for this optimization
fn minor_3x3(matrix: &Array<f32, Ix2>, row: usize, col: usize) -> f32 {
    let mut i = 0;
    let mut m: [f32; 4] = [0.0; 4];

    for r in 0..=2 {
        if r == row {
            continue;
        }

        for c in 0..=2 {
            if c == col {
                continue;
            }

            m[i] = matrix[[r, c]];
            i = i + 1;
        }
    }

    (m[0] * m[3]) - (m[1] * m[2])
}

// Credit to https://www.mathsisfun.com/algebra/matrix-determinant.html for this optimization
fn minor_4x4(matrix: &Array<f32, Ix2>, row: usize, col: usize) -> f32 {
    let mut index = 0;
    let mut m: [f32; 9] = [0.0; 9];

    for r in 0..=3 {
        if r == row {
            continue;
        }

        for c in 0..=3 {
            if c == col {
                continue;
            }

            m[index] = matrix[[r, c]];
            index = index + 1;
        }
    }

    m[0] * (m[4] * m[8] - m[5] * m[7])
        - m[1] * (m[3] * m[8] - m[5] * m[6])
        + m[2] * (m[3] * m[7] - m[4] * m[6])
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
