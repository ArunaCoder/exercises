# Prime Number Checker

Source: Exercise 06 from Rust Basic Exercises

### Objective

Create a function that determines whether a given number is prime. Additionally, implement a function that returns a Vec<u32> containing all prime numbers up to a given limit N. Use nested loops to check divisibility and boolean logic to validate primality.

### Step-by-Step

1. [ ] Create a function `is_prime(n: u32) -> bool` that checks if a number is prime
2. [ ] A prime number is only divisible by 1 and itself (numbers greater than 1)
3. [ ] Handle edge cases: 0 and 1 are not prime, 2 is the only even prime
4. [ ] For other numbers, check if divisible by any number from 2 to sqrt(n)
5. [ ] Create a function `find_primes_up_to(limit: u32) -> Vec<u32>` that finds all primes up to a limit
6. [ ] Loop from 2 to the limit and use `is_prime()` to check each number
7. [ ] Collect all prime numbers into a vector and return it

### How to test

Run the following command in your terminal:
`cargo test -p prime_number_checker`

Or run the program:
`cargo run -p prime_number_checker`
