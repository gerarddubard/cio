//! # CIO - Console Input/Output for Rust
//!
//! A powerful procedural macro crate that brings Python-like simplicity to Rust console operations.
//! Features advanced table formatting, ANSI colors, and type-safe input with elegant syntax.
//!
//! ## Key Features
//!
//! - **Enhanced println!** - ANSI colors with `@(color, style)` syntax
//! - **Advanced table formatting** - Sophisticated layouts with `:t` format specifier
//! - **Custom headers** - Personalized table headers with `:t(Header1, Header2)` syntax
//! - **Type-safe input!** - *Coming soon* - Automatic parsing with validation
//! - **Matrix display** - *Coming soon* - Beautiful formatting for 2D/3D data
//!
//! ## Installation
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! pyrust = "0.1.0"
//! serde_json = "1.0"  # Required for JSON data formatting
//! ```
//!
//! ## Quick Start
//!
//! ```rust
//! use cio::println;
//! use serde_json::json;
//!
//! // Colored output with table formatting
//! let data = json!({"France": "Paris", "Germany": "Berlin"});
//! println!("@(blue, bold)Countries:@() {data:t(Country, Capital)}");
//!
//! // Simple colored messages
//! println!("@(green, bold)Success!@() Operation completed.");
//! println!("@(red)Error:@() Something went wrong.");
//! ```
//!
//! ## Advanced Examples
//!
//!
//! // Table formatting with JSON data
//! let countries = json!({
//!     "France": "Paris",
//!     "Germany": "Berlin",
//!     "Spain": "Madrid"
//! });
//! println!("@(bright_blue, bold)European Capitals:@() {countries:t}");
//!
//! ## Table Format Specifiers
//!
//! - `:t` - Smart table formatting with automatic structure detection
//! - `:t(Col1, Col2)` - Custom column headers
//! - `:m` - Matrix format with mathematical brackets
//! - `:a` - Array format with proper indentation
//! - `:c` - Compact single-line format
//!
//! ## Color System
//!
//! Professional hierarchical coloring:
//! - **Level 1 Headers**: Bright Blue (bold + italic)
//! - **Level 2 Headers**: Bright Cyan (bold + italic)  
//! - **Level 3 Headers**: Bright Magenta (bold + italic)
//! - **Row Labels**: Bright White (bold + italic)
//! - **Data Cells**: Standard White (easy on eyes)
//!
//! ## Compatibility
//!
//! - Rust 1.70+ required for procedural macro features
//! - Cross-platform ANSI color support
//! - Automatic fallback on unsupported terminals

use proc_macro::TokenStream;

mod colorstyle;
mod extensions;
mod formatext;
mod println;
mod input;

#[proc_macro]
pub fn println(input: TokenStream) -> TokenStream {
    println::println_impl(input)
}

#[proc_macro]
pub fn input(input: TokenStream) -> TokenStream {
    input::input_impl(input)
}