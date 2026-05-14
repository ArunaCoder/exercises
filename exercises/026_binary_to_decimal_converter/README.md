# Binary to Decimal Converter

Source: Exercise 26 from Rust Basic Exercises

### Objective

Convert binary number strings to decimal integers and vice versa. Accept a String containing only '0' and '1' characters, iterate through it, and calculate the decimal value using powers of 2. For the reverse operation, repeatedly divide by 2 and collect remainders. Return the converted value as a String or u32.

### Step-by-Step

1. [ ] Create `binary_to_decimal(binary: &str) -> Result<u32, String>`
2. [ ] Validate input contains only '0' and '1' characters
3. [ ] Iterate through characters from right to left (or reverse the string)
4. [ ] For each digit, multiply by 2^position and add to sum
5. [ ] Create `decimal_to_binary(decimal: u32) -> String`
6. [ ] Handle special case: 0 → "0"
7. [ ] Repeatedly divide by 2, collect remainders, reverse the result
8. [ ] **Bonus**: Support conversion between any bases (2-36)

### How to test

Run the following command in your terminal:
`cargo test -p binary_to_decimal_converter`

Or run the program:
`cargo run -p binary_to_decimal_converter`
