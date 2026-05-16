# Median and Mode

Source: <https://doc.rust-lang.org/book/ch08-03-hash-maps.html>

### Objective

Given a list of integers, use a vector and return the median (the value in the middle position when sorted) and the mode (the value that occurs most often).

### Step-by-Step - Median Finder in Rust

- **Declare Mutable Vector**: In `main()`, declare the vector as `let mut numbers = vec![...]` because the median function needs to sort the data in-place, requiring mutable access.

- **Function Signature**: Define the function to accept a `&mut [i32]` (to allow in-place sorting without cloning) and return `Option<f64>` to handle empty input safely.

- **Length Assignment**: Bind `numbers.len()` to an immutable variable `len`.

- **Empty Check (Guard Clause)**: If `len` is 0, return `None` immediately to prevent out-of-bounds panics.

- **In-place Sorting**: Call `.sort_unstable()` on the slice. This is $O(n \log n)$ and avoids the heap allocation overhead of stable sorting.

- **Calculate Midpoint**: Create a `mid` variable using integer division (`len / 2`). Rust's `usize` division automatically truncates decimals.

- **Parity Branching**: Use an `if/else` expression to check if `len % 2 == 0`.

- **Even Logic**: Access elements at `mid - 1` and `mid`. Cast each to `f64` individually before performing the arithmetic to ensure floating-point precision.

- **Odd Logic**: Access the element at index `mid` and cast it to `f64`.

- **Wrap Result**: Enclose the outcome of the parity logic in `Some()` to satisfy the `Option` return type.

### Step-by-Step - Mode Finder in Rust

- **Function Signature**: Define the function to accept a `&[i32]` (immutable slice reference) and return `Option<i32>` to handle empty input safely.

- **Temporary Return Value**: Keep `todo!()` as the last line of the function body temporarily. This silences the compiler error `expected Option<i32>, found ()` and allows your code to compile while you build the implementation step by step. You'll replace this placeholder once the logic is complete.

- **Import HashMap**: Add `use std::collections::HashMap;` at the top of your file to access the hash map data structure.

- **Empty Check (Guard Clause)**: If `numbers.is_empty()` is true, return `None` immediately to prevent errors on empty input.

- **Create Frequency Map**: Initialize a mutable `HashMap<i32, u32>` using `HashMap::new()` to store value → count mappings.

- **Count Occurrences**: Iterate over the slice with `for &num in numbers`. For each number, use `.entry(num).or_insert(0)` to get a mutable reference to the count, then increment it with `+= 1`.

- **Run your code**: At this point, run `cargo run -p median_mode` to verify the HashMap is being populated correctly. You should see output like `{3: 1, 10: 1, 42: 2, 1: 1}` in the terminal, showing each number and its frequency count. The program will panic at `todo!()` after printing this, which is expected.

- **Find Maximum Frequency**: Use `.iter()` on the HashMap to get an iterator of `(&i32, &u32)` tuples (key-value pairs).

- **Max_by_key Operation**: Chain `.max_by_key(|&(_, count)| count)` to find the entry with the highest count value. This returns `Option<(&i32, &u32)>`.

- **Extract the Mode**: Use `.map(|(&num, _)| num)` to transform the Option by extracting just the key (the number), discarding the count.

- **Wrap Result**: The final expression already returns `Option<i32>` from the map operation, matching the function signature.

### How to test

Run the following command in your terminal:
`cargo test -p median_mode`
