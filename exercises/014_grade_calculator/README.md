# Grade Calculator

Source: Exercise 14 from Rust Basic Exercises

### Objective

Calculate a student's final grade based on multiple assignments with different weights. Create a struct to hold assignment names, scores, and weights. Store multiple assignments in a Vec and compute the weighted average. Return the final grade as a percentage or letter grade.

### Step-by-Step

1. [ ] Define a struct `Assignment` with fields: name (String), score (f64), weight (f64)
2. [ ] Create a Vec<Assignment> to store multiple assignments
3. [ ] Implement a function `calculate_final_grade(assignments: &[Assignment]) -> f64`
4. [ ] For each assignment, multiply score by weight: score * weight
5. [ ] Sum all weighted scores to get the final grade
6. [ ] Validate that weights sum to 1.0 (100%) - return error if not
7. [ ] **Bonus**: Create a function `grade_to_letter(grade: f64) -> char` that converts percentage to letter (A, B, C, D, F)

### How to test

Run the following command in your terminal:
`cargo test -p grade_calculator`

Or run the program:
`cargo run -p grade_calculator`
