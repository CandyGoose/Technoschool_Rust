use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

struct Counter {
    value: AtomicUsize,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: AtomicUsize::new(0),
        }
    }

    fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    fn get_value(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }
}

fn main() {
    let counter = Arc::new(Counter::new());

    let mut handles = vec![];

    // Создаем потоки
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.get_value());
}
