// VOWEL COUNTER
// This program counts vowels and consonants in a text string.
// Vowels are: a, e, i, o, u (case-insensitive)
// Only count alphabetic characters, ignore numbers and punctuation.

fn main() {
    let texts = vec![
        "Hello World",
        "Rust Programming 12",
        "aeiou AEIOU",
        "The quick brown fox jumps over the lazy dog",
        "Meditação é ausência do eu",
    ];

    for text in texts {
        let (vowels, consonants) = count_vowels_and_consonants(text);
        println!(
            "'{}' → Vowels: {}, Consonants: {}",
            text, vowels, consonants
        );
    }
}

/// Count vowels and consonants in a string (Unicode-aware for Portuguese)
/// Returns: (vowel_count, consonant_count)
pub fn count_vowels_and_consonants(text: &str) -> (usize, usize) {
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for c in text.chars() {
        if c.is_alphabetic() {
            // to_lowercase() retorna um iterador de chars (um char pode virar múltiplos!)
            // Para a maioria dos casos (incluindo português), é um único char
            let lower = c.to_lowercase().next().unwrap_or(c);

            match lower {
                'a' | 'e' | 'i' | 'o' | 'u' | // ASCII vogais
                'á' | 'à' | 'â' | 'ã' | // A com acentos
                'é' | 'ê' |               // E com acentos
                'í' |                     // I com acentos
                'ó' | 'ô' | 'õ' |         // O com acentos
                'ú' | 'ü'                 // U com acentos
                => vowel_count += 1,
                _ => consonant_count += 1,
            }
        }
    }

    (vowel_count, consonant_count)
}
