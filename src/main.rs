// Copyright (c) 2024 Adam Burucs. MIT license.

use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Stats {
    bytes: u64,
    words: u64,
    lines: u64,
    chars: u64,
}

#[derive(Debug)]
struct Options {
    print_bytes: bool,
    print_lines: bool,
    print_words: bool,
    print_chars: bool,
}

fn calculate_stats<R: BufRead>(reader: R) -> Stats {
    let mut bytes_count: u64 = 0;
    let mut lines_count: u64 = 0;
    let mut words_count: u64 = 0;
    let mut chars_count: u64 = 0;

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                bytes_count += line.len() as u64;
                lines_count += 1;
                chars_count += line.chars().count() as u64;

                let mut in_word = false;
                for char_read in line.chars() {
                    if char_read.is_whitespace() {
                        if in_word {
                            words_count += 1;
                            in_word = false;
                        }
                    } else {
                        in_word = true;
                    }
                }

                if in_word {
                    words_count += 1;
                }
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                break;
            }
        }
    }

    Stats {
        bytes: bytes_count,
        words: words_count,
        lines: lines_count,
        chars: chars_count,
    }
}

fn main() {
    let matches = Command::new("ccwc")
        .version("0.1.0")
        .about("Counts words and various attributes of files")
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Count bytes")
                .takes_value(false),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("Count lines")
                .takes_value(false),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("Count words")
                .takes_value(false),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("Count characters")
                .takes_value(false),
        )
        .arg(
            Arg::new("file")
                .help("File to process")
                .multiple_occurrences(false)
                .required(true),
        )
        .get_matches();

    let file_path = matches.value_of("file").expect("File path is required");

    let mut options = Options {
        print_bytes: matches.is_present("bytes"),
        print_lines: matches.is_present("lines"),
        print_words: matches.is_present("words"),
        print_chars: matches.is_present("chars"),
    };

    if !options.print_bytes && !options.print_lines && !options.print_words && !options.print_chars
    {
        options.print_chars = true;
        options.print_bytes = true;
        options.print_words = true;
        options.print_lines = true;
    }

    let path = Path::new(file_path);
    let file = File::open(path).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    let stats = calculate_stats(reader);
    if options.print_lines {
        println!("Lines: {}", stats.lines);
    }
    if options.print_words {
        println!("Words: {}", stats.words);
    }
    if options.print_bytes {
        println!("Bytes: {}", stats.bytes);
    }
    if options.print_chars {
        println!("Characters: {}", stats.chars);
    }
}
