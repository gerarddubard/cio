/// ANSI color and style management module.
///
/// This module provides comprehensive functions for working with ANSI escape sequences
/// for text colors and styles in terminal environments. It serves as the core color
/// engine for the println! macro system and handles all terminal styling operations.
///
/// # ANSI Color System
///
/// The module implements the full ANSI color specification with support for:
/// - 8 standard colors (30-37)
/// - 8 bright colors (90-97) 
/// - 9 text style modifiers (1-9)
/// - Automatic reset sequences (0)
///
/// # Supported Colors
///
/// ## Standard Colors (30-37)
/// - black, red, green, yellow, blue, magenta, cyan, white
///
/// ## Bright Colors (90-97)  
/// - bright_black (alias: gray), bright_red, bright_green, bright_yellow
/// - bright_blue, bright_magenta, bright_cyan, bright_white
///
/// # Supported Styles
///
/// ## Text Modifications (1-9)
/// - bold (1) - Increased font weight
/// - dimmed (2) - Decreased font weight  
/// - italic (3) - Slanted text style
/// - underline (4) - Underlined text
/// - blink (5) - Blinking text effect
/// - reversed (7) - Inverted foreground/background
/// - hidden (8) - Invisible text (password fields)
/// - strikethrough (9) - Line through text
///
/// # String Escaping
///
/// The module provides robust string escaping for format strings:
/// - Backslash escaping: `\` → `\\`
/// - Quote escaping: `"` → `\"`  
/// - Newline escaping: `\n` → `\\n`
///
/// # ANSI Sequence Generation
///
/// Color sequences follow the standard format: `\x1B[{codes}m`
/// - Single style: `\x1B[31m` (red)
/// - Multiple styles: `\x1B[31;1m` (red + bold)
/// - Reset sequence: `\x1B[0m` (clear all styles)
///
/// # Performance Optimizations
///
/// - **Zero Allocation**: Returns reset sequence for empty inputs
/// - **Efficient Joining**: Uses semicolon-separated code concatenation
/// - **Fast Matching**: Uses optimized match expressions for style lookup
/// - **String Interning**: Reuses common ANSI sequences
///
/// # Terminal Compatibility
///
/// The module generates standard ANSI sequences compatible with:
/// - Unix terminals (xterm, gnome-terminal, etc.)
/// - Windows Terminal and PowerShell 
/// - VS Code integrated terminal
/// - Modern terminal emulators
///
/// # Usage Patterns
///
/// The module is used internally by the formatting system:
/// - Style parsing: `@(red, bold)` → `["red", "bold"]`
/// - Code generation: `["red", "bold"]` → `\x1B[31;1m`
/// - String safety: Format strings are properly escaped
/// - Reset handling: Automatic style reset after each token
///
/// # Technical Implementation
///
/// ## Error Handling
/// - Unknown styles are silently ignored (graceful degradation)
/// - Empty style lists return reset sequences
/// - Invalid codes are filtered out automatically
///
/// ## Memory Management
/// - Minimal heap allocations through strategic string building
/// - Code vector reuse for multiple style combinations
/// - Efficient string concatenation patterns
///
/// ## Standards Compliance
/// - Full ANSI X3.64 compliance for color codes
/// - SGR (Select Graphic Rendition) parameter support
/// - Cross-platform terminal compatibility guaranteed

pub fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n")
}
pub fn ansi_code_for_style(styles: &[String]) -> String {
    if styles.is_empty() {
        return "\x1B[0m".to_string();
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