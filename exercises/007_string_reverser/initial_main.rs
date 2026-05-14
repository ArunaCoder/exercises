// STRING REVERSER
// This program reverses a string while correctly handling UTF-8 multi-byte characters.
// UTF-8 characters can be 1-4 bytes, so we must use .chars() instead of .bytes()
// Examples: "hello" → "olleh", "café" → "éfac", "Hello 😀" → "😀 olleH"

fn main() {
    let text1 = "hello world";
    let text2 = "café";
    let text3 = "Hello 😀🎉";

    println!(
        "Original: '{}' → Reversed: '{}'",
        text1,
        reverse_string(text1)
    );
    println!(
        "Original: '{}' → Reversed: '{}'",
        text2,
        reverse_string(text2)
    );
    println!(
        "Original: '{}' → Reversed: '{}'",
        text3,
        reverse_string(text3)
    );
}

/// Reverse a string while preserving UTF-8 characters
pub fn reverse_string(s: &str) -> String {
    // Your implementation here
    String::new()
}
