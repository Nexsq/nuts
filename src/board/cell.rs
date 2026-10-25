use super::tetros::Tetro;

#[derive(Clone, Debug)]
pub struct Cell {
    piece: Tetro,
    locked: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { piece: Tetro::None, locked: true }
    }

    pub fn is_used(&self) -> bool {
        match self.piece {
            Tetro::None => false,
            _ => true,
        }
    }

    pub fn is_locked(&self) -> bool {
        match self.piece {
            Tetro::None => panic!("This cell is empty!"),
            _ => self.locked,
        }
    }

    pub fn lock(&mut self) {
        match self.piece {
            Tetro::None => panic!("This cell is empty!"),
            _ => self.locked = true,
        }
    }

    pub fn set_piece(&mut self, piece: Tetro) {
        self.piece = piece;
    }
}
