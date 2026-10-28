mod board;
use board::{Dir, Grid, Tetro};

fn main() {
    let mut grid = Grid::new(10, 20);
    if grid.spawn(Tetro::I) {
        grid.print();
    }
    println!("---------");
    grid.shift(Dir::Down);
    grid.shift(Dir::Down);
    grid.shift(Dir::Down);
    grid.shift(Dir::Left);
    grid.shift(Dir::Left);
    grid.shift(Dir::Left);
    grid.print();
    grid.lock();
    grid.spawn(Tetro::I);
    println!("---------");
    grid.shift(Dir::Right);
    grid.shift(Dir::Right);
    grid.shift(Dir::Right);
    grid.shift(Dir::Down);
    grid.shift(Dir::Down);
    grid.shift(Dir::Down);
    grid.print();
    grid.lock();
    grid.spawn(Tetro::I);
    println!("---------");
    grid.shift(Dir::Down);
    grid.shift(Dir::Down);
    grid.shift(Dir::Left);
    grid.shift(Dir::Left);
    grid.shift(Dir::Left);
    grid.print();
    grid.lock();
    grid.spawn(Tetro::I);
    println!("---------");
    grid.shift(Dir::Right);
    grid.shift(Dir::Right);
    grid.shift(Dir::Right);
    grid.shift(Dir::Down);
    grid.shift(Dir::Down);
    grid.print();
    grid.lock();
    grid.spawn(Tetro::O);
    println!("---------");
    grid.shift(Dir::Down);
    grid.shift(Dir::Down);
    grid.print();
    grid.lock();
    grid.spawn(Tetro::I);
    println!("---------");
    grid.lock();
    grid.print();
    println!("AAA: {}", grid.clear());
    grid.print();
}
