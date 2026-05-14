# Rock Paper Scissors

Source: Exercise 22 from Rust Basic Exercises

### Objective

Create a command-line game of Rock, Paper, Scissors against the computer. Define an enum with variants Rock, Paper, and Scissors. Generate a random computer choice, accept user input, and determine the winner using pattern matching. Keep playing until the user chooses to quit.

### Step-by-Step

1. [ ] Define an enum `Choice` with variants: `Rock`, `Paper`, `Scissors`
2. [ ] Implement a function `random_choice() -> Choice` using `rand::thread_rng().gen_range(0..3)`
3. [ ] Implement `determine_winner(player: &Choice, computer: &Choice) -> GameResult`
4. [ ] Define an enum `GameResult` with variants: `PlayerWins`, `ComputerWins`, `Tie`
5. [ ] Use pattern matching to compare choices (Rock beats Scissors, Scissors beats Paper, Paper beats Rock)
6. [ ] In `main()`, create a loop that accepts user input ("rock", "paper", "scissors", or "quit")
7. [ ] Display the computer's choice and the game result
8. [ ] Keep track of wins, losses, and ties

### How to test

Run the following command in your terminal:
`cargo test -p rock_paper_scissors`

Or run the program:
`cargo run -p rock_paper_scissors`
