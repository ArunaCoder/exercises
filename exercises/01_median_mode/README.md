# Median and Mode

Source: <https://doc.rust-lang.org/book/ch08-03-hash-maps.html>

### Objective

Given a list of integers, use a vector and return the median (the value in the middle position when sorted) and the mode (the value that occurs most often).

### Step-by-Step

1. [ ] Import `std::collections::HashMap` at the top of your file.
2. [ ] Sort the vector of integers using the `.sort()` method.
3. [ ] Calculate the median: find the middle index. If the list has an even number of elements, you may return the lower middle value.
4. [ ] Calculate the mode: iterate over the vector and use a `HashMap` to count how many times each integer appears.
5. [ ] Find the key in the `HashMap` with the highest value (count) and return both results as a tuple.

### How to test

Run the following command in your terminal:
`cargo test -p median_mode`
