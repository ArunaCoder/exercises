# Palindrome Checker

Source: Exercise 09 from Rust Basic Exercises

### Objective

Determine whether a given word or phrase is a palindrome (reads the same forwards and backwards). Normalize the input by converting to lowercase and removing non-alphanumeric characters. Compare the cleaned string with its reverse to check for palindrome property.

### Step-by-Step

1. [ ] Create a function `is_palindrome(s: &str) -> bool` that checks if a string is a palindrome
2. [ ] Convert the input to lowercase using `.to_lowercase()`
3. [ ] Filter out non-alphanumeric characters (keep only letters and numbers)
4. [ ] Collect the filtered characters into a String or Vec
5. [ ] Compare the cleaned string with its reverse
6. [ ] Return true if they match, false otherwise
7. [ ] Test with: "racecar" (true), "A man a plan a canal Panama" (true), "hello" (false)

### How to test

Run the following command in your terminal:
`cargo test -p palindrome_checker`

Or run the program:
`cargo run -p palindrome_checker`
