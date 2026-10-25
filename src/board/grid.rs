use super::cell::Cell;

#[derive(Debug, Clone)]
pub struct Grid {
    matrix: Vec<Vec<Cell>>,
    width: u8,
    height: u8,
}

impl Grid {
    pub fn new(w: u8, h: u8) -> Grid {
        if w < 4 || h < 4 {
            panic!("Board too small!")
        }
        Grid {
            matrix: vec![vec![Cell::new(); w as usize]; h as usize],
            width: w,
            height: h,
        }
    }

    pub fn spawn(&mut self, piece: super::tetros::Tetro) -> bool {
        match piece {
            super::tetros::Tetro::None => { return false }
            _ => {
                let mut copy = self.clone();
                let mid = copy.width / 2;
                let shape = piece.shape();
                for (i, row) in shape.iter().enumerate() {
                    for (j, &cell) in row.iter().enumerate() {
                        if cell {
                            let x = i;
                            let y = mid as usize + j - (shape[0].len() as f32 / 2.0).ceil() as usize;
                            if copy.matrix[x][y].is_used() {
                                return false
                            } else {
                                copy.matrix[x][y].set_piece(piece.clone());
                            }
                        }
                    }
                }
                *self = copy;
                return true
            }
        }
    }

    pub fn shift(&self, dir: super::dirs::Dir) -> bool {
        // match dir typeshi
        let mut copy = self.clone();
        for i in self.matrix.iter() {
            println!("{:?}", i)
        }
        return true
    }

    pub fn print(&self) {
        for row in self.matrix.iter() {
            for cell in row.iter() {
                print!("{}", if cell.is_used() { "[]" } else { ". " });
            }
            println!();
        }
    }
}
