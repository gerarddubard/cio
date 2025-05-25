/// Format string processing module for the println! macro.
///
/// This module handles the parsing and generation of formatted output
/// with support for ANSI colors, custom formatting, and variable interpolation.
/// It's the core engine behind the enhanced formatting capabilities.
///
/// # Architecture
///
/// The module operates in two main phases:
/// 1. **Parsing**: Converts format strings into structured FormatToken sequences
/// 2. **Generation**: Transforms tokens into executable Rust code for output
///
/// # Token Types
///
/// - **StyleChange**: ANSI color/style modifications like `@(red, bold)`
/// - **StyleReset**: Style reset command `@()`  
/// - **StyleVariable**: Dynamic style from variables `@(color_var)`
/// - **Text**: Plain text content between format specifiers
/// - **Variable**: Data interpolation with optional formatting `{var:format}`
///
/// # Format Specifiers
///
/// ## Basic Formatting
/// - `:a` - Array format with proper indentation for nested structures
/// - `:c` - Compact single-line format using Debug trait
/// - `:j` - JSON-like pretty format with multi-line indentation
///
/// ## Mathematical Formatting  
/// - `:m` - Matrix format with proper borders for 2D arrays
/// - `:d` - Determinant format with vertical bars for mathematical notation
///
/// ## Table Formatting
/// - `:t` - Smart table format with automatic structure detection
/// - `:t(Col1, Col2)` - Table with custom column headers
///
/// # Style Processing
///
/// The module recognizes these color and style terms:
///
/// **Colors**: black, red, green, yellow, blue, magenta, cyan, white
/// **Bright Colors**: bright_red, bright_green, bright_blue, etc.
/// **Styles**: bold, italic, underline, dimmed, blink, reversed, hidden, strikethrough
///
/// # Variable Detection
///
/// Variables are automatically tracked for unused variable suppression:
/// - Simple identifiers: `name`, `age`, `data`
/// - Complex expressions with operators are ignored to prevent false positives
/// - Style variables in `@({var})` syntax are handled separately
///
/// # Code Generation
///
/// The module generates optimized Rust code that:
/// - Minimizes string allocations through result buffer reuse
/// - Handles ANSI escape sequences efficiently
/// - Provides proper error handling for formatting operations
/// - Integrates seamlessly with extension modules for specialized formats
///
/// # Extension Integration
///
/// Format processing delegates to specialized modules:
/// - Basic formats (`:a`, `:c`, `:j`) → extensions module
/// - Mathematical formats (`:m`, `:d`) → math formatting
/// - Table formats (`:t`) → table rendering with color hierarchies
///
/// # Technical Implementation
///
/// - Uses regex-based parsing for robust pattern matching
/// - Maintains style state throughout token processing  
/// - Generates compile-time verified Rust code
/// - Supports both newline and no-newline output modes

use crate::colorstyle;
use crate::extensions;
use regex::Regex;

#[derive(Clone, Debug)]
pub enum FormatToken {
    StyleChange { style_specs: Vec<String> },
    StyleReset,
    Text { content: String },
    Variable {
        name: String,
        format: Option<String>,
        format_args: Option<Vec<String>>,
    },
    StyleVariable {
        name: String,
    },
}
const KNOWN_COLORS: [&str; 17] = [
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright_black", "gray", "bright_red", "bright_green", "bright_yellow",
    "bright_blue", "bright_magenta", "bright_cyan", "bright_white"
];
const KNOWN_STYLES: [&str; 8] = [
    "bold", "italic", "underline", "dimmed", "blink", "reversed", "hidden", "strikethrough"
];
pub const DEFAULT_TABLE_HEADER_COLOR: &str = "bright_blue";
fn is_known_term(term: &str) -> bool {
    let trimmed = term.trim();
    KNOWN_COLORS.contains(&trimmed) || KNOWN_STYLES.contains(&trimmed)
}
fn is_style_list(expr: &str) -> bool {
    if expr.is_empty() {
        return false;
    }
    expr.split(',')
        .map(|s| s.trim())
        .any(|term| is_known_term(term))
}
pub fn parse_format_string(fmt_str: &str) -> (Vec<FormatToken>, Vec<String>) {
    let mut tokens = Vec::new();
    let mut used_vars = Vec::new();
    let style_pattern = Regex::new(r"@\(([^)]*)\)").unwrap();
    let var_pattern = Regex::new(r"\{([^{}]+?)(?::([^{}(]+)(?:\(([^)]*)\))?)?}").unwrap();
    let mut style_matches = Vec::new();
    for cap in style_pattern.captures_iter(fmt_str) {
        let whole_match = cap.get(0).unwrap();
        let content = cap.get(1).unwrap().as_str();
        let is_var_interpolated = content.starts_with('{') && content.ends_with('}');
        let is_style_term = is_style_list(content);
        let is_var = !is_style_term && !is_var_interpolated;
        if is_var_interpolated {
            let var_name = &content[1..content.len()-1];
            used_vars.push(var_name.to_string());
            let style_var = format!("{{{}}}", var_name);
            style_matches.push((whole_match.start(), whole_match.end(), style_var, true));
        } else {
            style_matches.push((whole_match.start(), whole_match.end(), content.to_string(), is_var));
        }
    }
    let mut var_matches = Vec::new();
    for cap in var_pattern.captures_iter(fmt_str) {
        let whole_match = cap.get(0).unwrap();
        let var_expr = cap.get(1).unwrap().as_str();
        let format_spec = cap.get(2).map(|m| m.as_str().to_string());
        let format_args = if let Some(args_str) = cap.get(3) {
            Some(args_str.as_str().split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>())
        } else {
            None
        };
        let is_style_var = style_matches.iter()
            .any(|(_, _, content, is_var)| *is_var && content == &format!("{{{}}}", var_expr));
        if !is_style_var {
            if !var_expr.contains(" ") && !var_expr.contains("*") && !var_expr.contains("+")
                && !var_expr.contains("-") && !var_expr.contains("/") && !var_expr.contains(".") {
                used_vars.push(var_expr.to_string());
            }
            var_matches.push((whole_match.start(), whole_match.end(),
                              var_expr.to_string(), format_spec, format_args));
        }
    }
    let mut all_matches = Vec::new();
    for (start, end, content, is_var) in style_matches {
        if content.is_empty() {
            all_matches.push((start, end, FormatToken::StyleReset));
        } else if is_var {
            let var_name = content.trim().to_string();
            all_matches.push((start, end, FormatToken::StyleVariable { name: var_name }));
        } else {
            let specs: Vec<String> = content
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            all_matches.push((start, end, FormatToken::StyleChange { style_specs: specs }));
        }
    }
    for (start, end, expr, format, format_args) in var_matches {
        all_matches.push((start, end, FormatToken::Variable {
            name: expr,
            format,
            format_args,
        }));
    }
    all_matches.sort_by_key(|m| m.0);
    let mut last_pos = 0;
    for (start, end, token) in all_matches {
        if start > last_pos {
            tokens.push(FormatToken::Text {
                content: fmt_str[last_pos..start].to_string()
            });
        }
        tokens.push(token);
        last_pos = end;
    }
    if last_pos < fmt_str.len() {
        tokens.push(FormatToken::Text {
            content: fmt_str[last_pos..].to_string()
        });
    }
    (tokens, used_vars)
}
pub fn generate_output_code(tokens: &[FormatToken], no_newline: bool) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current_styles = Vec::new();
    for token in tokens {
        match token {
            FormatToken::StyleChange { style_specs } => {
                segments.push("result.push_str(\"\\x1B[0m\");".to_string());
                current_styles = style_specs.clone();
                let ansi = colorstyle::ansi_code_for_style(&current_styles);
                segments.push(format!("result.push_str(\"{}\");", colorstyle::escape_string(&ansi)));
            },
            FormatToken::StyleVariable { name } => {
                segments.push("result.push_str(\"\\x1B[0m\");".to_string());
                segments.push(format!(
                    "let style_specs = {}.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();",
                    name
                ));
                segments.push("let ansi = colorstyle_internal::ansi_code_for_style(&style_specs);".to_string());
                segments.push("result.push_str(&ansi);".to_string());
            },
            FormatToken::StyleReset => {
                current_styles.clear();
                segments.push("result.push_str(\"\\x1B[0m\");".to_string());
            },
            FormatToken::Text { content } => {
                if !content.is_empty() {
                    segments.push(format!("result.push_str(\"{}\");", colorstyle::escape_string(content)));
                }
            },
            FormatToken::Variable { name, format, format_args } => {
                let format_code = match format.as_deref() {
                    Some("a") => format!("format_container(&{})", name),
                    Some("c") => format!("format!(\"{{:?}}\", {})", name),
                    Some("j") => format!("format!(\"{{:#?}}\", {})", name),
                    Some("m") => format!("format_matrix(&{})", name),
                    Some("d") => format!("format_determinant(&{})", name),
                    Some("t") => {
                        if let Some(cols) = format_args {
                            let cols_vec = cols.iter()
                                .map(|s| format!("String::from(\"{}\")", s))
                                .collect::<Vec<_>>()
                                .join(", ");
                            format!("format_table(&{}, &vec![{}], \"{}\")", name, cols_vec, DEFAULT_TABLE_HEADER_COLOR)
                        } else {
                            format!("format_table(&{}, &Vec::<String>::new(), \"{}\")", name, DEFAULT_TABLE_HEADER_COLOR)
                        }
                    },
                    Some(fmt) => format!("format!(\"{{:{}}}\", {})", fmt, name),
                    None => format!("format!(\"{{}}\", {})", name),
                };
                segments.push(format!("result.push_str(&({}));", format_code));
            },
        }
    }
    segments.push("result.push_str(\"\\x1B[0m\");".to_string());
    let print_code = if no_newline {
        "print!(\"{}\", result)".to_string()
    } else {
        "print!(\"{}\n\", result)".to_string()
    };
    segments.push(format!("{}; std::io::stdout().flush().expect(\"Failed to flush stdout\");", print_code));
    segments
}
pub fn get_helper_functions() -> &'static str {
    extensions::get_helper_functions()
}