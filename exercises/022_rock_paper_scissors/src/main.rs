// ROCK PAPER SCISSORS
// Classic game against the computer
// Rock beats Scissors, Scissors beats Paper, Paper beats Rock
// Play until user types "quit"

use rand::Rng;

fn main() {
    println!("=== Rock, Paper, Scissors ===");
    println!("Type 'rock', 'paper', 'scissors', or 'quit'\n");

    let mut wins = 0;
    let mut losses = 0;
    let mut ties = 0;

    // Game loop would go here
    // Example:
    let player = Choice::Rock;
    let computer = random_choice();

    println!("You chose: {:?}", player);
    println!("Computer chose: {:?}", computer);

    match determine_winner(&player, &computer) {
        GameResult::PlayerWins => {
            wins += 1;
            println!("You win! 🎉");
        }
        GameResult::ComputerWins => {
            losses += 1;
            println!("Computer wins! 💻");
        }
        GameResult::Tie => {
            ties += 1;
            println!("It's a tie! 🤝");
        }
    }

    println!("\nScore: {} wins, {} losses, {} ties", wins, losses, ties);
}

#[derive(Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub enum GameResult {
    PlayerWins,
    ComputerWins,
    Tie,
}

/// Generate a random computer choice
pub fn random_choice() -> Choice {
    todo!("Generate random choice using rand::thread_rng().gen_range(0..3)")
}

/// Determine the winner of the game
pub fn determine_winner(player: &Choice, computer: &Choice) -> GameResult {
    todo!("Use pattern matching to determine winner based on game rules")
}
