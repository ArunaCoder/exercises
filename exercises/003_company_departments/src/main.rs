// COMPANY DEPARTMENT MANAGER
// This program manages employee records using a HashMap and Vectors to:
// 1. Add employees to specific departments (e.g., "Add Sally to Engineering").
// 2. Retrieve alphabetically sorted lists of employees by department or for the whole company.

use std::collections::HashMap;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    // Add employees to departments
    add_employee(&mut company, "Sally", "Engineering");
    add_employee(&mut company, "Bob", "Sales");
    add_employee(&mut company, "Alice", "Engineering");
    add_employee(&mut company, "Charlie", "Sales");
    add_employee(&mut company, "Diana", "Marketing");

    println!("=== Company Employees by Department ===\n");

    // List employees by department (sorted alphabetically)
    for department in ["Engineering", "Sales", "Marketing"] {
        println!("{}:", department);
        let employees = list_department(&company, department);
        for emp in employees {
            println!("  - {}", emp);
        }
        println!();
    }

    // List all employees in the company
    println!("=== All Employees (sorted) ===\n");
    let all_employees = list_all_employees(&company);
    for emp in all_employees {
        println!("  - {}", emp);
    }
}

/// Add an employee to a department
pub fn add_employee(company: &mut HashMap<String, Vec<String>>, name: &str, department: &str) {
    // Your implementation here
}

/// List all employees in a department (sorted alphabetically)
pub fn list_department(company: &HashMap<String, Vec<String>>, department: &str) -> Vec<String> {
    // Your implementation here
    vec![]
}

/// List all employees in the company (sorted alphabetically)
pub fn list_all_employees(company: &HashMap<String, Vec<String>>) -> Vec<String> {
    // Your implementation here
    vec![]
}
