fn reverse_words(input: &str) -> String {
    input.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

fn main() {
    let input = "snow dog sun";
    let reversed = reverse_words(input);
    println!("{} — {}", input, reversed);
}
