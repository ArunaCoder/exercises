// GUESSING GAME ENHANCED
// The program picks a random number between 1-100 and you try to guess it.
// After each guess, you get feedback: "Too high!" or "Too low!"
// The game counts your attempts and congratulates you when you win.

use rand::Rng;
use std::io;

fn main() {
    println!("=== Guessing Game ===");
    println!("I'm thinking of a number between 1 and 100...");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    
    // Your implementation here: loop for guesses, read input, compare, give feedback
    
    println!("The secret number was: {}", secret_number);
}
