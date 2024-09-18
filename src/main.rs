use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use dashmap::DashMap;

fn main() {
    // Способ 1: Используем Mutex + HashMap
    let data_mutex = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let data_mutex = Arc::clone(&data_mutex);
        let handle = thread::spawn(move || {
            let mut map = data_mutex.lock().unwrap();
            map.insert(i, i + 100);
            println!("{} -> {}", i, i + 100);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Mutex + HashMap: {:?}", *data_mutex.lock().unwrap());

    // Способ 2: Используем DashMap
    let data_dashmap = Arc::new(DashMap::new());

    let mut handles_dashmap = vec![];

    for i in 0..10 {
        let data_dashmap = Arc::clone(&data_dashmap);
        let handle = thread::spawn(move || {
            data_dashmap.insert(i, i + 100);
            println!("{} -> {}", i, i + 100);
        });
        handles_dashmap.push(handle);
    }

    for handle in handles_dashmap {
        handle.join().unwrap();
    }

    println!("Final DashMap: {:?}", data_dashmap);
}
