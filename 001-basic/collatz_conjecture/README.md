# Collatz Conjecture

Source: Exercise 30 from Rust Basic Exercises

### Objective

Generate the Collatz sequence (also called the 3n+1 problem) for a starting number. If the number is even, divide it by 2; if odd, multiply by 3 and add 1. Continue until reaching 1. Return a Vec<u64> containing the complete sequence. Use a loop with conditional logic to apply the rules.

### Step-by-Step

1. [ ] Create function `collatz_sequence(n: u64) -> Vec<u64>`
2. [ ] Create an empty Vec to store the sequence
3. [ ] Add the starting number to the sequence
4. [ ] Loop while the current number is not 1
5. [ ] If even (n % 2 == 0): divide by 2
6. [ ] If odd: multiply by 3 and add 1
7. [ ] Add the new number to the sequence
8. [ ] Return the complete sequence
9. [ ] **Bonus**: Find the number below N that produces the longest sequence

### How to test

Run the following command in your terminal:
`cargo test -p collatz_conjecture`

Or run the program:
`cargo run -p collatz_conjecture`
