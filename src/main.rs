mod board;
use board::{Grid, Tetro};

fn main() {
    let mut grid = Grid::new(10, 20);
    grid.spawn(Tetro::I);
    grid.print();
}
