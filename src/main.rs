use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use serde_json::json;
use std::collections::HashMap;

fn count_letters(text: &str, start: usize, end: usize) -> HashMap<char, usize> {
    let mut frequency: HashMap<char, usize> = HashMap::new();

    for c in text.chars().skip(start).take(end - start) {
        let lower_c = c.to_ascii_lowercase();
        if lower_c.is_ascii_alphabetic() {
            *frequency.entry(lower_c).or_insert(0) += 1;
        }
    }

    frequency
}

fn merge_maps(a: &mut HashMap<char, usize>, b: &HashMap<char, usize>) {
    for (key, value) in b {
        *a.entry(*key).or_insert(0) += value;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [-t num_threads] <file>", args[0]);
        return;
    }

    let mut num_threads = 1;
    let mut file_name = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-t" => {
                i += 1;
                num_threads = args[i].parse().unwrap_or(1);
            }
            _ => {
                file_name = args[i].clone();
            }
        }
        i += 1;
    }

    let text = fs::read_to_string(&file_name).expect("Unable to read file");

    let start_time = Instant::now();

    let text_len = text.len();
    let chunk_size = text_len / num_threads;

    let frequency_map = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            text_len
        } else {
            (i + 1) * chunk_size
        };

        let text_chunk = text.clone();
        let frequency_map_clone = Arc::clone(&frequency_map);

        let handle = thread::spawn(move || {
            let local_freq = count_letters(&text_chunk, start, end);
            let mut freq_map = frequency_map_clone.lock().unwrap();
            merge_maps(&mut freq_map, &local_freq);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_time = start_time.elapsed();

    let freq_map = frequency_map.lock().unwrap();

    let result_json = json!({
        "elapsed": format!("{:.3} s", elapsed_time.as_secs_f64()),
        "result": *freq_map,
    });

    println!("{}", serde_json::to_string_pretty(&result_json).unwrap());
}
