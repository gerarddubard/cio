// println.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Token, punctuated::Punctuated, Expr, parse::{Parse, ParseStream}};
use regex::Regex;
use crate::formatext;

/// Advanced procedural macro for console output with rich formatting capabilities.
///
/// This macro extends the standard `println!` with:
/// - ANSI color and style formatting using @(...) syntax
/// - Matrix and container pretty-printing with specialized formats
/// - Dynamic separators using $(...) syntax
/// - Rich expression evaluation in format strings
///
/// # Format Specifiers
/// - `:a` - Array format with proper indentation for nested structures
/// - `:c` - Compact single-line format for any data structure
/// - `:j` - JSON-like pretty format for complex structures
/// - `:m` - Matrix format with proper borders for 2D arrays
/// - `:d` - Determinant format with vertical bars
///
/// # Style Syntax
/// - Basic: `@(red, bold)Hello @(blue)World@()`
/// - Dynamic: `@(color_var)Text@()` where color_var is a variable containing style names
/// - Reset: `@()` resets to default style
///
/// # Examples
///
/// println!("@(green, bold)Success:@() Operation completed in @(yellow){time}@() seconds");
/// println!("Matrix: \n{matrix:m}");  // Display with matrix formatting
/// println!("Values: @(cyan){values:a}@()");  // Array with colors
/// println!("@(bright_blue)Item@() @(bright_red){idx}@()$( | )"); // With separator

struct PrintlnInput {
    format_string: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for PrintlnInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format_string = input.parse()?;
        let mut args = Punctuated::new();
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            args = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        }
        Ok(PrintlnInput { format_string, args })
    }
}

pub fn println_impl(input: TokenStream) -> TokenStream {
    let PrintlnInput { format_string, args: _args } = parse_macro_input!(input as PrintlnInput);
    let mut fmt_str = format_string.value();

    // Pattern to detect $(...)
    let sep_pattern = Regex::new(r"\$\(([^)]*)\)$").unwrap();
    let sep_content = if let Some(caps) = sep_pattern.captures(&fmt_str) {
        caps.get(1).map(|m| m.as_str().to_string())
    } else {
        None
    };

    // Check if this is the specific call from input.rs with $("")
    let is_input_call = sep_content.as_ref().map_or(false, |s| s == "\"\"");

    // Remove $(...) unless it's an input.rs call
    if sep_content.is_some() && !is_input_call {
        fmt_str = sep_pattern.replace(&fmt_str, "").to_string();
    } else if is_input_call {
        // In this case, just remove $("") for input.rs
        fmt_str = sep_pattern.replace(&fmt_str, "").to_string();
    }

    // Determine if we should add a newline
    let no_newline = sep_content.is_some();

    // Process the format string
    let (tokens, used_vars) = formatext::parse_format_string(&fmt_str);

    // Generate display code for the main content
    let mut segments = formatext::generate_output_code(&tokens, no_newline);

    // If a separator is specified via $(...) and it's not an input.rs call
    if let Some(sep_var) = sep_content {
        if !segments.is_empty() && !is_input_call {
            // Remove the last instruction (containing print and flush)
            let last_segment = segments.pop().unwrap_or_default();

            // Extract the print part without the flush and put it back
            if let Some(print_part) = last_segment.split(';').next() {
                segments.push(format!("{};", print_part));
            }

            // Check if it's a valid Rust identifier (variable) or a literal
            let is_valid_ident = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap().is_match(&sep_var);

            // Generate appropriate code based on separator type
            let sep_code = if is_valid_ident {
                // It's a variable
                format!("print!(\"{{}}\", {});", sep_var)
            } else {
                // It's a literal value
                format!("print!(\"{}\");", sep_var)
            };

            segments.push(sep_code);
            segments.push("std::io::stdout().flush().expect(\"Failed to flush stdout\");".to_string());
        }
    }

    // Generate code to suppress warnings for unused variables
    let mut suppress_warnings = Vec::new();
    for var in used_vars {
        suppress_warnings.push(format!("let _ = &{};", var));
    }
    let suppressions = suppress_warnings.join(" ");

    // Get helper functions
    let helper_functions = formatext::get_helper_functions();

    // Include colorstyle.rs functions directly (single definition)
    let colorstyle_code = r#"mod colorstyle_inline {
        pub fn escape_string(s: &str) -> String {
            s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n")
        }
        pub fn ansi_code_for_style(styles: &[String]) -> String {
            if styles.is_empty() { return "\x1B[0m".to_string(); }
            let mut codes = Vec::new();
            for style in styles {
                match style.as_str() {
                    "black" => codes.push("30"),
                    "red" => codes.push("31"),
                    "green" => codes.push("32"),
                    "yellow" => codes.push("33"),
                    "blue" => codes.push("34"),
                    "magenta" => codes.push("35"),
                    "cyan" => codes.push("36"),
                    "white" => codes.push("37"),
                    "bright_black" | "gray" => codes.push("90"),
                    "bright_red" => codes.push("91"),
                    "bright_green" => codes.push("92"),
                    "bright_yellow" => codes.push("93"),
                    "bright_blue" => codes.push("94"),
                    "bright_magenta" => codes.push("95"),
                    "bright_cyan" => codes.push("96"),
                    "bright_white" => codes.push("97"),
                    "bold" => codes.push("1"),
                    "italic" => codes.push("3"),
                    "underline" => codes.push("4"),
                    "dimmed" => codes.push("2"),
                    "blink" => codes.push("5"),
                    "reversed" => codes.push("7"),
                    "hidden" => codes.push("8"),
                    "strikethrough" => codes.push("9"),
                    _ => {},
                }
            }
            if codes.is_empty() { return "\x1B[0m".to_string(); }
            format!("\x1B[{}m", codes.join(";"))
        }
    }"#;

    // Replace calls to crate::colorstyle with colorstyle_inline
    let processed_segments: Vec<String> = segments
        .iter()
        .map(|seg| seg.replace("crate::colorstyle", "colorstyle_inline"))
        .collect();

    // Finalize the code
    let segments_code = processed_segments.join("\n            ");
    let final_code = format!(
        r#"{{
            {helper}
            {colorstyle}
            {suppressions}
            let mut result = String::new();
            {segments}
        }}"#,
        helper = helper_functions,
        colorstyle = colorstyle_code,
        suppressions = suppressions,
        segments = segments_code
    );

    // Parse the generated code into tokens
    use syn::parse_str;
    let generated_code = match parse_str::<Expr>(&final_code) {
        Ok(code) => code,
        Err(e) => {
            let err_msg = format!("Error parsing in println: {}", e);
            let err_tokens = quote! { compile_error!(#err_msg) };
            return TokenStream::from(err_tokens);
        }
    };

    // Generate final code
    let result = quote! {
        {
            use std::io::Write;
            #generated_code
        }
    };

    TokenStream::from(result)
}