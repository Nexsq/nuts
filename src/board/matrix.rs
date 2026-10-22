use super::cell::Cell;

#[derive(Debug)]
pub struct Matrix {
    row: Vec<Cell>,
    col: Vec<Cell>,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            row: Vec::new(),
            col: Vec::new(),
        }
    }
}
