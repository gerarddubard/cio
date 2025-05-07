// formatext.rs
use crate::colorstyle;
use crate::extensions;
use regex::Regex;

/// Format string processing module for the println! macro.
///
/// This module handles the parsing and generation of formatted output
/// with support for ANSI colors, custom formatting, and variable interpolation.
/// It's the core engine behind the enhanced formatting capabilities.

#[derive(Clone, Debug)]
pub enum FormatToken {
    StyleChange { style_specs: Vec<String> },
    StyleReset,
    Text { content: String },
    Variable {
        name: String,
        format: Option<String>,
    },
    StyleVariable {
        name: String,
    },
}

// List of known colors and styles
const KNOWN_COLORS: [&str; 17] = [
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright_black", "gray", "bright_red", "bright_green", "bright_yellow",
    "bright_blue", "bright_magenta", "bright_cyan", "bright_white"
];

const KNOWN_STYLES: [&str; 8] = [
    "bold", "italic", "underline", "dimmed", "blink", "reversed", "hidden", "strikethrough"
];

// Function to check if a term is a known color or style
fn is_known_term(term: &str) -> bool {
    let trimmed = term.trim();
    KNOWN_COLORS.contains(&trimmed) || KNOWN_STYLES.contains(&trimmed)
}

// Function to check if an expression is a list of known styles/colors
fn is_style_list(expr: &str) -> bool {
    if expr.is_empty() {
        return false;
    }

    // If at least one term is known, consider it a style list
    expr.split(',')
        .map(|s| s.trim())
        .any(|term| is_known_term(term))
}

pub fn parse_format_string(fmt_str: &str) -> (Vec<FormatToken>, Vec<String>) {
    let mut tokens = Vec::new();
    let mut used_vars = Vec::new();

    // Single regex to capture all @(...)
    let style_pattern = Regex::new(r"@\(([^)]*)\)").unwrap();

    // Regex for variables in braces
    let var_pattern = Regex::new(r"\{([^{}]+?)(?::([^{}]+))?}").unwrap();

    // Collect all @(...) styles
    let mut style_matches = Vec::new();
    for cap in style_pattern.captures_iter(fmt_str) {
        let whole_match = cap.get(0).unwrap();
        let content = cap.get(1).unwrap().as_str();

        // Determine if it's a direct style, variable, or interpolated variable
        let is_var_interpolated = content.starts_with('{') && content.ends_with('}');
        let is_style_term = is_style_list(content);
        let is_var = !is_style_term && !is_var_interpolated;

        if is_var_interpolated {
            // Extract variable name between braces
            let var_name = &content[1..content.len()-1];
            used_vars.push(var_name.to_string());
            // Special interpolation in a style
            let style_var = format!("{{{}}}", var_name);
            style_matches.push((whole_match.start(), whole_match.end(), style_var, true));
        } else {
            style_matches.push((whole_match.start(), whole_match.end(), content.to_string(), is_var));
        }
    }

    // Collect variables between braces
    let mut var_matches = Vec::new();
    for cap in var_pattern.captures_iter(fmt_str) {
        let whole_match = cap.get(0).unwrap();
        let var_expr = cap.get(1).unwrap().as_str();
        let format_spec = cap.get(2).map(|m| m.as_str().to_string());

        // Check if this variable is already counted as a style variable
        let is_style_var = style_matches.iter()
            .any(|(_, _, content, is_var)| *is_var && content == &format!("{{{}}}", var_expr));

        // If not already a style variable, add as a normal variable
        if !is_style_var {
            // Add to used variables list
            if !var_expr.contains(" ") && !var_expr.contains("*") && !var_expr.contains("+")
                && !var_expr.contains("-") && !var_expr.contains("/") && !var_expr.contains(".") {
                used_vars.push(var_expr.to_string());
            }

            var_matches.push((whole_match.start(), whole_match.end(),
                              var_expr.to_string(), format_spec));
        }
    }

    let mut all_matches = Vec::new();

    // Process style tags (with and without variables)
    for (start, end, content, is_var) in style_matches {
        if content.is_empty() {
            all_matches.push((start, end, FormatToken::StyleReset));
        } else if is_var {
            // It's a style variable (like @(color_style)) or interpolation (@({color}))
            let var_name = content.trim().to_string();
            all_matches.push((start, end, FormatToken::StyleVariable { name: var_name }));
        } else {
            // It's a direct specification (like @(green, bold))
            let specs: Vec<String> = content
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            all_matches.push((start, end, FormatToken::StyleChange { style_specs: specs }));
        }
    }

    // Process normal variables {var}
    for (start, end, expr, format) in var_matches {
        all_matches.push((start, end, FormatToken::Variable {
            name: expr,
            format
        }));
    }

    // Sort all matches by position
    all_matches.sort_by_key(|m| m.0);

    // Build text tokens between matches
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

    // Add remaining text
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
                // For style variables, create code to process the variable
                segments.push(format!(
                    "let style_specs = {}.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();",
                    name
                ));
                segments.push("let ansi = crate::colorstyle::ansi_code_for_style(&style_specs);".to_string());
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
            FormatToken::Variable { name, format } => {
                let format_code = match format {
                    // Special formats for a, c, j, m, d
                    Some(fmt) if fmt == "a" => format!("format_container(&{})", name),
                    Some(fmt) if fmt == "c" => format!("format!(\"{{:?}}\", {})", name),
                    Some(fmt) if fmt == "j" => format!("format!(\"{{:#?}}\", {})", name),
                    Some(fmt) if fmt == "m" => format!("format_matrix(&{})", name),
                    Some(fmt) if fmt == "d" => format!("format_determinant(&{})", name),
                    Some(fmt) => format!("format!(\"{{:{}}}\", {})", fmt, name),
                    None => format!("format!(\"{{}}\", {})", name),
                };
                segments.push(format!("result.push_str(&({}));", format_code));
            },
        }
    }

    // Reset styles at the end
    segments.push("result.push_str(\"\\x1B[0m\");".to_string());

    // Generate display code with or without newline
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