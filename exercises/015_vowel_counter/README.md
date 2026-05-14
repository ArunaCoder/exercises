# Vowel Counter

Source: Exercise 15 from Rust Basic Exercises

### Objective

Count the number of vowels and consonants in a given text string. Iterate through each character using chars(), check if it's a vowel (a, e, i, o, u), and maintain separate counters. Return both counts as a tuple (usize, usize).

### Step-by-Step

1. [ ] Create a function `count_vowels_and_consonants(text: &str) -> (usize, usize)` returning (vowels, consonants)
2. [ ] Initialize two counters: vowel_count and consonant_count
3. [ ] Convert text to lowercase for case-insensitive counting
4. [ ] Iterate through each character using `.chars()`
5. [ ] Check if character is alphabetic using `.is_alphabetic()`
6. [ ] If alphabetic, check if it's a vowel (a, e, i, o, u)
7. [ ] Increment appropriate counter (vowel or consonant)
8. [ ] Return tuple with both counts

### How to test

Run the following command in your terminal:
`cargo test -p vowel_counter`

Or run the program:
`cargo run -p vowel_counter`
