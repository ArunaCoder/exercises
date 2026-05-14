# Leap Year Checker

Source: Exercise 21 from Rust Basic Exercises

### Objective

Determine whether a given year is a leap year according to the Gregorian calendar rules. A year is a leap year if it's divisible by 4, except for years divisible by 100 unless they are also divisible by 400. Take a u32 representing the year and return a bool. Use nested conditionals or boolean logic to implement the rules.

### Step-by-Step

1. [ ] Create a function `is_leap_year(year: u32) -> bool`
2. [ ] Check if the year is divisible by 400 → if yes, it's a leap year
3. [ ] Check if the year is divisible by 100 → if yes, it's NOT a leap year
4. [ ] Check if the year is divisible by 4 → if yes, it's a leap year
5. [ ] Otherwise, it's NOT a leap year
6. [ ] Use modulo operator (%) to check divisibility: `year % 4 == 0`
7. [ ] **Bonus**: Use a single boolean expression instead of nested conditionals

### How to test

Run the following command in your terminal:
`cargo test -p leap_year_checker`

Or run the program:
`cargo run -p leap_year_checker`
