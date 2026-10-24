#[derive(Clone, Debug)]
pub enum Tetro {
    None,
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Tetro {
    pub fn shape(&self) -> Vec<Vec<bool>> {
        match self {
            Tetro::None => vec![vec![false]],
            Tetro::I => vec![vec![true, true, true, true]],
            Tetro::O => vec![vec![true, true], vec![true, true]],
            Tetro::T => vec![vec![false, true, false], vec![true, true, true]],
            Tetro::S => vec![vec![false, true, true], vec![true, true, false]],
            Tetro::Z => vec![vec![true, true, false], vec![false, true, true]],
            Tetro::J => vec![vec![true, false, false], vec![true, true, true]],
            Tetro::L => vec![vec![false, false, true], vec![true, true, true]],
        }
    }
}
