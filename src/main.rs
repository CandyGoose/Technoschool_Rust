use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

#[derive(Debug)]
struct GrepConfig {
    after: usize,
    before: usize,
    context: usize,
    count: bool,
    ignore_case: bool,
    invert: bool,
    fixed: bool,
    line_num: bool,
    pattern: String,
}

impl GrepConfig {
    fn new(args: &[String]) -> GrepConfig {
        let mut config = GrepConfig {
            after: 0,
            before: 0,
            context: 0,
            count: false,
            ignore_case: false,
            invert: false,
            fixed: false,
            line_num: false,
            pattern: String::new(),
        };

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-A" => {
                    i += 1;
                    config.after = args[i].parse().unwrap_or(0);
                }
                "-B" => {
                    i += 1;
                    config.before = args[i].parse().unwrap_or(0);
                }
                "-C" => {
                    i += 1;
                    config.context = args[i].parse().unwrap_or(0);
                }
                "-c" => config.count = true,
                "-i" => config.ignore_case = true,
                "-v" => config.invert = true,
                "-F" => config.fixed = true,
                "-n" => config.line_num = true,
                _ => {
                    if config.pattern.is_empty() {
                        config.pattern = args[i].clone();
                    }
                }
            }
            i += 1;
        }

        if config.context > 0 {
            config.after = config.context;
            config.before = config.context;
        }

        config
    }
}

fn grep(config: GrepConfig, reader: impl BufRead) {
    let pattern = if config.ignore_case {
        config.pattern.to_lowercase()
    } else {
        config.pattern.clone()
    };

    let mut matches = vec![];
    let mut lines: Vec<String> = vec![];
    let mut count = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line_lowercase = if config.ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        let matched = if config.fixed {
            line_lowercase == pattern
        } else {
            line_lowercase.contains(&pattern)
        };

        let matched = if config.invert { !matched } else { matched };

        if matched {
            count += 1;
            matches.push(index);
        }

        lines.push(line);
    }

    if config.count {
        println!("{}", count);
        return;
    }

    for &match_index in &matches {
        let start = if match_index >= config.before {
            match_index - config.before
        } else {
            0
        };
        let end = std::cmp::min(match_index + config.after + 1, lines.len());

        for i in start..end {
            if config.line_num {
                print!("{}:", i + 1);
            }
            println!("{}", lines[i]);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <file> [options]", args[0]);
        process::exit(1);
    }

    let config = GrepConfig::new(&args);
    let file = File::open(&args[2]).expect("Could not open file");
    let reader = BufReader::new(file);

    grep(config, reader);
}
