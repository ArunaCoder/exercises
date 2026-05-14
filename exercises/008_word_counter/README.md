# Word Counter

Source: Exercise 08 from Rust Basic Exercises

### Objective

Analyze a text string and count the number of words, lines, and characters. Return these three counts as a tuple (usize, usize, usize). Split the string by whitespace for word counting, by newlines for line counting, and iterate over chars for character counting. Optionally use a HashMap to count word frequency.

### Step-by-Step

1. [ ] Create a function `count_text(text: &str) -> (usize, usize, usize)` that returns (words, lines, characters)
2. [ ] Count characters: use `.chars().count()` to handle UTF-8 properly
3. [ ] Count lines: split by '\n' or use `.lines()` method and count the iterator
4. [ ] Count words: split by whitespace using `.split_whitespace()` and count non-empty segments
5. [ ] Return the three counts as a tuple
6. [ ] **Bonus**: Create a `word_frequency(text: &str) -> HashMap<String, usize>` function to count how many times each word appears

### How to test

Run the following command in your terminal:
`cargo test -p word_counter`

Or run the program:
`cargo run -p word_counter`
