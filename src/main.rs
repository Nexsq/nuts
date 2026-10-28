mod board;
mod term;

use board::{Dir, Grid, Tetro};
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;
use term::{Terminal, ansi};

fn main() {
    let terminal = Terminal::new();
    let mut frame_count = 0;
    let mut grid = Grid::new(10, 20);

    grid.spawn(Tetro::T);

    'main: loop {
        print!("{}", ansi::CLEAR_SCREEN);
        print!("{}", ansi::CURSOR_HOME);
        print!("Frames rendered: {}\r\n\n", frame_count);

        grid.print();
        stdout().flush().unwrap();

        frame_count += 1;

        while let Some(key) = terminal.get_key() {
            match key {
                b'q' => break 'main,
                b'a' => {
                    grid.shift(Dir::Left);
                }
                b'd' => {
                    grid.shift(Dir::Right);
                }
                b's' => {
                    grid.shift(Dir::Down);
                }
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(50));
    }
}
