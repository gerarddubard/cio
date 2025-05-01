# CIO - Console I/O for Rust (v0.2.0)

CIO provides two powerful procedural macros (`input!` and `printf!`) that bring Python-like convenience to Rust's type-safe environment.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cio = "0.2.0"
```

## 1. Basic Input and Variable Display

The `input!` macro eliminates boilerplate code for user input handling with built-in validation:

```rust
// Type-safe input collection for various types
let first_name: String = input!("Your first name: ");
let last_name: String = input!("Your last name: ");
let age: i32 = input!("Your age: ");
let height: f64 = input!("Your height (in meters): ");
let married: bool = input!("Are you married? (true/false): ");
let favorite_letter: char = input!("What is your favorite letter? ");

// Simple variable display
let status = if married { "you are" } else { "you are not" };
printf!("Hello, {first_name} {last_name}, you are {age} years old, you are {height} m tall, your favorite letter is '{favorite_letter}', and {status} married.");
```

## 2. Expressions in Placeholders

The `printf!` macro supports complex expressions directly within placeholders:

```rust
// Direct mathematical operations
printf!("Age in months: {age * 12}");
printf!("Height in cm: {height * 100.0:.0}");

// Method calls within placeholders
printf!("Last name in uppercase: {last_name.to_uppercase()}");

// Conditional expressions and ternary-like operations
let letter_category = if favorite_letter.is_alphabetic() {
    if favorite_letter.is_ascii_lowercase() { "lowercase letter" }
    else { "uppercase letter" }
} else if favorite_letter.is_numeric() {
    "digit"
} else {
    "special character"
};
printf!("Letter category: {letter_category}");

// Complex expressions combining multiple operations
let letter_position = favorite_letter.to_uppercase().to_string().chars().next().unwrap() as u8 - 64;
printf!("Letter analysis: {letter_position} (position in alphabet if letter)");
```

## 3. Number Formatting

Full support for Rust's standard formatting options with extended capabilities:

```rust
let pi = std::f64::consts::PI;
printf!("PI with various formats:");
printf!("  - Standard: {pi}");
printf!("  - With 2 decimals: {pi:.2}");
printf!("  - With 6 decimals: {pi:.6}");
printf!("  - Scientific notation: {pi:e}");

printf!("Integer formats for {age}:");
printf!("  - Padded with zeros: {age:04}");
printf!("  - Hexadecimal lowercase: {age:x}");
printf!("  - Hexadecimal uppercase: {age:X}");
printf!("  - Binary: {age:b}");
printf!("  - Octal: {age:o}");
```

## 4. Multidimensional Arrays and Matrices

The `printf!` macro excels at displaying complex data structures:

```rust
// 1D array - Vector
let vec_1d = vec![1, 2, 3, 4, 5];

// 2D array - Square matrices
let matrix_2x2 = vec![vec![4, 3], vec![2, 1]];
let matrix_3x3 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

// 3D, 5D, and even 7D arrays
let vec_3d = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
// (5D and 7D arrays omitted for brevity)

// Format comparison for arrays
printf!("1D Vector (:a - array format):     {vec_1d:a}");
printf!("1D Vector (:c - compact format):   {vec_1d:c}");
printf!("3D Array (:a - array format):\n{vec_3d:a}");

// Matrix display with different formatters
printf!("2x2 Matrix (:m - matrix format):\n{matrix_2x2:m}");
// Output:
// ⎛  4  3  ⎞
// ⎝  2  1  ⎠

printf!("3x3 Matrix (:d - determinant format):\n{matrix_3x3:d}");
// Output:
// │  1  2  3  │
// │  4  5  6  │
// │  7  8  9  │

// Special matrix shapes
printf!("Row matrix (1x4) (:m):\n{matrix_row:m}");
printf!("Column matrix (4x1) (:m):\n{matrix_column:m}");
```

## 5. Associative Containers and Complex Structures

The `printf!` macro handles any collection from `std::collections`:

```rust
// HashMap
let mut hash_map = HashMap::new();
hash_map.insert("France", "Paris");
hash_map.insert("Germany", "Berlin");
hash_map.insert("Italy", "Rome");

// BTreeMap, HashSet, BTreeSet
let mut btree_map = BTreeMap::new();
btree_map.insert("A", 1);
btree_map.insert("C", 3);
btree_map.insert("B", 2);

// Display options
printf!("HashMap (:j - pretty format):\n{hash_map:j}");
printf!("HashMap (:c - compact format): {hash_map:c}");
printf!("BTreeMap (:j - pretty format):\n{btree_map:j}");

// Complex nested structures
let mut cities_data = HashMap::new();
// (Nested structure setup code omitted for brevity)
printf!("3-level nested structure (:j):\n{cities_data:j}");

// Deep access to nested elements
let paris_population = cities_data
    .get("France")
    .and_then(|france| france.get("Paris"))
    .and_then(|paris| paris.get("population"))
    .unwrap_or(&unknown_population);

printf!("Population of Paris: {paris_population}");
```

## 6. Advanced Turbofish and Complex Type Manipulation

The `printf!` macro seamlessly handles complex Rust idioms:

```rust
// Turbofish with method chaining
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Example: Filtering, transformation and collection
let even_squares: Vec<i32> = numbers.iter()
    .filter(|&n| n % 2 == 0)            // Keep only even numbers
    .map(|&n| n * n)                    // Square each number
    .collect();                         // Collect into Vec<i32>

printf!("Original numbers: {numbers:a}");
printf!("Even numbers squared: {even_squares:a}");

// Example: Complex operations with custom types
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

// Collection of custom structures
let people = vec![
    Person::new("Alice", 30),
    Person::new("Bob", 25),
    Person::new("Charlie", 35),
    Person::new("Diana", 28),
];

// Advanced grouping
let mut grouped_by_age: HashMap<i32, Vec<String>> = HashMap::new();
for p in &people {
    grouped_by_age.entry(p.age)
        .or_insert_with(Vec::new)
        .push(p.name.clone());
}

printf!("People grouped by age (:j):\n{grouped_by_age:j}");
```

## 7. Format Recommendation Summary

```rust
printf!("7. Format recommendation summary:");
printf!("- Format :a: Best for arrays and general data structures (adds indentation)");
printf!("- Format :j: Best for maps and complex structures (pretty-printed)");
printf!("- Format :c: Best for compact display (single-line for complex structures)");
printf!("- Format :m: Best for matrices (with special matrix border characters)");
printf!("- Format :d: Best for displaying matrix determinants (with vertical bars)");
```

## Key Features

- **Versatile Formatting**: Handle any collection from `std::collections` (HashMap, BTreeMap, HashSet, BTreeSet, VecDeque, LinkedList, BinaryHeap)
- **Matrix Visualization**: Beautiful display of matrices with proper notation
- **Nested Structures**: Intuitive formatting for deeply nested data structures
- **Expression Support**: Use complex expressions directly within format strings
- **Type Safety**: Maintain Rust's safety guarantees with convenient Python-like syntax
- **Turbofish Compatible**: Works seamlessly with Rust's advanced type system features

## License

This project is licensed under the MIT License.