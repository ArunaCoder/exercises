// ANAGRAM CHECKER
// This program checks if two words/phrases are anagrams.
// Anagrams contain the same letters in different order.
// Examples: "listen" & "silent", "The eyes" & "They see"

fn main() {
    let test_pairs = vec![
        ("listen", "silent"),
        ("hello", "world"),
        ("The eyes", "They see"),
        ("Dormitory", "Dirty room"),
        ("rust", "trust"),
    ];
    
    for (word1, word2) in test_pairs {
        let result = are_anagrams(word1, word2);
        println!("'{}' & '{}' → {}", word1, word2, if result { "Anagrams!" } else { "Not anagrams" });
    }
}

/// Check if two strings are anagrams
pub fn are_anagrams(word1: &str, word2: &str) -> bool {
    // Your implementation here
    false
}
