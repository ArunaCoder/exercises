# Guessing Game Enhanced

Source: Exercise 12 from Rust Basic Exercises

### Objective

Create an interactive number guessing game where the program randomly selects a number between 1 and 100, and the player tries to guess it. Provide feedback ("too high" or "too low") after each guess and count the number of attempts. Use loops for repeated guessing and comparison operators for feedback.

### Step-by-Step

1. [ ] Add the `rand` crate to dependencies in Cargo.toml (already done)
2. [ ] Use `rand::thread_rng().gen_range(1..=100)` to generate a random number
3. [ ] Create a loop that continues until the correct guess
4. [ ] Read user input using `std::io::stdin()`
5. [ ] Parse the input to a u32 and handle parse errors gracefully
6. [ ] Compare the guess with the secret number using `match` or `if/else`
7. [ ] Print "Too high!", "Too low!", or "Correct!" based on comparison
8. [ ] Count attempts and display the count when the player wins

### How to test

Run the program interactively:
`cargo run -p guessing_game_enhanced`
