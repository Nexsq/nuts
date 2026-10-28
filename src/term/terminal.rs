use super::ansi;
use super::raw;
use std::io::{Read, Write, stdin, stdout};
use std::sync::mpsc;
use std::thread;

pub struct Terminal {
    _raw: raw::RawMode,
    rx: mpsc::Receiver<u8>,
}

impl Terminal {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut buffer = [0u8; 1];
            loop {
                if stdin().read(&mut buffer).is_ok() {
                    if tx.send(buffer[0]).is_err() {
                        break;
                    }
                }
            }
        });

        print!("{}", ansi::ENTER_ALTERNATE_SCREEN);
        print!("{}", ansi::HIDE_CURSOR);
        stdout().flush().unwrap();

        Self {
            _raw: raw::RawMode::enable(),
            rx,
        }
    }

    pub fn get_key(&self) -> Option<u8> {
        self.rx.try_recv().ok()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        print!("{}", ansi::LEAVE_ALTERNATE_SCREEN);
        print!("{}", ansi::SHOW_CURSOR);
        stdout().flush().unwrap();
    }
}
