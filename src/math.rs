use ndarray::*;

pub trait RaytracerMatrix {
    type Unit;

    fn determinant(&self) -> Self::Unit;

    fn submatrix(&self, row: usize, col: usize) -> Self;

    fn minor(&self, row: usize, col: usize) -> Self::Unit;
}

impl RaytracerMatrix for Array<i32, Ix2> {
    type Unit = i32;

    fn determinant(&self) -> Self::Unit {
        if self.dim() == (2, 2) {
            determinant_i32_2x2(self)
        } else {
            unimplemented!("Matrices with dimensions {:?} are unsupported", self.dim())
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let rows_to_keep: Vec<usize> = (0..(self.nrows())).filter(|n| *n != row).collect();
        let cols_to_keep: Vec<usize> = (0..(self.ncols())).filter(|n| *n != col).collect();

        self.select(Axis(0), &rows_to_keep).select(Axis(1), &cols_to_keep)
    }

    fn minor(&self, row: usize, col: usize) -> Self::Unit {
        self.submatrix(row, col).determinant()
    }
}

impl RaytracerMatrix for Array<f32, Ix2> {
    type Unit = f32;

    fn determinant(&self) -> Self::Unit {
        let a = self[[0,0]];
        let b = self[[0,1]];
        let c = self[[1,0]];
        let d = self[[1,1]];

        (a*d) - (b*c)
    }

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let rows_to_keep: Vec<usize> = (0..(self.nrows())).filter(|n| *n != row).collect();
        let cols_to_keep: Vec<usize> = (0..(self.ncols())).filter(|n| *n != col).collect();

        self.select(Axis(0), &rows_to_keep).select(Axis(1), &cols_to_keep)
    }

    fn minor(&self, row: usize, col: usize) -> Self::Unit {
        self.submatrix(row, col).determinant()
    }
}

fn determinant_i32_2x2(matrix: &Array<i32, Ix2>) -> i32 {
        let a = matrix[[0,0]];
        let b = matrix[[0,1]];
        let c = matrix[[1,0]];
        let d = matrix[[1,1]];

        (a*d) - (b*c)
}

#[allow(dead_code)]
fn determinant_f32_2x2(matrix: &Array<f32, Ix2>) -> f32 {
        let a = matrix[[0,0]];
        let b = matrix[[0,1]];
        let c = matrix[[1,0]];
        let d = matrix[[1,1]];

        (a*d) - (b*c)
}
