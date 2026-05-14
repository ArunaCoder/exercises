// SIMPLE INVENTORY SYSTEM
// Manage products in a store inventory
// Track name, quantity, and price for each product
// Support add, update, remove, and display operations

fn main() {
    let mut inventory: Vec<Product> = Vec::new();

    // Add products
    add_product(&mut inventory, Product::new("Laptop", 10, 999.99));
    add_product(&mut inventory, Product::new("Mouse", 50, 25.50));
    add_product(&mut inventory, Product::new("Keyboard", 30, 75.00));

    println!("=== Initial Inventory ===");
    display_inventory(&inventory);

    // Update quantity
    update_quantity(&mut inventory, "Mouse", 45);
    println!("\n=== After updating Mouse quantity ===");
    display_inventory(&inventory);

    // Remove product
    remove_product(&mut inventory, "Keyboard");
    println!("\n=== After removing Keyboard ===");
    display_inventory(&inventory);

    // Calculate total value
    let total = total_value(&inventory);
    println!("\nTotal Inventory Value: ${:.2}", total);
}

pub struct Product {
    name: String,
    quantity: u32,
    price: f64,
}

impl Product {
    pub fn new(name: &str, quantity: u32, price: f64) -> Self {
        Product {
            name: name.to_string(),
            quantity,
            price,
        }
    }
}

/// Add a product to the inventory
pub fn add_product(inventory: &mut Vec<Product>, product: Product) {
    todo!("Add product to the inventory vector")
}

/// Update the quantity of a product by name
pub fn update_quantity(inventory: &mut Vec<Product>, name: &str, new_quantity: u32) {
    todo!("Find product by name and update its quantity")
}

/// Remove a product from inventory by name
pub fn remove_product(inventory: &mut Vec<Product>, name: &str) -> bool {
    todo!("Find and remove product, return true if found")
}

/// Display all products in the inventory
pub fn display_inventory(inventory: &[Product]) {
    todo!("Print each product with name, quantity, and price")
}

/// Calculate total value of all products
pub fn total_value(inventory: &[Product]) -> f64 {
    todo!("Sum up price × quantity for all products")
}
