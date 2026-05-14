// VOWEL COUNTER
// This program counts vowels and consonants in a text string.
// Vowels are: a, e, i, o, u (case-insensitive)
// Only count alphabetic characters, ignore numbers and punctuation.

fn main() {
    let texts = vec![
        "Hello World",
        "Rust Programming",
        "aeiou AEIOU",
        "The quick brown fox jumps over the lazy dog",
    ];
    
    for text in texts {
        let (vowels, consonants) = count_vowels_and_consonants(text);
        println!("'{}' → Vowels: {}, Consonants: {}", text, vowels, consonants);
    }
}

/// Count vowels and consonants in a string
/// Returns: (vowel_count, consonant_count)
pub fn count_vowels_and_consonants(text: &str) -> (usize, usize) {
    // Your implementation here
    (0, 0)
}
