use sysinfo::System;
use crate::terminal::{CLEAR, MOVE_HOME};

pub fn render(sys: &System) {
    print!("{}", MOVE_HOME);
    print!("{}", CLEAR);

    let cpu = sys.global_cpu_usage();
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();

    println!("TUI Top (press 'q' to quit)");
    println!("============================");
    println!("CPU Usage: {:.2}%", cpu);
    println!("Memory: {}/{}", used_mem, total_mem);
}
