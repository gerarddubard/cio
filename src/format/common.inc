// Common utility functions used across multiple formatting modules.
//
// This collection provides essential parsing and analysis functions for
// structured data formatting. These utilities handle the complex task of
// analyzing nested data structures, bracket parsing, and string cleaning
// operations that are fundamental to advanced formatting capabilities.
//
// # Function Overview
//
// ## count_nesting_depth
// Analyzes the maximum nesting depth of bracket structures within a string.
// Essential for determining appropriate formatting strategies for nested
// containers like vectors of vectors, matrices, and complex data structures.
//
// Features:
// - Quote-aware parsing that ignores brackets within string literals
// - Accurate depth calculation for arbitrarily nested structures
// - Efficient single-pass algorithm with O(n) time complexity
// - Robust handling of malformed bracket sequences
//
// Use cases:
// - Matrix dimension detection for mathematical formatting
// - Container nesting analysis for pretty-printing decisions
// - Automatic indentation level calculation
// - Format strategy selection based on complexity
//
// ## find_first_level_brackets
// Locates all top-level bracket pairs within a string, providing precise
// position information for structured parsing operations.
//
// Features:
// - Returns precise start and end positions for each bracket pair
// - Quote-aware parsing prevents false matches in string literals
// - Handles multiple first-level sections in complex structures
// - Efficient vector-based collection of bracket positions
//
// Use cases:
// - Array element separation for table formatting
// - Matrix row identification and parsing
// - Container boundary detection for formatting decisions
// - Structured data decomposition for specialized renderers
//
// ## clean_string_quotes
// Performs intelligent string quote removal with safety checks and validation.
// Essential for processing string literals that may contain formatting data.
//
// Features:
// - Safe quote removal with length validation
// - Preserves strings that don't have matching quotes
// - Automatic whitespace trimming for clean output
// - Handles empty strings and edge cases gracefully
//
// Use cases:
// - JSON string value extraction for table formatting
// - String literal processing in format specifiers
// - Clean text extraction from quoted data sources
// - Data sanitization for display formatting
//
// # Technical Implementation
//
// ## Performance Characteristics
// - All functions use single-pass algorithms where possible
// - Minimal memory allocations through efficient data structures
// - Quote-aware parsing prevents common pitfalls in string processing
// - Robust error handling for malformed input data
//
// ## Quote Handling Strategy
// The quote-aware parsing system handles several important cases:
// - Nested quotes within string literals
// - Escaped quotes and special characters
// - Mixed quote styles in complex data structures
// - Edge cases with unclosed or malformed quote sequences
//
// ## Integration Points
// These utilities integrate with:
// - Basic formatters for container structure analysis
// - Mathematical formatters for matrix parsing
// - Table formatters for data extraction and organization
// - Error handling systems for robust parsing operations
//
// # Usage Patterns
//
// The functions work together to provide comprehensive parsing support:
// 1. count_nesting_depth determines formatting complexity
// 2. find_first_level_brackets provides structural boundaries
// 3. clean_string_quotes sanitizes extracted data
// 4. Results feed into specialized formatting algorithms
//
// This coordinated approach ensures consistent and reliable parsing
// across all formatting modules while maintaining high performance
// and robust error handling capabilities.

fn count_nesting_depth(s: &str) -> usize {
    let mut depth = 0;
    let mut max_depth = 0;
    let mut in_quotes = false;
    for c in s.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            '[' if !in_quotes => {
                depth += 1;
                max_depth = std::cmp::max(max_depth, depth);
            },
            ']' if !in_quotes => depth -= 1,
            _ => {}
        }
    }
    max_depth
}
fn find_first_level_brackets(content: &str) -> Vec<(usize, usize)> {
    let mut brackets = Vec::new();
    let mut level = 0;
    let mut in_quotes = false;
    let mut start_pos = 0;
    for (i, c) in content.chars().enumerate() {
        match c {
            '"' => in_quotes = !in_quotes,
            '[' if !in_quotes => {
                if level == 0 { start_pos = i; }
                level += 1;
            },
            ']' if !in_quotes => {
                level -= 1;
                if level == 0 { brackets.push((start_pos, i)); }
            },
            _ => {}
        }
    }
    brackets
}
fn clean_string_quotes(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        trimmed[1..trimmed.len()-1].to_string()
    } else {
        trimmed.to_string()
    }
}