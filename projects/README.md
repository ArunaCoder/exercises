# Projects: Integrative Challenges

**Learning Focus**: Combining multiple concepts into complete, real-world applications

## Philosophy

You've completed the tracks. Now build something real.

These projects combine:

- Multiple data structures
- File I/O and serialization
- Error handling across boundaries
- User interaction
- Real-world problem-solving

## Project List

**Currently Available: 0 projects** (coming soon!)

Run with: `cargo run -p <folder_name>`

### CLI Todo List Manager

**Folder:** `todo_list_cli`

Build a command-line todo list application with persistence.

**Features:**

- Add, remove, complete, and list tasks
- Save/load from JSON file
- Filter by status (pending, completed, all)
- Due dates and priorities

**Concepts:** File I/O, serialization (serde), command parsing, error handling, HashMap

**Estimated time:** 6-8 hours

---

### Text Adventure Game

**Folder:** `text_adventure_game`

Create an interactive text-based adventure game.

**Features:**

- Multiple rooms with descriptions
- Item inventory system
- Puzzle-solving logic
- Save/load game state
- Game loop with command parsing

**Concepts:** State machine, enums, user input, HashMap for rooms, Vec for inventory

**Estimated time:** 8-10 hours

---

### HTTP Log Analyzer

**Folder:** `http_log_analyzer`

Parse and analyze web server logs.

**Features:**

- Read log files (Apache/Nginx format)
- Extract: IP addresses, timestamps, URLs, status codes
- Calculate statistics: requests per hour, most visited pages, error rates
- Generate summary report

**Concepts:** File reading, regex, HashMap statistics, formatting, error handling

**Estimated time:** 6-8 hours

---

### Simple Web Scraper

**Folder:** `web_scraper`

Fetch and extract data from web pages.

**Features:**

- HTTP GET requests to URLs
- Parse HTML to extract specific elements
- Save data to CSV or JSON
- Handle rate limiting and errors

**Concepts:** HTTP client (reqwest), HTML parsing, CSV writing, error propagation

**Estimated time:** 8-10 hours

---

### Markdown to HTML Converter

**Folder:** `markdown_converter`

Convert Markdown syntax to HTML.

**Features:**

- Parse: headers, bold, italic, links, lists, code blocks
- State machine for multi-line blocks
- Generate valid HTML output
- Support for nested structures

**Concepts:** Parsing, state machine, string building, regex, recursion

**Estimated time:** 10-12 hours

---

## Additional Project Ideas

If you complete the main 5, try these:

- **CSV Data Analyzer**: Read CSV, compute statistics, generate charts (text-based)
- **Password Generator**: Customizable passwords with strength validation
- **File Organizer**: Sort files into folders by type, date, size
- **JSON API Client**: Fetch data from public APIs, deserialize, display
- **Simple Chat Server**: TCP server with multiple clients (networking)
- **Expression Evaluator**: Parse and evaluate mathematical expressions
- **Hangman Game**: Classic word guessing with ASCII art
- **RSS Feed Reader**: Fetch and parse RSS feeds
- **ASCII Art Generator**: Convert images to ASCII
- **Pomodoro Timer**: CLI productivity timer with notifications

## Prerequisites

- Completion of at least **Track 001** and **Track 002**
- Basic understanding of:
  - File I/O
  - Error handling (`Result`, `?` operator)
  - Command-line arguments
  - Optional: serde, reqwest, regex crates

## Resources

### Useful Crates

- **serde** + **serde_json**: JSON serialization/deserialization
- **clap**: Command-line argument parsing
- **reqwest**: HTTP client
- **regex**: Regular expressions
- **csv**: CSV reading/writing
- **chrono**: Date and time handling

### Learning Materials

- [Rust Book Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) - I/O Project
- [Command Line Apps in Rust](https://rust-cli.github.io/book/)
- [docs.rs](https://docs.rs) - Crate documentation

## How to Approach Projects

1. **Plan first**: Write down features, data structures, functions
2. **Start simple**: Get basic version working, then add features
3. **Handle errors**: Use `Result` and `?` operator properly
4. **Test incrementally**: Test each feature as you build
5. **Refactor**: Clean up code once it works
6. **Document**: Write a README explaining usage

## Next Steps

After completing projects:

- Share your work on GitHub
- Write blog posts explaining your approach
- Contribute to open-source Rust projects
- Build your own project ideas
- Explore advanced topics: async, macros, unsafe

**Congratulations on your Rust journey!**
