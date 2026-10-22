#[derive(Debug)]
pub struct Cell {
    lit: bool
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            lit: false,
        }
    }
}
