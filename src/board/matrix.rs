use super::cell::Cell;

#[derive(Debug)]
pub struct Matrix {
    pub row: Vec<Cell>,
    pub col: Vec<Cell>,
}

impl Matrix {
    pub fn new(w: u8, h: u8) -> Matrix {
        Matrix {
            row: vec![Cell::new(); w as usize],
            col: vec![],
        }
    }
}
