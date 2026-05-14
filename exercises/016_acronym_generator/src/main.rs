// ACRONYM GENERATOR
// This program generates acronyms from phrases.
// Take the first letter of each word and capitalize it.
// Examples: "Portable Network Graphics" → "PNG", "HyperText Markup Language" → "HTML"

fn main() {
    let phrases = vec![
        "Portable Network Graphics",
        "HyperText Markup Language",
        "World Wide Web",
        "as soon as possible",
        "Complementary metal-oxide semiconductor",
    ];
    
    for phrase in phrases {
        println!("'{}' → {}", phrase, generate_acronym(phrase));
    }
}

/// Generate an acronym from a phrase
pub fn generate_acronym(phrase: &str) -> String {
    // Your implementation here
    String::new()
}
