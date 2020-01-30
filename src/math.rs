use ndarray::*;

pub trait Determinantable {
    type Unit;

    fn determinant(&self) -> Self::Unit;
}

impl Determinantable for Array<i32, Ix2> {
    type Unit = i32;

    fn determinant(&self) -> Self::Unit {
        if self.dim() == (2, 2) {
            determinant_i32_2x2(self)
        } else {
            unimplemented!("Not implemented for matrices of dimensions {:?}", self.dim())
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
