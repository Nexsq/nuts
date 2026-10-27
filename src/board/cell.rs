use super::tetros::Tetro;

#[derive(Clone, Debug)]
pub struct Cell {
    piece: Tetro,
    locked: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            piece: Tetro::None,
            locked: false,
        }
    }

    pub fn clear(&mut self) {
        self.piece = Tetro::None;
        self.locked = false;
    }

    pub fn used(&self) -> bool {
        match self.piece {
            Tetro::None => false,
            _ => true,
        }
    }

    pub fn locked(&self) -> bool {
        self.locked
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
