use super::cell::Cell;

#[derive(Debug)]
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

    pub fn spawn(&mut self, piece: super::tetros::Tetro) {
        match piece {
            super::tetros::Tetro::None => {}
            _ => {
                let mid = self.width / 2;
                let shape = piece.shape();
                for (i, row) in shape.iter().enumerate() {
                    for (j, &cell) in row.iter().enumerate() {
                        if cell {
                            let x = i;
                            let y = mid + j as u8 - (shape[0].len() as f32 / 2.0).ceil() as u8;
                            self.matrix[x][y as usize].set_piece(piece.clone());
                        }
                    }
                }
            }
        }
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
