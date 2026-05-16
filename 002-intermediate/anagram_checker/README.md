# Anagram Checker

Source: Exercise 17 from Rust Basic Exercises

### Objective

Determine if two words are anagrams (contain the same letters in different order). Normalize both strings to lowercase, convert to char vectors, sort both vectors, and compare for equality. Return a boolean indicating whether they are anagrams.

### Step-by-Step

1. [ ] Create a function `are_anagrams(word1: &str, word2: &str) -> bool`
2. [ ] Convert both words to lowercase for case-insensitive comparison
3. [ ] Remove spaces and non-alphabetic characters (for phrases like "Tom Marvolo Riddle")
4. [ ] Convert each cleaned string to a Vec<char> using `.chars().collect()`
5. [ ] Sort both character vectors using `.sort()`
6. [ ] Compare the sorted vectors for equality
7. [ ] Return true if they match, false otherwise

### How to test

Run the following command in your terminal:
`cargo test -p anagram_checker`

Or run the program:
`cargo run -p anagram_checker`
