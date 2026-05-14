# Company Department Manager

Source: <https://doc.rust-lang.org/book/ch08-03-hash-maps.html>

### Objective

Create a text interface (or function logic) using a `HashMap` and `Vectors` to add employees to departments and retrieve them sorted alphabetically.

### Step-by-Step

1. [ ] Create a `HashMap<String, Vec<String>>` where the key is the Department name and the value is a list of Employee names.
2. [ ] Implement a function/logic to parse a string like "Add Sally to Engineering".
3. [ ] Push the employee name into the vector corresponding to the department key in the map.
4. [ ] To retrieve names, access the vector of a specific department and use `.sort()` to order them alphabetically.
5. [ ] Implement a way to list all people in the company, grouped by department, with all names sorted.

### How to test

Run the following command in your terminal:
`cargo test -p company_departments`
