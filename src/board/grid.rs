use super::matrix::Matrix;

#[derive(Debug)]
pub struct Grid {
    pub matrix: Matrix,
}

impl Grid {
    pub fn new(w: u8, h: u8) -> Grid {
        Grid {
            matrix: Matrix::new(w, h),
        }
    }

    pub fn print(&self) {
        for row in self.matrix.row.iter() {
            println!("{:?}", row);
        }
    }
}
