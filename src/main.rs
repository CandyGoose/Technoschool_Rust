use std::collections::HashMap;

fn find_anagrams(words: &[String]) -> HashMap<String, Vec<String>> {
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        let lower_word = word.to_lowercase();

        let mut sorted_chars: Vec<char> = lower_word.chars().collect();
        sorted_chars.sort_unstable();
        let sorted_word = sorted_chars.into_iter().collect::<String>();

        anagram_map.entry(sorted_word)
            .or_insert_with(Vec::new)
            .push(lower_word.clone());
    }

    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for (_, mut anagrams) in anagram_map {
        if anagrams.len() > 1 {
            anagrams.sort();
            let first_word = anagrams[0].clone();
            result.insert(first_word, anagrams);
        }
    }

    result
}

fn main() {
    let words = vec![
        "пятак".to_string(),
        "пятка".to_string(),
        "слиток".to_string(),
        "столик".to_string(),
        "кот".to_string(),
        "ток".to_string(),
        "окт".to_string(),
        "тяпка".to_string(),
        "листок".to_string(),
    ];

    let anagrams = find_anagrams(&words);

    for (key, anagram_set) in &anagrams {
        println!("{}: {:?}", key, anagram_set);
    }
}
