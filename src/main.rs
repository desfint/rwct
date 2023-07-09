use std::{env, fs, io::Read, path};

use clap::{arg, ArgMatches, Command};

struct FileStats<'a> {
    name: &'a str,
    words: u32,
    lines: u32,
    size: u32,
    chars: u32,
}

fn main() {
    let matches = Command::new("rwct")
        .args(&[
            arg!(lines: -l "count the lines in a text"),
            arg!(words: -w "count the words in the text"),
            arg!(chars: -m "count the characters in the text"),
            arg!(bytes: -c "measure the size of the text in bytes"),
            arg!(file: [text_file]),
        ])
        .get_matches();

    let mut stats = FileStats {
        name: "",
        words: 0,
        lines: 0,
        size: 0,
        chars: 0,
    };

    let mut buf = String::new();
    let options = get_options(&matches);

    match matches.get_one::<String>("file") {
        Some(input_file) => {
            let file_path = path::Path::new(input_file);
            fs::File::open(file_path)
                .unwrap()
                .read_to_string(&mut buf)
                .unwrap();
            stats.name = file_path.file_name().unwrap().to_str().unwrap();

            if env::args().len() > 2 {
                get_file_stats(buf, &mut stats);
                print_stats(&options, &stats);
            } else {
                get_file_stats(buf, &mut stats);
                print_stats(&["lines", "words", "bytes"], &stats);
            }
            {}
        }
        None => {
            std::io::stdin().read_to_string(&mut buf).unwrap();

            get_file_stats(buf, &mut stats);
            print_stats(&options, &stats)
        }
    }
}

fn get_file_stats(buffer: String, file_stats: &mut FileStats) {
    file_stats.chars = buffer.chars().count() as u32;

    let mut is_word = false;
    let buff_bytes = buffer.as_bytes();
    file_stats.size = buff_bytes.len() as u32;

    for byte in buff_bytes {
        if byte.is_ascii_whitespace() {
            if is_word {
                file_stats.words += 1;
            }

            if *byte == b'\n' {
                file_stats.lines += 1;
            }

            is_word = false;
        } else {
            is_word = true;
        }
    }
}

fn get_options<'a>(matches: &ArgMatches) -> Vec<&'a str> {
    let mut options = Vec::new();

    if *matches.get_one::<bool>("lines").unwrap() {
        options.push("lines")
    }

    if *matches.get_one::<bool>("words").unwrap() {
        options.push("words")
    }

    if *matches.get_one::<bool>("chars").unwrap() {
        options.push("chars")
    }

    if *matches.get_one::<bool>("bytes").unwrap() {
        options.push("bytes")
    }

    options
}

fn print_stats(options: &[&str], stats: &FileStats) {
    if options.contains(&"lines") {
        print!(" {}", stats.lines);
    }

    if options.contains(&"words") {
        print!(" {}", stats.words)
    }

    if options.contains(&"chars") {
        print!(" {}", stats.chars);
    }

    if options.contains(&"bytes") {
        print!(" {}", stats.size);
    }

    println!(" {}", stats.name);
}
