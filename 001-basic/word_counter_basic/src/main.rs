// WORD COUNTER
// This program analyzes text and counts words, lines, and characters.
// Returns a tuple: (word_count, line_count, character_count)
// Example: "Hello world\nRust is great!" → (5 words, 2 lines, 29 characters)

fn main() {
    let text = "Hello world!\nWelcome to Rust programming.\nRust is amazing!";

    let (words, lines, chars) = count_text(text);
    println!("Text analysis:");
    println!("  Words: {}", words);
    println!("  Lines: {}", lines);
    println!("  Characters: {}", chars);
}

/// Count words, lines, and characters in a text
/// Returns: (word_count, line_count, character_count)
pub fn count_text(text: &str) -> (usize, usize, usize) {
    // Your implementation here
    (0, 0, 0)
}
