// Contains the ANSI codes we will use
use std::os::unix::io::AsRawFd;
use termios::*;

pub const CLEAR: &str = "\x1b[2J";
pub const MOVE_HOME: &str = "\x1b[H";
pub const HIDE_CURSOR: &str = "\x1b[?25l";
pub const SHOW_CURSOR: &str = "\x1b[?25h";
pub const ALT_SCREEN_ENABLE: &str = "\x1b[?1049h";
pub const ALT_SCREEN_DISABLE: &str = "\x1b[?1049l";

pub fn enable_raw_mode() -> Termios {
    let fd = std::io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(fd).unwrap();
    let original = termios.clone();

    // turn off canonical mode and echo, use bitwise operator
    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(fd, TCSANOW, &termios).unwrap();

    original
}

pub fn disable_raw_mode(original: Termios) {
    let fd = std::io::stdin().as_raw_fd();
    tcsetattr(fd, TCSANOW, &original).unwrap();
}