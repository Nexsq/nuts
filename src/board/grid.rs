use super::matrix::Matrix;

#[derive(Debug)]
pub struct Grid {
    matrix: Matrix,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            matrix: Matrix::new(),
        }
    }
}
