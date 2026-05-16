# Rust Exercises Repository

A comprehensive collection of Rust exercises designed to take you from beginner to advanced functional programming, organized by learning tracks.

## 🎯 Philosophy: Own Your Code, Don't Just Ride Along

> "The abstraction is only useful when you already know what it's hiding. If you don't know what `max_by_key` is doing under the hood, you are not the owner of your code; you're just a passenger."

This repository teaches Rust by having you **implement logic manually first**, then showing you the elegant functional alternatives **after** you understand what they're abstracting.

## 📚 Repository Structure

```
exercises/
├── 001-basic/          # Foundation: loops, variables, ownership basics
├── 002-intermediate/   # Structures: structs, enums, algorithms
├── 003-advanced/       # Functional: iterators, traits, generics
├── projects/           # Integrative real-world projects
├── TAXONOMY.md         # Complete exercise classification
└── EXERCISES_LIST.md   # Full list of 100 exercises
```

### Track Progression

```
001-basic (33 exercises)
   ↓ Master explicit loops and manual logic
   ↓ Understand ownership and borrowing
   ↓
002-intermediate (33 exercises)
   ↓ Design data structures (structs/enums)
   ↓ Implement algorithms with loops
   ↓
003-advanced (34 exercises)
   ↓ See the same problems solved functionally
   ↓ Use iterators, closures, traits
   ↓
projects (5+ projects)
   → Build complete applications
```

## 🚀 Getting Started

### Prerequisites

- Rust installed ([rustup.rs](https://rustup.rs))
- Basic programming knowledge
- Willingness to write verbose code to learn

### Running Exercises

Each exercise is a separate Cargo package. Run with:

```bash
# List all available exercises
cargo check --workspace

# Run a specific exercise
cargo run -p temperature_converter

# Run tests (when available)
cargo test -p fibonacci_generator

# Build all exercises
cargo build --workspace
```

### Exercise Structure

Every exercise folder contains:

```
exercise_name/
├── Cargo.toml         # Package configuration
├── README.md          # Problem description and instructions
├── src/
│   └── main.rs        # Main implementation (with stubs)
├── initial_main.rs    # Reset point (copy to src/main.rs to restart)
├── solution.rs        # Reference solution (spoiler!)
└── tests/             # Automated tests (when applicable)
```

## 📖 Learning Path

### Track 001: Basic Foundations (~20-25 hours)

**Focus**: Manual implementation, explicit logic, ownership fundamentals

You will write:

- Explicit `for` loops (no `.map()` yet)
- Manual variable management
- Clear, verbose logic

**Start here**: [001-basic/README.md](001-basic/README.md)

**Example exercises**:

- FizzBuzz - loops and conditionals
- Fibonacci Generator - mutable variables and loops
- Palindrome Checker - string manipulation without iterators

### Track 002: Intermediate Structures (~30-35 hours)

**Focus**: Structs, methods, algorithms, collection patterns

You will learn:

- Designing your own data types
- Implementing methods on structs
- HashMap and Vec patterns with loops
- Error handling with `Option` and `Result`

**Start here**: [002-intermediate/README.md](002-intermediate/README.md)

**Example exercises**:

- Grade Calculator - struct with methods
- Median (Loops) - manual median calculation
- Company Departments - HashMap<String, Vec> management

### Track 003: Advanced Functional (~40-50 hours)

**Focus**: Iterators, closures, traits, functional patterns

You will master:

- Iterator adapters: `.map()`, `.filter()`, `.fold()`
- Closures and capture environments
- Trait bounds and generic implementations
- Comparing functional vs explicit solutions

**Start here**: [003-advanced/README.md](003-advanced/README.md)

**Example exercises**:

- Median (Functional) - compare with Track 002 version
- Mode (Functional) - see what `.max_by_key()` abstracts
- Custom Set (Generic) - implement your own data structure

### Projects: Real-World Applications

**Focus**: Combining concepts into complete applications

Build:

- CLI Todo List Manager (file I/O, serialization)
- Text Adventure Game (state machine, user input)
- HTTP Log Analyzer (parsing, statistics)
- Web Scraper (HTTP, HTML parsing)
- Markdown to HTML Converter (parsing, state machine)

**Start here**: [projects/README.md](projects/README.md)

## 🎓 Pedagogical Approach

### Why This Structure?

1. **No Physical Numbering**: Exercise folders have descriptive names (`fizzbuzz`, not `003_fizzbuzz`)
   - Easy to add new exercises without renaming everything
   - Clean package names: `cargo run -p temperature_converter`
   - Order defined in track READMEs, not filesystem

2. **Duplicate Exercises**: Some problems appear in multiple tracks
   - `median_loops` (Track 002) vs `median_functional` (Track 003)
   - Compare your solutions to see what abstractions hide
   - Reinforces that there are multiple valid approaches

3. **Progressive Complexity**:
   - Basic: "Here's how to do it manually"
   - Intermediate: "Here's how to structure it"
   - Advanced: "Here's the elegant way (now that you know why)"

### Learning Principles

✅ **Write verbose code first** - understand every step
✅ **See the pattern** - recognize what gets repeated
✅ **Learn the abstraction** - appreciate what it hides
✅ **Choose wisely** - know when to be explicit vs concise

## 🛠️ Exercise Numbering

Exercises are **not numbered in their folder names**. Order is defined in each track's README:

```
001-basic/
├── fizzbuzz/              # Defined as #3 in README
├── temperature_converter/ # Defined as #2 in README
└── fibonacci_generator/   # Defined as #8 in README
```

**Benefits**:

- Insert new exercises without renumbering
- Reorder exercises by editing README tables
- Multiple ordering possibilities (by difficulty, by concept, etc.)

## 📊 Progress Tracking

Track your progress by checking off exercises in each track's README. Each table has a checkbox column for marking completion.

## 🤝 Contributing

Want to add exercises? Great!

1. Choose the appropriate track
2. Create a descriptive folder name (no numbers)
3. Add entry to track's README with recommended position
4. Include: README, Cargo.toml, src/main.rs, initial_main.rs, solution.rs
5. Ensure `cargo check -p your_exercise` passes

## 📚 Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - complementary exercises
- [Exercism Rust Track](https://exercism.org/tracks/rust) - more practice

## 📝 License

This repository is for educational purposes. Individual exercises may be inspired by various sources including "The Rust Programming Language" book exercises.

## 🎯 Quick Reference

| Command                   | Purpose               |
| ------------------------- | --------------------- |
| `cargo run -p <name>`     | Run specific exercise |
| `cargo test -p <name>`    | Run exercise tests    |
| `cargo check --workspace` | Check all exercises   |
| `cargo build --workspace` | Build all exercises   |

---

**Remember**: You are not a passenger in your code. You are the driver. Start with [Track 001](001-basic/README.md) and own every line you write.
