use std::collections::HashSet;

fn all_unique_characters(input: &str) -> bool {
    let mut seen = HashSet::new();

    for ch in input.chars() {
        let lower_ch = ch.to_lowercase().next().unwrap();
        if seen.contains(&lower_ch) {
            return false;
        }
        seen.insert(lower_ch);
    }

    true
}

fn main() {
    let input1 = "abcd";
    let input2 = "abCdefA";
    let input3 = "aabcd";

    println!("{} — {}", input1, all_unique_characters(input1));
    println!("{} — {}", input2, all_unique_characters(input2));
    println!("{} — {}", input3, all_unique_characters(input3));
}
