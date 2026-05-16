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

| #   | Exercise                | Key Concepts               |
| --- | ----------------------- | -------------------------- |
| 034 | Grade Calculator        | Struct with methods        |
| 035 | Simple Inventory System | CRUD operations, structs   |
| 036 | Clock Implementation    | Modulo arithmetic, Display |
| 037 | Queen Attack            | Coordinate logic           |
| 038 | Phone Number Formatter  | String validation          |
| 039 | Roman Numeral Converter | Bidirectional conversion   |
| 040 | Unit Converter          | Enum pattern matching      |
| 041 | Space Age Calculator    | Astronomical calculations  |
| 042 | Median (Loops)          | Vec sorting with loops     |
| 043 | Mode (Loops)            | HashMap max finding        |
| 044 | Pig Latin               | String manipulation        |
| 045 | Company Departments     | HashMap<String, Vec>       |
| 046 | Array Statistics        | Multiple calculations      |
| 047 | Word Frequency          | HashMap word counting      |
| 048 | Duplicate Remover       | HashSet + Vec              |
| 049 | Anagram Checker         | Sort + compare             |
| 050 | Hamming Distance        | Dual iteration, errors     |
| 051 | Matching Brackets       | Vec as stack               |
| 052 | Secret Handshake        | Binary operations          |
| 053 | Allergies               | Binary flags               |
| 054 | Sieve of Eratosthenes   | Prime algorithm            |
| 055 | Bob Chatbot             | Text pattern analysis      |
| 056 | Protein Translation     | Codon mapping              |
| 057 | Saddle Points           | Matrix operations          |
| 058 | ETL Transform           | HashMap transformation     |
| 059 | Grains on Chessboard    | Exponentiation, overflow   |
| 060 | Largest Series Product  | Window-like logic          |
| 061 | Rail Fence Cipher       | 2D vectors, encryption     |
| 062 | Diffie-Hellman          | Modular arithmetic         |
| 063 | Dot Product             | Manual zip iteration       |
| 064 | Matrix Operations       | Nested loops, math         |
| 065 | Spiral Matrix           | 2D generation              |
| 066 | Pascal's Triangle       | Nested Vec generation      |

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
