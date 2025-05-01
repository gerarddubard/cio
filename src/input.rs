use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

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
pub fn input_impl(input: TokenStream) -> TokenStream {
    let prompt = parse_macro_input!(input as LitStr);
    let prompt_str = prompt.value();

    quote! {{
        use std::io::{self, Write};
        loop {
            print!(#prompt_str);
            io::stdout().flush().expect("Failed to flush stdout");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let trimmed = input.trim();
            if trimmed.is_empty() {
                println!("Error: Unauthorized empty input.");
                continue;
            }
            match trimmed.parse() {
                Ok(value) => break value,
                Err(e) => {
                    println!("Error: {e}.");
                    continue;
                }
            }
        }
    }}.into()
}