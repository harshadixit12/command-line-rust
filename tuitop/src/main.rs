mod terminal;
mod input;
mod render;

use std::time::{Duration, Instant};
use sysinfo::{System};

use terminal::*;
use input::spawn_input_thread;



fn main() {
    let original_term = enable_raw_mode();

    print!("{}", ALT_SCREEN_ENABLE);
    print!("{}", HIDE_CURSOR);

    let input_rx = spawn_input_thread();
    let mut sys = System::new_all();

    let tick_rate = Duration::from_millis(500);
    let mut last_tick = Instant::now();

    loop {
        if let Ok(byte) = input_rx.try_recv() {
            // Exit on 'q'
            if byte == b'q' {
                break;
            }
        }

        if last_tick.elapsed() >= tick_rate {
            sys.refresh_all();
            render::render(&sys);
            last_tick = Instant::now();
        }
    }

    print!("{}", SHOW_CURSOR);
    print!("{}", ALT_SCREEN_DISABLE);
    disable_raw_mode(original_term);
}
