// SHOPPING LIST MANAGER
// This program manages a shopping list with add, remove, and list operations.
// Use a Vec<String> to store items and a loop with a menu system.

use std::io;

fn main() {
    let mut shopping_list: Vec<String> = Vec::new();
    
    loop {
        println!("\n=== Shopping List Manager ===");
        println!("1. Add item");
        println!("2. Remove item");
        println!("3. List all items");
        println!("4. Quit");
        println!("Choose an option: ");
        
        // Your implementation here: read choice, match on it, call appropriate function
    }
}

fn add_item(list: &mut Vec<String>) {
    // Read item name and push to list
}

fn remove_item(list: &mut Vec<String>) {
    // Display list, read index, remove item
}

fn list_items(list: &Vec<String>) {
    // Display all items with numbers
}
