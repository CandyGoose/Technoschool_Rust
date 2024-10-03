use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::exit;

#[derive(Debug)]
enum SortType {
    Lexicographical,
    Numerical,
    Month,
    HumanReadable,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file> [options]", args[0]);
        exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let mut column_key: Option<usize> = None;
    let mut reverse = false;
    let mut unique = false;
    let mut sort_type = SortType::Lexicographical;
    let mut check_sorted = false;
    let mut ignore_trailing_spaces = false;

    for arg in &args[3..] {
        match arg.as_str() {
            "-k" => {
                if let Some(index) = args.iter().position(|x| x == "-k") {
                    if index + 1 < args.len() {
                        column_key = args[index + 1].parse().ok();
                    }
                }
            }
            "-n" => sort_type = SortType::Numerical,
            "-M" => sort_type = SortType::Month,
            "-h" => sort_type = SortType::HumanReadable,
            "-r" => reverse = true,
            "-u" => unique = true,
            "-c" => check_sorted = true,
            "-b" => ignore_trailing_spaces = true,
            _ => {}
        }
    }

    let file = File::open(input_file).expect("Unable to open input file");
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    if unique {
        let set: HashSet<String> = lines.drain(..).collect();
        lines = set.into_iter().collect();
    }

    if ignore_trailing_spaces {
        for line in lines.iter_mut() {
            *line = line.trim_end().to_string();
        }
    }

    if check_sorted {
        if is_sorted(&lines, &sort_type, column_key, reverse) {
            println!("Data is sorted.");
            exit(0);
        } else {
            println!("Data is not sorted.");
            exit(1);
        }
    }

    sort_lines(&mut lines, &sort_type, column_key, reverse);

    let mut output = File::create(output_file).expect("Unable to create output file");
    for line in lines {
        writeln!(output, "{}", line).expect("Unable to write to output file");
    }
}

fn is_sorted(lines: &[String], sort_type: &SortType, column_key: Option<usize>, reverse: bool) -> bool {
    let mut iter = lines.iter().peekable();
    while let Some(current) = iter.next() {
        if let Some(next) = iter.peek() {
            let cmp = compare_lines(current, next, sort_type, column_key);
            if (reverse && cmp == Ordering::Less) || (!reverse && cmp == Ordering::Greater) {
                return false;
            }
        }
    }
    true
}

fn sort_lines(lines: &mut Vec<String>, sort_type: &SortType, column_key: Option<usize>, reverse: bool) {
    lines.sort_by(|a, b| {
        let cmp = compare_lines(a, b, sort_type, column_key);
        if reverse {
            cmp.reverse()
        } else {
            cmp
        }
    });
}

fn compare_lines(a: &str, b: &str, sort_type: &SortType, column_key: Option<usize>) -> Ordering {
    let key_a = get_column(a, column_key);
    let key_b = get_column(b, column_key);

    match sort_type {
        SortType::Lexicographical => key_a.cmp(&key_b),
        SortType::Numerical => {
            let num_a = extract_first_number(&key_a).unwrap_or(f64::MIN);
            let num_b = extract_first_number(&key_b).unwrap_or(f64::MIN);
            num_a.partial_cmp(&num_b).unwrap_or(Ordering::Equal)
        }
        SortType::Month => {
            let months = [
                "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December",
            ];
            let idx_a = months.iter().position(|&m| m == key_a).unwrap_or(usize::MAX);
            let idx_b = months.iter().position(|&m| m == key_b).unwrap_or(usize::MAX);
            idx_a.cmp(&idx_b)
        }
        SortType::HumanReadable => {
            let num_a = parse_human_readable(&key_a);
            let num_b = parse_human_readable(&key_b);
            num_a.partial_cmp(&num_b).unwrap_or(Ordering::Equal)
        }
    }
}

fn extract_first_number(s: &str) -> Option<f64> {
    s.split_whitespace()
        .filter_map(|word| word.parse::<f64>().ok())
        .next()
}

fn get_column(line: &str, column_key: Option<usize>) -> String {
    if let Some(key) = column_key {
        line.split_whitespace().nth(key - 1).unwrap_or(line).to_string()
    } else {
        line.to_string()
    }
}

fn parse_human_readable(s: &str) -> f64 {
    let suffixes = [("K", 1_000.0), ("M", 1_000_000.0), ("G", 1_000_000_000.0), ("T", 1_000_000_000_000.0)];
    for &(suffix, multiplier) in &suffixes {
        if s.ends_with(suffix) {
            let num = s.trim_end_matches(suffix).parse::<f64>().unwrap_or(0.0);
            return num * multiplier;
        }
    }
    s.parse::<f64>().unwrap_or(0.0)
}
