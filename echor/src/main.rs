use clap::{App, Arg};

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Harsha <harshadixit12@gmail.com>")
        .about("A simple echo command")
        .arg(Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Omit printing newline")
                .takes_value(false)
        )
        .get_matches();

    let text = _matches.values_of_lossy("text").unwrap(); // text is a required arg, so safe to unwrap
    let omit_newline = _matches.is_present("omit_newline");
    let mut ending = "\n";
    if omit_newline {
        ending = "";
    } 

    println!("{}{}", text.join(" "), ending);
}
