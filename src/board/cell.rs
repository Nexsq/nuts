use super::tetros::Tetro;

#[derive(Clone, Debug)]
pub struct Cell {
    piece: Tetro,
}

impl Cell {
    pub fn new() -> Cell {
        Cell { piece: Tetro::None }
    }

    pub fn is_used(&self) -> bool {
        match self.piece {
            Tetro::None => false,
            _ => true,
        }
    }

    pub fn set_piece(&mut self, piece: Tetro) {
        self.piece = piece;
    }
}
