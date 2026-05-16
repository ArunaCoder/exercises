// CAESAR CIPHER
// This program encrypts/decrypts text using the Caesar cipher.
// Each letter is shifted by N positions in the alphabet.
// Example: "Hello" with shift 3 → "Khoor"

fn main() {
    let messages = vec![
        ("Hello, World!", 3),
        ("Rust Programming", 5),
        ("The quick brown fox", 13),
        ("ATTACK AT DAWN", 7),
    ];
    
    for (text, shift) in messages {
        let encrypted = caesar_cipher(text, shift);
        let decrypted = caesar_cipher(&encrypted, -shift);
        println!("Original: {}", text);
        println!("Encrypted (shift {}): {}", shift, encrypted);
        println!("Decrypted: {}\n", decrypted);
    }
}

/// Encrypt/decrypt text using Caesar cipher
/// Positive shift for encryption, negative for decryption
pub fn caesar_cipher(text: &str, shift: i32) -> String {
    // Your implementation here
    String::new()
}
