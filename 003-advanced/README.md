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

**Currently Available: 0 exercises** (coming soon!)

Run with: `cargo run -p <folder_name>`

### Functional Rewrites (Compare with Track 002!)

These exercises solve the **exact same problems** as Track 002, but using functional programming patterns. Open both versions side-by-side to see what abstractions hide!

| #   | Exercise                       | Folder                      | Compare With                                                    | Key Abstraction               |
| --- | ------------------------------ | --------------------------- | --------------------------------------------------------------- | ----------------------------- |
| 1   | Median (Functional)            | median_functional           | [median_loops](../002-intermediate/median_loops/)               | `.get()`, slice patterns      |
| 2   | Mode (Functional)              | mode_functional             | [median_loops](../002-intermediate/median_loops/)               | `.max_by_key()`               |
| 3   | Company (Functional)           | company_functional          | [company_departments](../002-intermediate/company_departments/) | `.entry()` API, `.collect()`  |
| 4   | Array Statistics (Functional)  | array_stats_functional      | [array_statistics](../002-intermediate/array_statistics/)       | `.fold()`, `.map()`           |
| 5   | Duplicate Remover (Functional) | duplicate_remove_functional | [duplicate_remover](../002-intermediate/duplicate_remover/)     | `.collect::<HashSet>()`       |
| 6   | Anagram (Functional)           | anagram_functional          | [anagram_checker](../002-intermediate/anagram_checker/)         | `.chars().sorted().collect()` |

### Advanced Exercises (New Concepts)

These exercises introduce concepts not covered in previous tracks: recursion, graph algorithms, dynamic programming, and advanced trait usage.

| #   | Exercise                   | Folder              | Key Concepts                     |
| --- | -------------------------- | ------------------- | -------------------------------- |
| 7   | Tournament Results         | tournament_results  | Custom comparators, sorting      |
| 8   | Beer Song Generator        | beer_song           | Range iteration, string building |
| 9   | Reverse String (Recursive) | reverse_recursive   | Recursion basics, base cases     |
| 10  | Factorial (Recursive)      | factorial_recursive | Memoization, recursion           |
| 11  | Binary Search              | binary_search       | Classic algorithm, O(log n)      |
| 12  | Simple Grep                | simple_grep         | String filtering, iterators      |
| 13  | Pythagorean Triplet        | pythagorean_triplet | Iterator combinations            |
| 14  | Two Bucket Problem         | two_bucket          | BFS, state exploration           |

### More Coming Soon

Additional exercises will cover: dynamic programming, graph traversal, custom data structures with generics, trait implementations, and more!

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

For the functional rewrites, **open your Track 002 solution side-by-side**:

```rust
// Track 002 (loops) - median_loops
let mut max_count = 0;
let mut mode = None;
for (num, count) in &counts {
    if *count > max_count {
        max_count = *count;
        mode = Some(*num);
    }
}

// Track 003 (functional) - mode_functional
let mode = counts.iter()
    .max_by_key(|(_, count)| *count)
    .map(|(num, _)| *num);
```

**Ask yourself:** What is `.max_by_key()` hiding? (Answer: Your for loop!)

This is why you learned loops first. Now you appreciate the abstraction.

## Next Steps

After completing this track:

1. Review all three tracks - see your evolution
2. Tackle **Projects** for integrative challenges
3. Contribute to open-source Rust projects
4. Read "Rust for Rustaceans" by Jon Gjengset

**You are now the owner of your code.**
