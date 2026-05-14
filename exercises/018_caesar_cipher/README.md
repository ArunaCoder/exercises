# Caesar Cipher

Source: Exercise 18 from Rust Basic Exercises

### Objective

Implement the Caesar cipher encryption technique where each letter is shifted by N positions in the alphabet. Accept a String and a shift amount (i32), then return the encrypted String. Handle wrapping (e.g., 'z' shifted by 1 becomes 'a') using modulo arithmetic. Preserve non-alphabetic characters.

### Step-by-Step

1. [ ] Create a function `caesar_cipher(text: &str, shift: i32) -> String`
2. [ ] Iterate through each character in the input text using `.chars()`
3. [ ] For each character, check if it's alphabetic
4. [ ] If alphabetic, determine if it's uppercase or lowercase
5. [ ] Convert character to a number (a=0, b=1, ..., z=25)
6. [ ] Add the shift value and use modulo 26 to wrap around
7. [ ] Convert back to a character and preserve the original case
8. [ ] Non-alphabetic characters (spaces, numbers, punctuation) remain unchanged

### How to test

Run the following command in your terminal:
`cargo test -p caesar_cipher`

Or run the program:
`cargo run -p caesar_cipher`
