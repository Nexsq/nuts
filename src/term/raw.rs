#[cfg(unix)]
use std::process::Command;

#[cfg(unix)]
pub struct RawMode {
    original_state: String,
}

#[cfg(unix)]
impl RawMode {
    pub fn enable() -> Self {
        let output = Command::new("stty").arg("-g").output().unwrap();
        let original_state = String::from_utf8(output.stdout).unwrap().trim().to_string();
        Command::new("stty")
            .args(["raw", "-echo"])
            .status()
            .unwrap();
        Self { original_state }
    }
}

#[cfg(unix)]
impl Drop for RawMode {
    fn drop(&mut self) {
        Command::new("stty")
            .args(["-raw", "echo"])
            .status()
            .unwrap();
        Command::new("stty")
            .arg(&self.original_state)
            .status()
            .unwrap();
    }
}

#[cfg(windows)]
pub struct RawMode {
    original_in_mode: u32,
    original_out_mode: u32,
}

#[cfg(windows)]
extern "system" {
    fn GetStdHandle(nStdHandle: u32) -> *mut std::ffi::c_void;
    fn GetConsoleMode(hConsoleHandle: *mut std::ffi::c_void, lpMode: *mut u32) -> i32;
    fn SetConsoleMode(hConsoleHandle: *mut std::ffi::c_void, dwMode: u32) -> i32;
}

#[cfg(windows)]
impl RawMode {
    pub fn enable() -> Self {
        unsafe {
            let h_in = GetStdHandle(-10i32 as u32);
            let h_out = GetStdHandle(-11i32 as u32);
            let mut in_mode = 0;
            let mut out_mode = 0;

            GetConsoleMode(h_in, &mut in_mode);
            GetConsoleMode(h_out, &mut out_mode);

            let original_in_mode = in_mode;
            let original_out_mode = out_mode;

            in_mode &= !(0x0004 | 0x0002);
            out_mode |= 0x0004;

            SetConsoleMode(h_in, in_mode);
            SetConsoleMode(h_out, out_mode);

            Self {
                original_in_mode,
                original_out_mode,
            }
        }
    }
}

#[cfg(windows)]
impl Drop for RawMode {
    fn drop(&mut self) {
        unsafe {
            let h_in = GetStdHandle(-10i32 as u32);
            let h_out = GetStdHandle(-11i32 as u32);
            SetConsoleMode(h_in, self.original_in_mode);
            SetConsoleMode(h_out, self.original_out_mode);
        }
    }
}
