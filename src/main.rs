use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let worker_count = 4;

    let (tx, rx) = mpsc::channel();

    let rx = Arc::new(Mutex::new(rx));

    thread::spawn(move || {
        let mut counter = 0;
        loop {
            counter += 1;
            tx.send(format!("Message {}", counter)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let mut workers = vec![];
    for i in 0..worker_count {
        let rx = Arc::clone(&rx);
        let worker_id = i + 1;
        let handle = thread::spawn(move || {
            loop {
                let message = rx.lock().unwrap().recv();
                match message {
                    Ok(message) => {
                        println!("Worker {} received: {}", worker_id, message);
                    }
                    Err(_) => break,
                }
            }
        });
        workers.push(handle);
    }

    for worker in workers {
        worker.join().unwrap();
    }
}
