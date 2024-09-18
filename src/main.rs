use std::thread;
use std::sync::mpsc;

fn main() {
    let n = 10;
    let numbers: Vec<i32> = (1..=n).collect();

    let (tx, rx) = mpsc::channel();

    for num in numbers {
        let tx = tx.clone();
        thread::spawn(move || {
            let square = num * num;
            tx.send(square).unwrap();
        });
    }

    let mut sum = 0;
    for _ in 1..=n {
        sum += rx.recv().unwrap();
    }

    println!("Sum of squares: {}", sum);
}
