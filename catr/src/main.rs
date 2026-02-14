use catr::{run, get_args};
fn main() {
    if let Err(r) = get_args().and_then(run) {
        eprintln!("Error: {}", r);
        std::process::exit(1);
    }
}
