# CIO - Console Input/Output for Rust

[![Crates.io](https://img.shields.io/crates/v/cio.svg)](https://crates.io/crates/cio)
[![Documentation](https://docs.rs/cio/badge.svg)](https://docs.rs/cio)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**CIO** brings Python-like simplicity to Rust console operations with powerful procedural macros that enhance `println!` and provide type-safe `input!`. Experience advanced table formatting, ANSI colors, mathematical matrices, and seamless JSON integration.

## ✨ Key Features

- 🎨 **Rich ANSI Colors**: Easy `@(color, style)` syntax
- 📊 **Advanced Table Formatting**: Professional layouts with Unicode borders
- 🔢 **Mathematical Matrices**: Beautiful matrix display with `:m` and `:d` formats
- 📥 **Type-Safe Input**: Automatic parsing with `input!` macro
- 🚀 **Dynamic Separators**: Custom output control with `$(...)`
- 🌟 **JSON Native Support**: Seamless `serde_json` integration
- 📋 **Multiple Format Specifiers**: `:t`, `:m`, `:d`, `:a`, `:c`, `:j`

## 🚀 Quick Start

### Installation

```toml
[dependencies]
cio = "0.5.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Basic Usage

```rust
use cio::{println, input};
use serde_json::json;

fn main() {
    // Type-safe input with automatic parsing
    let name: String = input!("Your name: ");
    let age: i32 = input!("Your age: ");
    
    // Rich colored output
    println!("Hello, @(green, bold){name}@()! You are @(yellow){age}@() years old.");
    
    // Advanced table formatting
    let data = json!({"France": "Paris", "Germany": "Berlin"});
    println!("Capitals: {data:t(Country, Capital)}");
}
```

## 📥 Type-Safe Input with `input!`

The `input!` macro provides automatic type parsing with elegant error handling:

```rust
use cio::input;

// Basic types with automatic parsing
let name: String = input!("Your @(green,italic)first name@(): ");
let age: i32 = input!("Your @(yellow)age@(): ");
let height: f64 = input!("Your @(blue)height@() (in meters): ");
let married: bool = input!("Are you @(magenta)married@()? (true/false): ");
let favorite_letter: char = input!("What is your @(cyan,italic)favorite letter@()? ");
```

**Features:**
- ✅ **Automatic Type Conversion**: Handles all standard Rust types
- ✅ **Color Support in Prompts**: Use `@(color)` syntax in input prompts
- ✅ **Error Resilience**: Automatically retries on invalid input
- ✅ **Zero Runtime Cost**: All parsing happens at compile-time

## 🎨 Enhanced `println!` with ANSI Colors

### Color Syntax

Use `@(color, style)` for rich terminal output:

```rust
use cio::println;

// Basic colors
println!("@(red)Error:@() Something went wrong");
println!("@(green,bold)Success!@() Operation completed");

// Multiple styles
println!("@(blue,bold,italic)Important Notice@() - Please read carefully");

// Reset with @()
println!("Normal text @(yellow)highlighted@() back to normal");
```

### Available Colors

**Standard Colors**: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
**Bright Colors**: `bright_red`, `bright_green`, `bright_blue`, `bright_cyan`, `bright_magenta`, `bright_yellow`, `bright_white`, `bright_black`
**Styles**: `bold`, `italic`, `underline`, `dimmed`, `blink`, `reversed`, `hidden`, `strikethrough`

### Dynamic Colors

```rust
let color_style = "bright_blue,bold,italic";
let temp = 18.7;
println!("@({color_style}){temp:.1}°C@()");
```

## 🚀 Dynamic Separators with `$(...)`

Control output flow with dynamic separators:

```rust
use cio::println;

// Progress indicators without newlines
for i in 1..10 {
    println!("@(yellow){i}$( → )");  // Custom separator
}
println!("@(yellow)10");  // Final item with newline

// Output: 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10
```

**Separator Options:**
- `$( → )` - Custom separator string
- `$(\n)` - Explicit newline
- `$(sep_variable)` - Dynamic separator from variable

## 📊 Advanced Table Formatting

### Smart Table Format (`:t`)

CIO automatically detects data structure and creates professional tables:

```rust
use cio::println;
use serde_json::json;

// JSON object → Key-value table
let capitals = json!({
    "France": "Paris",
    "Italy": "Rome", 
    "Germany": "Berlin",
    "Spain": "Madrid"
});
println!("European Capitals: {capitals:t(Country, Capital)}");
```

**Output:**
```
┌─────────┬─────────┐
│ Country │ Capital │
├─────────┼─────────┤
│ France  │ Paris   │
├─────────┼─────────┤
│ Germany │ Berlin  │
├─────────┼─────────┤
│ Italy   │ Rome    │
├─────────┼─────────┤
│ Spain   │ Madrid  │
└─────────┴─────────┘
```

### Complex 3D Structures

CIO handles deeply nested JSON with hierarchical headers:

```rust
let class_grades = json!({
    "6A": {
        "Français": [16.0, 13.0, 18.0, 15.0, 17.0],
        "Mathématiques": [15.0, 11.0, 16.0, 14.0, 15.0]
    },
    "6B": {
        "Français": [14.0, 12.0, 15.0, 13.0, 14.0],
        "Mathématiques": [12.0, 11.0, 14.0, 13.0, 11.0]
    }
});
println!("Class Grades: {class_grades:t}");
```

**Output:**
```
                ┌──────────────────────────────────┬──────────────────────────────────┐
                │                6A                │                6B                │
┌───────────────┼──────┬──────┬──────┬──────┬──────┼──────┬──────┬──────┬──────┬──────┤
│ Français      │ 16.0 │ 13.0 │ 18.0 │ 15.0 │ 17.0 │ 14.0 │ 12.0 │ 15.0 │ 13.0 │ 14.0 │
├───────────────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┤
│ Mathématiques │ 15.0 │ 11.0 │ 16.0 │ 14.0 │ 15.0 │ 12.0 │ 11.0 │ 14.0 │ 13.0 │ 11.0 │
└───────────────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┘
```

### Native Rust Collections

CIO works seamlessly with standard Rust collections:

```rust
use std::collections::{HashMap, BTreeMap, HashSet, VecDeque};

// HashMap
let mut capitals = HashMap::new();
capitals.insert("France", "Paris");
capitals.insert("Germany", "Berlin");
println!("HashMap Display: {capitals:t(Country, Capital)}");

// BTreeMap (automatically sorted)
let mut btree_map = BTreeMap::new();
btree_map.insert("C", 3);
btree_map.insert("A", 1);
btree_map.insert("B", 2);
println!("BTreeMap: {btree_map:t(Key, Value)}");

// HashSet
let mut hash_set = HashSet::new();
hash_set.insert("apple");
hash_set.insert("banana");
println!("HashSet: {hash_set:t}");

// VecDeque
let mut vecdeque = VecDeque::new();
vecdeque.push_back("Second");
vecdeque.push_front("First");
println!("VecDeque: {vecdeque:t}");
```

## 🔢 Mathematical Matrix Formatting

### Matrix Format (`:m`)

Display matrices with proper mathematical notation:

```rust
let matrix_2d = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
println!("3x3 Matrix: {matrix_2d:m}");
```

**Output:**
```
⎛  1  2  3  ⎞
│  4  5  6  │
⎝  7  8  9  ⎠
```

### Determinant Format (`:d`)

Display determinants with vertical bars:

```rust
let matrix_small = vec![vec![4, 3], vec![2, 1]];
println!("Determinant: {matrix_small:d}");
```

**Output:**
```
│  4  3  │
│  2  1  │
```

### Matrix Operations

```rust
let matrix = vec![vec![4, 3], vec![2, 1]];
let sum: i32 = matrix.iter().flatten().sum();

println!("Matrix: {matrix:m}");
println!("Sum of all elements: @(bright_green){sum}");
```

## 📋 Complete Format Specifier Reference

| Format | Description | Best For |
|--------|-------------|----------|
| `:t` | Smart table formatting | Data display, JSON objects, collections |
| `:t(Col1, Col2)` | Table with custom headers | Structured data with specific column names |
| `:m` | Mathematical matrix notation | 2D arrays, mathematical computations |
| `:d` | Determinant notation | Square matrices, mathematical expressions |
| `:a` | Array format with indentation | Nested data structures, debug output |
| `:c` | Compact single-line format | Dense data, debugging |
| `:j` | JSON pretty-print format | Complex objects, configuration files |

### Format Comparison Example

```rust
let demo_matrix = json!([[1, 2, 3], [4, 5, 6]]);
let demo_native = vec![vec![1, 2, 3], vec![4, 5, 6]];

println!("Standard: {demo_matrix}");
println!("Compact: {demo_matrix:c}");
println!("Array: {demo_matrix:a}");
println!("Matrix: {demo_native:m}");
println!("Table: {demo_matrix:t}");
```

## 💡 Advanced Examples

### Employee Data Analysis

```rust
use cio::println;
use std::collections::HashMap;

let dataset = vec![
    ("Alice", 28, 75.5, "Marketing"),
    ("Bob", 35, 82.3, "Engineering"),
    ("Charlie", 42, 91.0, "Management"),
];

// Advanced data analysis with turbofish
let mut dept_count = HashMap::<&str, i32>::new();
let mut dept_ages = HashMap::<&str, Vec<i32>>::new();

for (_name, age, _weight, dept) in &dataset {
    *dept_count.entry(*dept).or_insert(0) += 1;
    dept_ages.entry(*dept).or_insert_with(Vec::<i32>::new).push(*age);
}

println!("@(blue,bold)Department Statistics:");
for (dept, count) in &dept_count {
    let ages = &dept_ages[dept];
    let avg_age = ages.iter().sum::<i32>() as f64 / ages.len() as f64;
    println!("@(cyan){dept}@(): {count} people, avg age: {avg_age:.1}");
}
```

### Sales Data Analysis

```rust
let sales_data = vec![
    ("Q1", "North", "Software", 12000.0, 150),
    ("Q1", "South", "Hardware", 15000.0, 75),
    ("Q2", "North", "Software", 14500.0, 180),
];

// Multi-dimensional analysis
let quarterly_summary: HashMap<&str, HashMap<&str, HashMap<&str, (f64, i32)>>> =
    sales_data.iter()
        .fold(
            HashMap::<&str, HashMap<&str, HashMap<&str, (f64, i32)>>>::new(),
            |mut acc, (quarter, region, product, revenue, units)| {
                let quarter_entry = acc.entry(*quarter)
                    .or_insert_with(HashMap::<&str, HashMap<&str, (f64, i32)>>::new);
                let region_entry = quarter_entry.entry(*region)
                    .or_insert_with(HashMap::<&str, (f64, i32)>::new);
                let product_entry = region_entry.entry(*product).or_insert((0.0, 0));
                product_entry.0 += *revenue;
                product_entry.1 += *units;
                acc
            }
        );

println!("@(blue,bold)Quarterly Summary:");
for (quarter, regions) in &quarterly_summary {
    println!("@(cyan){quarter}:@()");
    for (region, products) in regions {
        println!("  @(yellow){region}:@()");
        for (product, (revenue, units)) in products {
            println!("    {product}: ${revenue:.0} revenue, {units} units");
        }
    }
}
```

## 🎯 Best Practices

### When to Use JSON vs Native Collections

**Use JSON (`json!`) for:**
- ✅ Static data structures and demonstrations
- ✅ Clean, readable data declarations  
- ✅ Complex nested structures for display
- ✅ Table formatting examples

```rust
// Perfect for JSON
let config = json!({
    "database": {
        "host": "localhost",
        "port": 5432,
        "name": "myapp"
    }
});
println!("Configuration: {config:t}");
```

**Use Native Collections for:**
- ✅ Data analysis and computation
- ✅ Operations like `.iter()`, `.get()`, `.insert()`
- ✅ Performance-critical code
- ✅ Type safety and compile-time checks

```rust
// Perfect for Native
let mut sales = HashMap::new();
sales.insert("Q1", 50000.0);
sales.insert("Q2", 75000.0);

let total: f64 = sales.values().sum();  // Native operations
println!("Sales Analysis: {sales:t(Quarter, Revenue)}");
println!("Total: @(green,bold)${total:.0}@()");
```

### Color Usage Guidelines

```rust
// System messages
println!("@(red,bold)Error:@() Connection failed");
println!("@(yellow,bold)Warning:@() Low disk space");
println!("@(green,bold)Success:@() File saved");
println!("@(blue,bold)Info:@() Processing...");

// Data highlighting
println!("Temperature: @(bright_red){temp:.1}°C@() (above normal)");
println!("Status: @(bright_green,bold)ACTIVE@()");

// Progress indicators
for i in 1..=5 {
    println!("Step {i}: @(cyan)Processing@()$( → )");
}
```

### Expression Integration

```rust
let first_name = "John";
let last_name = "Doe";
let age = 30;
let height = 1.85;

// Direct expressions in placeholders
println!("Age in months: @(yellow){age * 12}");
println!("Height in cm: @(blue){height * 100.0:.0}");
println!("Full name: @(green,bold){first_name} {last_name.to_uppercase()}@()");

// Complex expressions
let letter = 'A';
let category = if letter.is_alphabetic() { "letter" } else { "symbol" };
println!("Character '@(magenta){letter}@()' is a {category}");
```

## 🔧 Technical Details

### Performance Characteristics

- **Zero Runtime Overhead**: All formatting decisions made at compile-time
- **Memory Efficient**: Optimized string building with capacity pre-allocation
- **ANSI Optimized**: Efficient color code generation and reset management
- **Unicode Safe**: Proper handling of multi-byte characters in alignment

### Compatibility

- **Rust Version**: 1.70+ required for procedural macro features
- **Terminal Support**: Works with all ANSI-compatible terminals
- **Platform Support**: Cross-platform (Windows, macOS, Linux)
- **Fallback Handling**: Graceful degradation on unsupported terminals

### Error Handling

```rust
// Input macro handles errors gracefully
let number: i32 = input!("Enter a number: ");  // Retries automatically on invalid input

// Println macro provides safe defaults
let data = problematic_data;
println!("Data: {data:t}");  // Falls back to Debug format if JSON fails
```

## 📚 Examples Repository

All examples from this README are available in a complete demonstration:

```bash
cargo new cio_demo
cd cio_demo
# Add the complete main.rs example from the repository
cargo run
```

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🚀 Roadmap

- [ ] Custom color themes
- [ ] More mathematical formats (vectors, tensors)
- [ ] Export formats (CSV, JSON, HTML)
- [ ] Interactive table editing
- [ ] Graph/chart ASCII rendering
- [ ] Custom format specifier plugins
