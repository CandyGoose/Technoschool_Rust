use std::thread;

fn main() {
    let n = 10;
    let numbers: Vec<i32> = (1..=n).collect();

    let mut handles = vec![];

    for num in numbers {
        let handle = thread::spawn(move || {
            let square = num * num;
            println!("{}^2 = {}", num, square);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
