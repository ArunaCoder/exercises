# ISBN Validator

Source: Exercise 25 from Rust Basic Exercises

### Objective

Validate ISBN-10 and ISBN-13 book identification numbers using their respective checksum algorithms. For ISBN-10, multiply each of the first 9 digits by their position (1-9) and sum them, then check if the sum modulo 11 equals the check digit. For ISBN-13, alternate multiplying digits by 1 and 3. Accept a String and return a bool indicating validity.

### Step-by-Step

1. [ ] Create function `validate_isbn10(isbn: &str) -> bool`
2. [ ] Remove hyphens and spaces from input string
3. [ ] Check length is exactly 10 characters
4. [ ] For each of the first 9 digits, multiply by position (1-9) and sum
5. [ ] The 10th character can be 0-9 or 'X' (representing 10)
6. [ ] Check if sum % 11 equals the check digit value
7. [ ] **Bonus**: Implement `validate_isbn13(isbn: &str) -> bool` with alternating 1/3 multipliers
8. [ ] **Bonus**: Create wrapper function that detects ISBN type by length

### How to test

Run the following command in your terminal:
`cargo test -p isbn_validator`

Or run the program:
`cargo run -p isbn_validator`
