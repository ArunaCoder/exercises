// GRADE CALCULATOR
// This program calculates a student's final grade based on weighted assignments.
// Each assignment has a score (0-100) and a weight (portion of final grade).
// Example: Midterm 85 (weight 0.3) + Final 90 (weight 0.7) = 88.5

fn main() {
    let assignments = vec![
        Assignment::new("Homework", 95.0, 0.2),
        Assignment::new("Midterm", 85.0, 0.3),
        Assignment::new("Final Exam", 90.0, 0.5),
    ];
    
    match calculate_final_grade(&assignments) {
        Ok(grade) => {
            println!("Final Grade: {:.2}%", grade);
            println!("Letter Grade: {}", grade_to_letter(grade));
        }
        Err(e) => println!("Error: {}", e),
    }
}

/// Struct representing an assignment with score and weight
pub struct Assignment {
    name: String,
    score: f64,   // 0-100
    weight: f64,  // 0.0-1.0 (portion of final grade)
}

impl Assignment {
    pub fn new(name: &str, score: f64, weight: f64) -> Self {
        Assignment {
            name: name.to_string(),
            score,
            weight,
        }
    }
}

/// Calculate weighted final grade
pub fn calculate_final_grade(assignments: &[Assignment]) -> Result<f64, String> {
    // Your implementation here
    Ok(0.0)
}

/// Convert percentage grade to letter grade
pub fn grade_to_letter(grade: f64) -> char {
    // Your implementation here: A(90+), B(80+), C(70+), D(60+), F(<60)
    'F'
}
