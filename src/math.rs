use ndarray::*;

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

#[allow(dead_code)]
fn determinant_f32_2x2(matrix: &Array<f32, Ix2>) -> f32 {
        let a = matrix[[0,0]];
        let b = matrix[[0,1]];
        let c = matrix[[1,0]];
        let d = matrix[[1,1]];

        (a*d) - (b*c)
}

#[allow(dead_code)]
fn determinant_f32_n_x_n(matrix: &Array<f32, Ix2>) -> f32 {
    let mut determinant: f32 = 0.0;

    for c in 0..matrix.ncols() {
        let element = matrix[[0,c]];
        let cofactor = matrix.cofactor(0, c);

        determinant += cofactor * element;
    }

    determinant
}
