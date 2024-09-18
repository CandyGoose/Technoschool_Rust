use flume::unbounded;
use tokio::time::{self, Duration};
use tokio_util::sync::CancellationToken;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let (tx, rx) = unbounded();
    let n_seconds = 5;
    let tx_clone = tx.clone();
    let cancel_token = Arc::new(CancellationToken::new());

    let cancel_token_clone = cancel_token.clone();
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
                _ = cancel_token_clone.cancelled() => {
                    println!("Sender task cancelled.");
                    break;
                }
            }
        }
    });

    let cancel_token_clone = cancel_token.clone();
    let receiver_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                message = rx.recv_async() => {
                    if let Ok(value) = message {
                        println!("Received: {}", value);
                    } else {
                        println!("Channel closed, stopping receiver.");
                        break;
                    }
                }
                _ = cancel_token_clone.cancelled() => {
                    println!("Receiver task cancelled.");
                    break;
                }
            }
        }
    });

    time::sleep(Duration::from_secs(n_seconds)).await;

    cancel_token.cancel();

    drop(tx);

    sender_task.await.unwrap();
    receiver_task.await.unwrap();

    println!("Program completed after {} seconds", n_seconds);
}
