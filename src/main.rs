use std::env;
use std::io::{self, BufRead};

#[derive(Debug)]
struct CutConfig {
    fields: Vec<usize>,
    delimiter: char,
    only_separated: bool,
}

impl CutConfig {
    fn new(args: &[String]) -> CutConfig {
        let mut config = CutConfig {
            fields: vec![],
            delimiter: '\t',
            only_separated: false,
        };

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-f" => {
                    i += 1;
                    config.fields = args[i]
                        .split(',')
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
                }
                "-d" => {
                    i += 1;
                    config.delimiter = args[i].chars().next().unwrap_or('\t');
                }
                "-s" => config.only_separated = true,
                _ => {}
            }
            i += 1;
        }

        config
    }
}

fn cut(config: CutConfig, reader: impl BufRead) {
    for line in reader.lines() {
        let line = line.unwrap();
        let columns: Vec<&str> = line.split(config.delimiter).collect();

        if config.only_separated && columns.len() < 2 {
            continue;
        }

        let mut selected_fields = vec![];
        for &field_index in &config.fields {
            if let Some(column) = columns.get(field_index - 1) {
                selected_fields.push(column.to_string());
            }
        }

        if !selected_fields.is_empty() {
            println!("{}", selected_fields.join(&config.delimiter.to_string()));
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = CutConfig::new(&args);

    let stdin = io::stdin();
    let reader = stdin.lock();
    cut(config, reader);
}
