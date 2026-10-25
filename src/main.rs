mod board;
use board::{Grid, Tetro, Dir};

fn main() {
    let mut grid = Grid::new(10, 20);
    if grid.spawn(Tetro::L) {
        grid.print();
    }
    println!("---------");
    grid.shift(Dir::Down);
}
