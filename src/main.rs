use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut unique_strings: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if !unique_strings.contains(&line) {
            unique_strings.push(line);
        }
    }

    for string in unique_strings {
        println!("{}", string);
    }
}
