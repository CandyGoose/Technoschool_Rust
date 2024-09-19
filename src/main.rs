fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let index = 3;

    if index < vec.len() {
        let removed_element = vec.remove(index);
        println!("Removed element: {}", removed_element);
        println!("Updated vector: {:?}", vec);
    } else {
        println!("Index out of bounds!");
    }
}
