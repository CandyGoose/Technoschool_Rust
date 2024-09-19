fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let input = "главрыба";
    let reversed = reverse_string(input);
    println!("Original: {}", input);
    println!("Reversed: {}", reversed);
}
