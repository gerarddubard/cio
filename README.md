# CIO - Console Input/Output for Rust

CIO provides two powerful procedural macros (`println!` and `input!`) that bring Python-like convenience to Rust's type-safe environment. These macros streamline console interaction with advanced formatting and validation capabilities.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cio = "0.4.0"
```

## Features Overview

### `println!` macro
- **Python f-string style** direct variable insertion (`{var}` instead of `{}` with separate arguments)
- **ANSI color and style** formatting with `@(...)` syntax
- **Special formatters** for data structures and collections (`:a`, `:c`, `:j`, `:m`, `:d`)
- **Separator control** with `$(...)` syntax to unify `print!` and `println!`
- **Rich expression evaluation** in format strings
- **Pretty-print** for all `std::collections` types with intuitive formatting

### `input!` macro
- **Type-safe input** with automatic parsing and validation
- **Error handling** with colored error messages and re-prompting
- **Support for multiple types** (strings, numbers, booleans, chars)
- **Validation of input length** (especially for `char` inputs)

## Examples

### 1. Basic Input and Output

```rust
use cio::{println, input};

// Type-safe input collection with validation
let name: String = input!("Your name: ");
let age: i32 = input!("Your age: ");
let height: f64 = input!("Your height (in meters): ");
let married: bool = input!("Are you married? (true/false): ");
let favorite_letter: char = input!("What is your favorite letter? ");

// Direct variable names in placeholders (Python f-string style)
println!("Hello, {name}, you are {age} years old!");
```

### 2. Colored and Styled Output

```rust
// Apply colors and styles with @(...) syntax
println!("@(green, bold)Success:@() Operation completed!");
println!("@(red, italic)Error:@() Something went wrong.");
println!("@(blue)Information:@() System is @(yellow)running@() normally.");

// Mix variables with styling
println!("@(bright_cyan, bold)User:@() {name} (@(bright_green){age}@() years old)");
```

### 3. Rich Expression Evaluation

```rust
// Direct expressions in format placeholders
println!("Age in months: {age * 12}");
println!("Height in cm: {height * 100.0:.0}");
println!("Name in uppercase: {name.to_uppercase()}");

// Conditional expressions
println!("Status: {if age >= 18 { "Adult" } else { "Minor" }}");

// Method calls
println!("First letter: {name.chars().next().unwrap_or('?')}");
```

### 4. Separator Control (Unifying print! and println!)

```rust
// No separator - works like println!
println!("Regular line with newline");

// With separator - works like print! with explicit separator
for i in 1..10 {
    println!("{i}$( - )");  // Prints "1 - 2 - 3 - ..." on one line
}

// Dynamic separators with variables
let separator = " | ";
println!("Item: {item}$(separator)");
```

### 5. Container Formatting

```rust
use std::collections::{HashMap, BTreeMap, VecDeque};

// Arrays and vectors
let numbers = vec![1, 2, 3, 4, 5];
println!("Numbers (array format): {numbers:a}");  // [1, 2, 3, 4, 5]

// Matrices
let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
println!("Matrix (standard format):\n{matrix:a}");  // Indented array
println!("Matrix (matrix format):\n{matrix:m}");    // With matrix borders
println!("Matrix (determinant format):\n{matrix:d}"); // With determinant bars

// HashMap
let mut capitals = HashMap::new();
capitals.insert("France", "Paris");
capitals.insert("Japan", "Tokyo");
println!("Capitals (pretty):\n{capitals:j}");  // JSON-like pretty format
println!("Capitals (compact): {capitals:c}");  // Single-line compact format

// Other collections
let queue = VecDeque::from([1, 2, 3, 4]);
println!("Queue: {queue:a}");  // Pretty prints any collection
```

### 6. Nested Structures

```rust
// Deeply nested data
let nested = vec![
    HashMap::from([
        ("name", "Alice"),
        ("scores", &[95, 87, 92])
    ]),
    HashMap::from([
        ("name", "Bob"),
        ("scores", &[78, 85, 90])
    ])
];

println!("Nested data (pretty):\n{nested:j}");

// 3D arrays
let cube = vec![
    vec![vec![1, 2], vec![3, 4]], 
    vec![vec![5, 6], vec![7, 8]]
];
println!("3D array:\n{cube:a}");  // Properly indented 3D structure
```

### 7. Input Validation

```rust
// String inputs (can't be empty)
let username: String = input!("Username: ");

// Numeric inputs (validates correct type)
let score: u32 = input!("Score (0-100): ");  // Rejects non-numbers or floats

// Boolean inputs (accepts various formats)
let proceed: bool = input!("Continue? (y/n): ");  // Accepts "true"/"false", "yes"/"no", "y"/"n", "1"/"0"

// Character inputs (ensures single character)
let grade: char = input!("Grade (A-F): ");  // Rejects multiple characters
```

### 8. Colored Input Prompts

```rust
// Styled input prompts
let name = input!("@(green, bold)Your name:@() ");
let age = input!("@(bright_cyan)Your age:@() ");

// Errors are automatically displayed in red with blinking
// Error: invalid digit found in string
```

## Available Colors and Styles

### Colors
- Basic: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
- Bright: `bright_black`, `bright_red`, `bright_green`, `bright_yellow`, `bright_blue`, `bright_magenta`, `bright_cyan`, `bright_white`, `gray` (alias for `bright_black`)

### Styles
- `bold`, `italic`, `underline`, `dimmed`, `blink`, `reversed`, `hidden`, `strikethrough`

## Format Specifiers

- `:a` - Array format with proper indentation for nested structures
- `:c` - Compact single-line format for any data structure
- `:j` - JSON-like pretty format for complex structures
- `:m` - Matrix format with proper borders for 2D arrays
- `:d` - Determinant format with vertical bars

## Key Benefits

- **Python-like Simplicity**: Familiar syntax for Python users with Rust's type safety
- **Reduced Boilerplate**: Eliminate repetitive input/output code patterns
- **Visual Enhancement**: Easily add colors and styles for better UX
- **Data Visualization**: Beautiful display of complex data structures
- **Unified Interface**: Consistent syntax for all output formatting needs
- **Type Safety**: Maintain Rust's safety guarantees with convenient syntax

## How It Compares to Python

CIO combines the best of both worlds:

- **Like Python's f-strings**: Direct variable names in format strings (`{var}`)
- **Like Python's input()**: Single-line input collection
- **Beyond Python**: Type validation, error handling, and rich formatting
- **Rust Advantage**: Maintains full type safety while offering convenience

## License

This project is licensed under the MIT License.

---

*CIO: Making console interaction in Rust as enjoyable as Python, but with the safety of Rust.*
