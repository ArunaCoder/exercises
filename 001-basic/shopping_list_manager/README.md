# Shopping List Manager

Source: Exercise 13 from Rust Basic Exercises

### Objective

Implement a shopping list application where users can add items, remove items, and display all items. Store items in a Vec<String>. Present a text menu with options (1: Add, 2: Remove, 3: List, 4: Quit) and loop until the user chooses to exit.

### Step-by-Step

1. [ ] Create a Vec<String> to store shopping list items
2. [ ] Create a main loop that displays a menu with 4 options
3. [ ] Read user input to determine which action to perform
4. [ ] **Add**: Read an item name and push it to the vector
5. [ ] **Remove**: Show numbered list, read index, remove item at that position
6. [ ] **List**: Iterate through vector and display all items with numbers
7. [ ] **Quit**: Break the loop and end the program
8. [ ] Handle invalid input and empty list cases gracefully

### How to test

Run the program interactively:
`cargo run -p shopping_list_manager`
