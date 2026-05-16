# Track 003: Advanced Functional Programming

**Learning Focus**: Iterators, closures, traits, generics, functional patterns

## Philosophy

You've earned your syntactic sugar. Now you'll:

- Use iterator adapters: `.map()`, `.filter()`, `.fold()`
- Write closures and capture environments
- Understand trait bounds and generic implementations
- See the same problems solved elegantly
- Appreciate what abstractions hide

**You're the owner AND the passenger.** You choose when to be explicit and when to be concise.

## What You'll Learn

- Iterator trait and adapters
- Closures: `Fn`, `FnMut`, `FnOnce`
- Functional patterns: map, filter, fold, chain
- Traits: defining, implementing, bounds
- Generics: type parameters, where clauses
- Advanced enums: recursive types, Box
- Recursion: base cases, tail recursion, memoization
- Advanced algorithms: DFS, BFS, dynamic programming
- Custom data structures with generics

## What Makes This Advanced

✅ `.max_by_key()` - you know it finds max with a for loop
✅ `.collect()` - you know it's pushing to a Vec
✅ `.fold(0, |acc, x| acc + x)` - you know it's accumulation
✅ Closures - you understand captured variables
✅ Traits - you can implement your own
✅ Generics - you understand monomorphization

## Exercise List

### Functional Rewrites (Compare with Track 002!)

| #   | Exercise                       | Compare With |
| --- | ------------------------------ | ------------ |
| 067 | Median (Functional)            | 042 (loops)  |
| 068 | Mode (Functional)              | 043 (loops)  |
| 069 | Company (Functional)           | 045 (loops)  |
| 070 | Word Frequency (Functional)    | 047 (loops)  |
| 071 | Array Statistics (Functional)  | 046 (loops)  |
| 072 | Anagram (Functional)           | 049 (loops)  |
| 073 | Duplicate Remover (Functional) | 048 (loops)  |

### Advanced Exercises

| #   | Exercise                       | Key Concepts             |
| --- | ------------------------------ | ------------------------ |
| 074 | Tournament Results             | Custom comparators       |
| 075 | Beer Song Generator            | Range iteration          |
| 076 | Food Chain Song                | Iterator accumulation    |
| 077 | Bottle Song                    | Range with pluralization |
| 078 | Kindergarten Garden            | HashMap parsing          |
| 079 | Poker Hand Evaluator           | Ord trait, complex logic |
| 080 | Twelve Days Song               | Range accumulation       |
| 081 | Word Search Puzzle             | 2D iteration patterns    |
| 082 | Change Calculator              | Dynamic programming      |
| 083 | Knapsack Problem               | DP optimization          |
| 084 | Pythagorean Triplet            | Iterator combinations    |
| 085 | Largest Palindrome Product     | Product finding          |
| 086 | Simple Substitution Cipher     | HashMap mapping          |
| 087 | Gigasecond Calculator          | Date/time operations     |
| 088 | Reverse String (Recursive)     | Recursion basics         |
| 089 | Factorial (Recursive)          | Memoization              |
| 090 | Binary Search                  | Classic algorithm        |
| 091 | Simple Grep                    | String filtering         |
| 092 | Word Wrap                      | Text accumulation        |
| 093 | Minesweeper Board              | Neighbor counting        |
| 094 | Connect Game Winner            | Graph traversal DFS/BFS  |
| 095 | Dominoes Chain                 | Backtracking             |
| 096 | Simple State Machine           | Advanced enums           |
| 097 | Rotational Cipher (Generic)    | Generic ROT-N            |
| 098 | Two Bucket Problem             | BFS state exploration    |
| 099 | Yacht Scoring                  | Complex pattern matching |
| 100 | Linked List Length (Recursive) | Recursive traversal      |
| 101 | Flatten Nested Array           | Recursive enum           |
| 102 | Sublist Checker                | `.windows()` iterator    |
| 103 | All Your Base                  | Base conversion          |
| 104 | Custom Set (Generic)           | Generic implementation   |
| 105 | Circular Buffer (Generic)      | Generic data structure   |
| 106 | Simple Linked List (Generic)   | Box, Option, generics    |

## Estimated Time

**~40-50 hours** for completion (assuming 60-90 minutes per exercise)

## Prerequisites

- Completion of **Track 001** and **Track 002**
- Comfort with structs, enums, collections
- Understanding of manual loop patterns
- Curiosity about functional programming

## Key Learnings

After this track, you'll understand:

- When to use functional style vs explicit loops
- How iterator adapters work internally
- The power (and cost) of abstractions
- How to design generic, reusable code
- Advanced algorithmic patterns

## Compare Your Solutions

For exercises 067-073, **open your Track 002 solution side-by-side**:

```rust
// Track 002 (loops) - Exercise 043
let mut max_count = 0;
let mut mode = None;
for (num, count) in &counts {
    if *count > max_count {
        max_count = *count;
        mode = Some(*num);
    }
}

// Track 003 (functional) - Exercise 068
let mode = counts.iter()
    .max_by_key(|(_, count)| *count)
    .map(|(num, _)| *num);
```

**Ask yourself:** What is `.max_by_key()` hiding? (Answer: Your for loop!)

## Next Steps

After completing this track:

1. Review all three tracks - see your evolution
2. Tackle **Projects** for integrative challenges
3. Contribute to open-source Rust projects
4. Read "Rust for Rustaceans" by Jon Gjengset

**You are now the owner of your code.**
