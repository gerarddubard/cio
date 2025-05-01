//! # CIO - Console I/O for Rust
//!
//! CIO provides two procedural macros (`printf!` and `input!`) that improve
//! console input/output operations in Rust, bringing Python's convenience
//! to Rust's safely typed environment.
//!
//! ## Features
//!
//! ### `printf!`
//!
//! An enhanced formatted display macro that allows for better visualization
//! of complex data structures such as multidimensional arrays.
//!
//! ```
//! use cio::printf;
//!
//! let arr = vec![vec![1, 2], vec![3, 4]];
//! printf!("Array: {arr:a}");
//! ```
//!
//! ### `input!`
//!
//! A user input macro with built-in type validation.
//!
//! ```
//! use cio::input;
//!
//! let number: i32 = input!("Enter a number: ");
//! ```

mod printf;
mod input;

use proc_macro::TokenStream;

/// Procedural macro that implements formatted output similar to C's printf
/// but with enhanced features for Rust.
///
/// # Format
/// - `{expr}` : Displays the value of the expression
/// - `{expr:spec}` : Displays the value with a format specifier
///
/// # Specifiers
/// - `a` : Format for arrays (with special formatting)
/// - `c` : Simple debug format
/// - `j` : Pretty debug format
/// - Other specifiers conforming to `std::fmt`
///
/// # Example
///
/// printf!("Value: {x}, Array: {arr:a}");
///
#[proc_macro]
pub fn printf(input: TokenStream) -> TokenStream {
    printf::printf_impl(input)
}

/// Procedural macro that facilitates user input and validation.
///
/// This macro displays a message and then waits for valid user input.
/// If the input is empty or cannot be parsed according to the expected return type,
/// it displays an error and asks for input again.
///
/// # Example
///
/// let number: i32 = input!("Enter a number: ");
///
#[proc_macro]
pub fn input(input: TokenStream) -> TokenStream {
    input::input_impl(input)
}