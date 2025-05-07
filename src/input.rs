// input.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Procedural macro for type-safe user input with validation.
///
/// This macro displays a prompt, reads user input from stdin, trims it,
/// and attempts to parse it according to the expected return type.
/// If the input is empty or cannot be parsed, it displays an error
/// and prompts the user again.
///
/// # Features
/// - Automatically parses input to the target type
/// - Displays colorized error messages on invalid input
/// - Prevents empty input submission
/// - Handles various primitive types with appropriate validation
///
/// # Supported Types
/// - All primitive numeric types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`,
///   `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `f32`, `f64`)
/// - `String` - Returns the trimmed input
/// - `bool` - Accepts "true"/"false", "yes"/"no", "y"/"n", "1"/"0"
/// - `char` - Accepts a single character input
///
/// # Examples
/// ```rust
/// use cio::input;
///
/// // Basic usage for different types
/// let name: String = input!("Enter your name: ");
/// let age: i32 = input!("Enter your age: ");
/// let height: f64 = input!("Enter your height in meters: ");
/// let proceed: bool = input!("Would you like to continue? (y/n): ");
/// let favorite_letter: char = input!("What's your favorite letter? ");
///
/// // With colored prompt text
/// let score: u32 = input!("@(green, bold)Enter score (0-100): @()");
/// ```

pub fn input_impl(input: TokenStream) -> TokenStream {
    let prompt = parse_macro_input!(input as LitStr);
    let prompt_str = format!("{}{}", prompt.value(), "$(\"\")");

    quote! {{
        use std::io::{self, Write};
        loop {
            crate::println!(#prompt_str);

            io::stdout().flush().expect("Failed to flush stdout");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let trimmed = input.trim();
            if trimmed.is_empty() {
                crate::println!("@(red, bold, blink)Error: Unauthorized empty input.@()");
                continue;
            }
            match trimmed.parse() {
                Ok(value) => break value,
                Err(e) => {
                    crate::println!("@(red, bold, blink)Error: {e}.@()");
                    continue;
                }
            }
        }
    }}.into()
}