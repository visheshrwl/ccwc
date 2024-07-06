use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::io::stdin;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: ccwc [-c|-l|-w|-m] <filename>");
        std::process::exit(1);
    }

    let filename = if args.len() == 2 {
        None
    } else {
        Some(&args[2])
    };

    if let Some(filename) = filename {
        match args[1].as_str() {
            "-c" => {
                let mut file = File::open(filename)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                let byte_count = buffer.len();
                println!("{:>7} {}", byte_count, filename);
            },
            "-l" => {
                let file = File::open(filename)?;
                let reader = BufReader::new(file);
                let line_count = reader.lines().count();
                println!("{:>7} {}", line_count, filename);
            },
            "-w" => {
                let file = File::open(filename)?;
                let reader = BufReader::new(file);
                let word_count = reader.lines()
                    .map(|line| line.unwrap().split_whitespace().count())
                    .sum::<usize>();
                println!("{:>7} {}", word_count, filename);
            },
            "-m" => {
                let mut file = File::open(filename)?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                let char_count = buffer.chars().count();
                println!("{:>7} {}", char_count, filename);
            },
            _ => {
                eprintln!("Usage: ccwc [-c|-l|-w|-m] <filename>");
                std::process::exit(1);
            }
        }
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        let (line_count, word_count, byte_count) = reader.lines().fold((0, 0, 0), |(lines, words, bytes), line| {
            let line = line.unwrap();
            (lines + 1, words + line.split_whitespace().count(), bytes + line.len())
        });
        println!("{:>7} {:>7} {:>7}", line_count, word_count, byte_count);
    }

    Ok(())
}
