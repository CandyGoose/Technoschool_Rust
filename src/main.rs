use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: wc [option] <filename>");
        return Ok(());
    }

    let mut count_type = "words";
    let mut file_name = "";

    if args.len() == 2 {
        file_name = &args[1];
    } else if args.len() == 3 {
        count_type = &args[1];
        file_name = &args[2];
    }

    let path = Path::new(file_name);
    let file = File::open(&path)?;

    match count_type {
        "-c" => {
            let char_count = count_chars(file)?;
            println!("{}", char_count);
        }
        "-l" => {
            let line_count = count_lines(file)?;
            println!("{}", line_count);
        }
        "-w" | _ => {
            let word_count = count_words(file)?;
            println!("{}", word_count);
        }
    }

    Ok(())
}

fn count_chars(mut file: File) -> io::Result<usize> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.chars().count())
}

fn count_lines(file: File) -> io::Result<usize> {
    let reader = io::BufReader::new(file);
    Ok(reader.lines().count())
}

fn count_words(file: File) -> io::Result<usize> {
    let reader = io::BufReader::new(file);
    let mut word_count = 0;
    for line in reader.lines() {
        let line = line?;
        word_count += line.split_whitespace().count();
    }
    Ok(word_count)
}
