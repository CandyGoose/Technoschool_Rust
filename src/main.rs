use flume::unbounded;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let (tx, rx) = unbounded();

    let n_seconds = 5;

    let tx_clone = tx.clone();

    let sender_task = tokio::spawn(async move {
        let mut counter = 0;
        let timeout = time::sleep(Duration::from_secs(n_seconds));
        tokio::pin!(timeout);

        loop {
            tokio::select! {
                _ = &mut timeout => {
                    println!("Time is up, stopping sender.");
                    break;
                }
                _ = time::sleep(Duration::from_millis(500)) => {
                    counter += 1;
                    if tx_clone.send(counter).is_err() {
                        break;
                    }
                    println!("Sent: {}", counter);
                }
            }
        }
    });

    let receiver_task = tokio::spawn(async move {
        while let Ok(value) = rx.recv_async().await {
            println!("Received: {}", value);
        }
    });

    time::sleep(Duration::from_secs(n_seconds)).await;

    drop(tx);

    sender_task.await.unwrap();
    receiver_task.await.unwrap();

    println!("Program completed after {} seconds", n_seconds);
}
