// PALINDROME CHECKER
// This program checks if a word or phrase is a palindrome.
// A palindrome reads the same forwards and backwards, ignoring spaces, punctuation, and case.
// Examples: "racecar" → true, "A man a plan a canal Panama" → true, "hello" → false

fn main() {
    let test_cases = vec![
        "racecar",
        "A man a plan a canal Panama",
        "hello",
        "Was it a car or a cat I saw?",
        "Madam",
    ];

    for text in test_cases {
        println!("'{}' is palindrome: {}", text, is_palindrome(text));
    }
}

/// Check if a string is a palindrome
/// Ignores case, spaces, and non-alphanumeric characters
pub fn is_palindrome(s: &str) -> bool {
    // Your implementation here
    false
}
