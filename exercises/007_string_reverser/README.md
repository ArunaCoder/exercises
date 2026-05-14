# String Reverser

Source: Exercise 07 from Rust Basic Exercises

### Objective

Reverse a String while correctly handling UTF-8 multi-byte characters (like emojis and accented letters). Take a String or &str as input and return a new String with characters in reverse order. Use the chars() method to iterate over Unicode scalar values rather than bytes.

### Step-by-Step

1. [ ] Create a function `reverse_string(s: &str) -> String` that reverses a string
2. [ ] Use the `.chars()` method to iterate over Unicode characters (not bytes)
3. [ ] Collect the characters into a Vec or use `.rev()` to reverse the iterator
4. [ ] Convert the reversed characters back into a String
5. [ ] Test with regular ASCII text, accented characters (é, ñ, ü), and emojis (😀, 🎉)
6. [ ] Ensure multi-byte UTF-8 characters are handled correctly

### How to test

Run the following command in your terminal:
`cargo test -p string_reverser`

Or run the program:
`cargo run -p string_reverser`
