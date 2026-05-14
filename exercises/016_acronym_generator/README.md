# Acronym Generator

Source: Exercise 16 from Rust Basic Exercises

### Objective

Generate acronyms from phrases by taking the first letter of each word and capitalizing it. For example, "Portable Network Graphics" becomes "PNG". Split the input string by whitespace, extract the first character of each word, convert to uppercase, and collect into a new String.

### Step-by-Step

1. [ ] Create a function `generate_acronym(phrase: &str) -> String` that returns an acronym
2. [ ] Split the phrase by whitespace using `.split_whitespace()`
3. [ ] For each word, get the first character using `.chars().next()`
4. [ ] Convert the character to uppercase using `.to_uppercase()`
5. [ ] Collect all first letters into a String
6. [ ] Handle edge cases: empty string, single word, words with hyphens
7. [ ] **Bonus**: Handle hyphenated words like "Self-Contained" → "SC"

### How to test

Run the following command in your terminal:
`cargo test -p acronym_generator`

Or run the program:
`cargo run -p acronym_generator`
