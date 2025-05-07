// lib.rs
use proc_macro::TokenStream;

/// CIO - Console Input/Output library for Rust
///
/// This library provides two powerful procedural macros that enhance
/// console I/O operations in Rust:
///
/// # Macros
///
/// ## println!
/// An enhanced version of the standard println! macro that adds:
/// - ANSI color and text styling with @() syntax
/// - Special formatting for data structures and collections
/// - Separator control with $() syntax
/// - Rich expression evaluation in format strings
///
/// ## input!
/// A macro for handling user input with built-in validation and type conversion:
/// - Automatic parsing to the target type
/// - Error handling and re-prompting on invalid input
/// - Support for multiple types (numbers, strings, booleans, chars)
/// - Colored error messages
///
/// # Examples
///
/// ```rust
/// use cio::{println, input};
///
/// // Basic colored output
/// println!("@(green, bold)Hello@() @(blue){name}@()!");
///
/// // With data structures
/// println!("Array: {array:a}");  // Pretty-printed array
/// println!("Matrix: \n{matrix:m}");  // Matrix formatting
///
/// // Input with validation
/// let name: String = input!("Your name: ");
/// let age: i32 = input!("Your age: ");
/// let proceed: bool = input!("Continue? (y/n): ");
/// ```

// Internal modules
mod colorstyle;  // ANSI color and style handling
mod extensions;  // Helper functions for container formatting
mod formatext;   // Format string processing
mod println;     // println! macro implementation
mod input;       // input! macro implementation

// Export macros
#[proc_macro]
pub fn println(input: TokenStream) -> TokenStream {
    println::println_impl(input)
}

#[proc_macro]
pub fn input(input: TokenStream) -> TokenStream {
    input::input_impl(input)
}