use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type CatResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run (config: Config) -> CatResult<()> {
    
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => {
                let mut line_number = 1;
                for line in file.lines() {
                    let line = line?;
                    if config.number_nonblank_lines {
                        if !line.trim().is_empty() {
                            println!("{} {}", line_number, line);
                            line_number += 1;
                        } else {
                            println!("  {}", line);
                        }
                        
                    } else if config.number_lines {
                        println!("{} {}", line_number, line);
                        line_number += 1;
                    } else {
                        println!("{}", line);
                        continue;
                    }
                }
                
            } 
        }
    }
    Ok(())
}

pub fn get_args() -> CatResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Harsha Dixit <harshadixit12@gmail.com")
        .about("Rust implementation of cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .multiple(true)
                .default_value("-")
                .min_values(1)
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number of lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number of non-blank lines")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(), // Safe since files is a required arg
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> CatResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
