// colorstyle.rs

/// ANSI color and style handling module.
///
/// This module provides functions to work with ANSI escape sequences
/// for terminal colors and text styles. It's used by the println! macro
/// to provide colored and styled text output.
///
/// Supported colors:
/// - Basic: black, red, green, yellow, blue, magenta, cyan, white
/// - Bright: bright_black/gray, bright_red, bright_green, bright_yellow,
///   bright_blue, bright_magenta, bright_cyan, bright_white
///
/// Supported styles:
/// - bold, italic, underline, dimmed, blink, reversed, hidden, strikethrough

pub fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n")
}

pub fn ansi_code_for_style(styles: &[String]) -> String {
    if styles.is_empty() {
        return "\x1B[0m".to_string(); // Reset
    }
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
    if codes.is_empty() {
        return "\x1B[0m".to_string();
    }
    format!("\x1B[{}m", codes.join(";"))
}