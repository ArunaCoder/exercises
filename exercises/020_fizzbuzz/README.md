# FizzBuzz

Source: Exercise 20 from Rust Basic Exercises

### Objective

Implement the classic FizzBuzz problem: print numbers from 1 to 100, but for multiples of 3 print "Fizz", for multiples of 5 print "Buzz", and for multiples of both print "FizzBuzz". Use a loop with conditionals and modulo operations to check divisibility.

### Step-by-Step

1. [ ] Create a function `fizzbuzz(n: u32)` that prints FizzBuzz from 1 to n
2. [ ] Loop from 1 to n (inclusive)
3. [ ] For each number, check divisibility:
   - If divisible by both 3 and 5 (i.e., divisible by 15): print "FizzBuzz"
   - Else if divisible by 3: print "Fizz"
   - Else if divisible by 5: print "Buzz"
   - Else: print the number itself
4. [ ] Use modulo operator (%) to check divisibility: `n % 3 == 0`
5. [ ] Order matters: check for 15 first, then 3, then 5
6. [ ] **Bonus**: Return a Vec<String> instead of printing directly

### How to test

Run the following command in your terminal:
`cargo test -p fizzbuzz`

Or run the program:
`cargo run -p fizzbuzz`
