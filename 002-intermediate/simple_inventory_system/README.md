# Simple Inventory System

Source: Exercise 24 from Rust Basic Exercises

### Objective

Create an inventory management system for a store. Define a struct Product with fields for name (String), quantity (u32), and price (f64). Store multiple products in a Vec<Product>. Implement functionality to add new products, update quantities, remove products, and display all items with total inventory value.

### Step-by-Step

1. [ ] Define a struct `Product` with fields: `name: String`, `quantity: u32`, `price: f64`
2. [ ] Create a type alias or struct `Inventory` containing `Vec<Product>`
3. [ ] Implement `add_product(inventory: &mut Vec<Product>, product: Product)`
4. [ ] Implement `update_quantity(inventory: &mut Vec<Product>, name: &str, new_quantity: u32)`
5. [ ] Implement `remove_product(inventory: &mut Vec<Product>, name: &str) -> bool`
6. [ ] Implement `display_inventory(inventory: &[Product])` to print all products
7. [ ] Implement `total_value(inventory: &[Product]) -> f64` to calculate sum of (price × quantity)
8. [ ] **Bonus**: Use methods on a struct instead of standalone functions

### How to test

Run the following command in your terminal:
`cargo test -p simple_inventory_system`

Or run the program:
`cargo run -p simple_inventory_system`
