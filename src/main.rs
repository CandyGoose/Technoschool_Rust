use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    let n = 10;
    let (tx1, mut rx1) = mpsc::channel(10); 
    let (tx2, mut rx2) = mpsc::channel(10);

    let producer = task::spawn(async move {
        for i in 1..=n {
            tx1.send(i).await.unwrap();
            time::sleep(Duration::from_millis(500)).await;
        }
    });

    let processor = task::spawn(async move {
        while let Some(num) = rx1.recv().await {
            let square = num * num;
            tx2.send(square).await.unwrap();
        }
    });

    let consumer = task::spawn(async move {
        while let Some(square) = rx2.recv().await {
            println!("Squared value: {}", square);
        }
    });

    producer.await.unwrap();
    processor.await.unwrap();
    consumer.await.unwrap();
}
