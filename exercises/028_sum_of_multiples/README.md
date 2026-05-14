# Sum of Multiples

Source: Exercise 28 from Rust Basic Exercises

### Objective

Calculate the sum of all positive integers below a given number N that are multiples of 3 or 5. For example, if N is 10, the multiples are 3, 5, 6, and 9, which sum to 23. Use a loop to iterate from 1 to N-1, check divisibility with modulo, and accumulate the sum in a mutable variable.

### Step-by-Step

1. [ ] Create function `sum_of_multiples(n: u32) -> u32`
2. [ ] Initialize a mutable variable `sum = 0`
3. [ ] Loop from 1 to n-1 (exclusive of n)
4. [ ] Check if number is divisible by 3: `i % 3 == 0`
5. [ ] Check if number is divisible by 5: `i % 5 == 0`
6. [ ] If either condition is true, add to sum
7. [ ] Return the accumulated sum
8. [ ] **Bonus**: Generalize to accept Vec<u32> of divisors instead of hardcoded 3 and 5

### How to test

Run the following command in your terminal:
`cargo test -p sum_of_multiples`

Or run the program:
`cargo run -p sum_of_multiples`
