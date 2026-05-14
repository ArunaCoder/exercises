# Duplicate Remover

Source: Exercise 19 from Rust Basic Exercises

### Objective

Remove duplicate elements from a vector while preserving the original order of first occurrences. Take a Vec<i32> as input and return a new Vec<i32> without duplicates. Use a HashSet to track seen elements while iterating through the original vector.

### Step-by-Step

1. [ ] Create a function `remove_duplicates(numbers: &[i32]) -> Vec<i32>`
2. [ ] Create a HashSet<i32> to track which numbers have been seen
3. [ ] Create an empty Vec<i32> for the result
4. [ ] Iterate through the input slice
5. [ ] For each number, check if it's already in the HashSet
6. [ ] If not seen before, add it to both the HashSet and result Vec
7. [ ] If already seen, skip it (don't add to result)
8. [ ] Return the result vector with duplicates removed, order preserved

### How to test

Run the following command in your terminal:
`cargo test -p duplicate_remover`

Or run the program:
`cargo run -p duplicate_remover`
