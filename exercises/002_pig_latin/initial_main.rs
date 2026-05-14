// PIG LATIN CONVERTER
// This program converts English words into Pig Latin following two rules:
// 1. For words starting with a consonant, move the first letter to the end and add "ay".
// 2. For words starting with a vowel, simply add "hay" at the end.

fn main() {
    let words = vec!["first", "apple", "hello", "orange", "rust", "umbrella"];

    println!("Pig Latin Converter:\n");

    for word in words {
        let pig_latin = to_pig_latin(word);
        println!("{:12} → {}", word, pig_latin);
    }
}

/// Convert a word to Pig Latin
/// Consonant-first words: move first letter to end and add "ay"
/// Vowel-first words: add "hay" at the end
pub fn to_pig_latin(word: &str) -> String {
    // Your implementation here
    String::new()
}
