use flume::{Receiver};
use std::time::Duration;
use tokio::signal;
use tokio::time;

#[tokio::main]
async fn main() {
    let (tx, rx) = flume::unbounded();

    let worker_count = 4;

    let mut worker_tasks = vec![];
    for i in 0..worker_count {
        let rx = rx.clone();
        let worker_id = i + 1;
        let worker_task = tokio::spawn(worker(rx, worker_id));
        worker_tasks.push(worker_task);
    }

    let tx_task = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    println!("Ctrl+C detected, shutting down...");
                    break;
                }
                _ = time::sleep(Duration::from_millis(500)) => {
                    counter += 1;
                    if tx.send(Some(format!("Message {}", counter))).is_err() {
                        break;
                    }
                }
            }
        }

        for _ in 0..worker_count {
            let _ = tx.send(None);
        }
    });

    tx_task.await.unwrap();

    for task in worker_tasks {
        task.await.unwrap();
    }

    println!("All workers have shut down.");
}

async fn worker(rx: Receiver<Option<String>>, worker_id: usize) {
    while let Ok(message) = rx.recv_async().await {
        match message {
            Some(msg) => println!("Worker {} received: {}", worker_id, msg),
            None => {
                println!("Worker {} shutting down.", worker_id);
                break;
            }
        }
    }
}
