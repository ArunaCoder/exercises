# Largest Product in Series

Source: Exercise 29 from Rust Basic Exercises

### Objective

Given a string of digits, find the largest product of N consecutive digits. For example, in the string "1234", the largest product of 2 consecutive digits is 3×4 = 12. Use the windows() method on a Vec<u32> of digits to examine consecutive subslices, calculate their product, and track the maximum.

### Step-by-Step

1. [ ] Create function `largest_product(digits: &str, span: usize) -> Result<u64, String>`
2. [ ] Convert string to Vec<u32> of individual digits
3. [ ] Validate that span is not larger than the number of digits
4. [ ] Use `.windows(span)` to iterate over consecutive subslices
5. [ ] For each window, calculate the product of all digits
6. [ ] Track the maximum product seen
7. [ ] Return the maximum product
8. [ ] Handle edge cases: empty string, span of 0, invalid characters

### How to test

Run the following command in your terminal:
`cargo test -p largest_product_in_series`

Or run the program:
`cargo run -p largest_product_in_series`
