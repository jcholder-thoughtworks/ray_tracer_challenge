use ndarray::*;

pub trait Determinantable {
    type Unit;

    fn determinant(&self) -> Self::Unit;
}

pub trait Submatrixable {
    type Unit;

    fn submatrix(&self, row: usize, col: usize) -> Self;
}

impl Determinantable for Array<i32, Ix2> {
    type Unit = i32;

    fn determinant(&self) -> Self::Unit {
        if self.dim() == (2, 2) {
            determinant_i32_2x2(self)
        } else {
            unimplemented!("Matrices with dimensions {:?} are unsupported", self.dim())
        }
    }
}

fn determinant_i32_2x2(matrix: &Array<i32, Ix2>) -> i32{
        let a = matrix[[0,0]];
        let b = matrix[[0,1]];
        let c = matrix[[1,0]];
        let d = matrix[[1,1]];

        (a*d) - (b*c)
}

impl Determinantable for Array<f32, Ix2> {
    type Unit = f32;

    fn determinant(&self) -> Self::Unit {
        let a = self[[0,0]];
        let b = self[[0,1]];
        let c = self[[1,0]];
        let d = self[[1,1]];

        (a*d) - (b*c)
    }
}

impl Submatrixable for Array<i32, Ix2> {
    type Unit = i32;

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let rows_to_keep: Vec<usize> = (0..(self.nrows())).filter(|n| *n != row).collect();
        let cols_to_keep: Vec<usize> = (0..(self.ncols())).filter(|n| *n != col).collect();

        self.select(Axis(0), &rows_to_keep).select(Axis(1), &cols_to_keep)
    }
}
