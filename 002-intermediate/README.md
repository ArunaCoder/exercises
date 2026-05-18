# Track 002: Intermediate Structures

**Learning Focus**: Structs, methods, algorithms, collection patterns with explicit loops

## Philosophy

You've mastered the basics. Now you'll:

- Design your own data types with `struct` and `enum`
- Implement methods and associated functions
- Work with complex collection patterns
- Understand algorithmic thinking
- Handle errors explicitly

**Still no iterator magic.** You're building the mental model for what `.fold()` and `.map()` do internally.

## What You'll Learn

- Structs: defining, instantiating, field access
- Methods: `impl` blocks, `self`, `&self`, `&mut self`
- Enums: variants with data, pattern matching
- Traits: `Display`, `Debug`, `PartialEq` (basic usage)
- HashMap patterns: entry API basics, iteration
- Vec patterns: capacity, shrinking, searching
- Algorithms: validation, transformation, statistics
- Error handling: `Option`, `Result`, basic patterns
- File I/O basics (some exercises)

## What You Won't Use (Yet)

❌ `.map()`, `.filter()`, `.fold()` - still avoiding
❌ Complex iterator chains
❌ Closure-based collection operations
❌ Generic implementations (only usage)
❌ Custom traits and derive macros

## Exercise List

**Currently Available: 10 exercises**

Run with: `cargo run -p <folder_name>`

| #   | Exercise                | Folder                                              | Key Concepts                          |
| --- | ----------------------- | --------------------------------------------------- | ------------------------------------- |
| 1   | Grade Calculator        | [grade_calculator](grade_calculator/)               | Struct with methods, weighted average |
| 2   | Simple Inventory System | [simple_inventory_system](simple_inventory_system/) | CRUD operations, structs              |
| 3   | Unit Converter          | [unit_converter](unit_converter/)                   | Enum pattern matching, conversions    |
| 4   | Median & Mode (Loops)   | [median_loops](median_loops/)                       | Vec sorting, HashMap, manual loops    |
| 5   | Pig Latin               | [pig_latin](pig_latin/)                             | String manipulation, UTF-8            |
| 6   | Company Departments     | [company_departments](company_departments/)         | HashMap<String, Vec>, sorting         |
| 7   | Array Statistics        | [array_statistics](array_statistics/)               | Multiple statistical calculations     |
| 8   | Duplicate Remover       | [duplicate_remover](duplicate_remover/)             | HashSet + Vec, order preservation     |
| 9   | Anagram Checker         | [anagram_checker](anagram_checker/)                 | Sort + compare, manual approach       |
| 10  | Largest Series Product  | [largest_series_product](largest_series_product/)   | Window-like logic with loops          |

### Compare with Track 003 Later

These exercises will have **functional versions** in Track 003:

- **Median & Mode** → See `.max_by_key()` replace your for loop
- **Company Departments** → Learn the entry API + `.collect()`
- **Array Statistics** → Discover `.fold()` and `.map()`
- **Duplicate Remover** → Use functional deduplication methods
- **Anagram Checker** → Chain `.chars().collect()` functionally

You'll appreciate the abstractions because you'll know what they're hiding!

### Coming Soon

More exercises will be added following the [TAXONOMY.md](../TAXONOMY.md) plan, including:

- Clock Implementation (modulo arithmetic, Display trait)
- Roman Numeral Converter (bidirectional conversion)
- Hamming Distance (dual iteration, error handling)
- Matching Brackets (Vec as stack)
- And many more!

## Estimated Time

**~30-35 hours** for completion (assuming 45-60 minutes per exercise)

## Prerequisites

- Completion of **Track 001: Basic Foundations**
- Comfort with for loops, if/else, basic collections
- Understanding of ownership and borrowing

## Key Learnings

After this track, you'll understand:

- How to design data structures in Rust
- The relationship between data and behavior (structs + methods)
- When to use `Vec` vs `HashMap` vs `HashSet`
- How validation and error handling work
- Algorithmic problem-solving patterns

## Next Steps

After completing this track:

1. Notice patterns in your loop-based solutions
2. Ask yourself: "Could this be abstracted?"
3. Move to **Track 003: Advanced** to see functional solutions
4. Compare your intermediate solutions with advanced versions

**Now you're ready for abstractions.**
