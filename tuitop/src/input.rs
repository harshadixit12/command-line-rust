use std::sync::mpsc;
use std::thread;
use std::io::Read;

pub fn spawn_input_thread() -> mpsc::Receiver<u8> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stdin = std::io::stdin();

        for byte in stdin.bytes() {
            if let Ok(b) = byte {
                tx.send(b).unwrap();
            }
        }
    });

    rx
}