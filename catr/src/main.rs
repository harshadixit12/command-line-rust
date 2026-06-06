use catr::{get_args, run};
fn main() {
    if let Err(r) = get_args().and_then(run) {
        eprintln!("Error: {}", r);
        std::process::exit(1);
    }
}
