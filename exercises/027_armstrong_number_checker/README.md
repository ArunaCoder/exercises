# Armstrong Number Checker

Source: Exercise 27 from Rust Basic Exercises

### Objective

Determine if a number is an Armstrong number (also called narcissistic number), where the sum of its digits raised to the power of the number of digits equals the number itself. For example, 153 is an Armstrong number because 1³ + 5³ + 3³ = 153. Extract individual digits from a u32 and perform exponentiation.

### Step-by-Step

1. [ ] Create function `is_armstrong_number(n: u32) -> bool`
2. [ ] Convert number to string to count digits: `n.to_string().len()`
3. [ ] Extract each digit by repeatedly dividing by 10 and taking modulo 10
4. [ ] Raise each digit to the power of total digit count using `.pow()`
5. [ ] Sum all powered digits
6. [ ] Compare sum with original number
7. [ ] **Bonus**: Return a Vec<u32> of all Armstrong numbers up to N

### How to test

Run the following command in your terminal:
`cargo test -p armstrong_number_checker`

Or run the program:
`cargo run -p armstrong_number_checker`
