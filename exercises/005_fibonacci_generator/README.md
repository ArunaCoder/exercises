# Fibonacci Generator

Source: Exercise 05 from Rust Basic Exercises

### Objective

Generate and return the first N numbers of the Fibonacci sequence. Take an integer N as input and produce a Vec<u64> containing the sequence. Start with 0 and 1, then each subsequent number is the sum of the previous two. Use a loop with mutable variables to build the sequence.

### Step-by-Step

1. [ ] Create a function `generate_fibonacci(n: usize) -> Vec<u64>` that takes the count of numbers to generate
2. [ ] Initialize a Vec<u64> to store the sequence
3. [ ] Handle edge cases: if n == 0, return empty vector; if n == 1, return [0]
4. [ ] Start with 0 and 1 as the first two numbers
5. [ ] Use a loop to calculate each subsequent number as the sum of the previous two
6. [ ] Push each new number to the vector until you have N numbers
7. [ ] Return the complete sequence

### How to test

Run the following command in your terminal:
`cargo test -p fibonacci_generator`

Or run the program:
`cargo run -p fibonacci_generator`
