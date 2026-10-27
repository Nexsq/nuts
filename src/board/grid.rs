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

    pub fn copy(&self) -> Grid {
        let mut out = self.clone();
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell.used() && !cell.locked() {
                    out.matrix[i][j].clear();
                }
            }
        }
        out
    }

    pub fn spawn(&mut self, piece: super::tetros::Tetro) -> bool {
        match piece {
            super::tetros::Tetro::None => false,
            _ => {
                let mut temp = self.clone();
                let mid = temp.width / 2;
                let shape = piece.shape();
                for (i, row) in shape.iter().enumerate() {
                    for (j, &cell) in row.iter().enumerate() {
                        if cell {
                            let x = i;
                            let y =
                                mid as usize + j - (shape[0].len() as f32 / 2.0).ceil() as usize;
                            if temp.matrix[x][y].used() {
                                return false;
                            } else {
                                temp.matrix[x][y].set_piece(piece.clone());
                            }
                        }
                    }
                }
                *self = temp;
                true
            }
        }
    }

    pub fn shift(&mut self, dir: super::dirs::Dir) -> bool {
        let mut temp = self.copy();
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell.used() && !cell.locked() {
                    match dir {
                        super::dirs::Dir::Down => {
                            if i + 1 < temp.height as usize && !temp.matrix[i + 1][j].locked() {
                                temp.matrix[i + 1][j] = self.matrix[i][j].clone();
                            } else {
                                return false;
                            }
                        }
                        super::dirs::Dir::Left => {
                            if j > 0 && !temp.matrix[i][j - 1].locked() {
                                temp.matrix[i][j - 1] = self.matrix[i][j].clone();
                            } else {
                                return false;
                            }
                        }
                        super::dirs::Dir::Right => {
                            if j + 1 < temp.width as usize && !temp.matrix[i][j + 1].locked() {
                                temp.matrix[i][j + 1] = self.matrix[i][j].clone();
                            } else {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        *self = temp;
        true
    }

    pub fn lock(&mut self) {
        for i in self.matrix.iter_mut() {
            for j in i.iter_mut() {
                if j.used() && !j.locked() {
                    j.lock();
                }
            }
        }
    }

    pub fn clear(&mut self) -> u8 {
        // todo: this
        for i in self.matrix.iter_mut() {
            let mut full = true;
            for j in i.iter_mut() {
                if j.used() && j.locked() {
                    full = false
                }
            }
        }
        0
    }

    pub fn print(&self) {
        for row in self.matrix.iter() {
            for cell in row.iter() {
                print!(
                    "{}",
                    if cell.used() {
                        if cell.locked() {
                            "\x1b[;m[]"
                        } else {
                            "\x1b[;31m[]"
                        }
                    } else {
                        "\x1b[;m. "
                    }
                );
            }
            print!("\r\n");
        }
    }
}
