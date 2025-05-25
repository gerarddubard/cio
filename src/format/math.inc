// Mathematical formatting functions for matrices and determinants.
//
// This module provides sophisticated mathematical notation formatting using
// Unicode mathematical symbols and proper spacing algorithms. It handles
// matrix display with bracket styles and determinant notation with vertical
// bars, ensuring professional mathematical presentation.
//
// # Mathematical Notation Support
//
// ## Matrix Formatting (`:m`)
// Renders matrices with appropriate mathematical brackets using Unicode symbols:
// - **Single Row**: Rounded brackets `⦅ ... ⦆`
// - **Top Row**: Upper rounded brackets `⎛ ... ⎞`
// - **Middle Rows**: Vertical bars `│ ... │`
// - **Bottom Row**: Lower rounded brackets `⎝ ... ⎠`
//
// ## Determinant Formatting (`:d`)
// Renders determinants with vertical bar notation `│ ... │` for all rows,
// following standard mathematical conventions for determinant display.
//
// # Core Architecture
//
// ## Data Extraction Pipeline
// The formatting system uses a multi-stage extraction process:
// 1. **Debug String Generation**: Convert input to Debug representation
// 2. **Structure Analysis**: Detect 1D vs 2D array patterns
// 3. **Element Extraction**: Parse individual elements with quote handling
// 4. **Data Cleaning**: Remove formatting artifacts and normalize strings
//
// ## Layout Algorithm
// The formatting engine uses sophisticated layout calculations:
// 1. **Column Width Analysis**: Calculate maximum width per column
// 2. **Visual Width Calculation**: Handle Unicode characters properly
// 3. **Alignment Strategy**: Right-pad elements for perfect alignment
// 4. **Symbol Placement**: Add appropriate mathematical brackets/bars
//
// # Function Specifications
//
// ## extract_2d_array
// Primary data extraction function that converts Debug output into structured
// 2D string arrays suitable for mathematical formatting.
//
// **Algorithm Steps**:
// 1. Parse Debug string representation of input value
// 2. Detect array dimension (1D arrays become single-row 2D)
// 3. Use bracket parsing to identify matrix rows
// 4. Extract individual elements with proper quote cleaning
// 5. Return structured Vec<Vec<String>> representation
//
// **Edge Case Handling**:
// - Empty arrays return empty Vec for consistent error handling
// - 1D arrays are automatically promoted to single-row matrices
// - Malformed input falls back gracefully without panicking
// - String quotes are intelligently cleaned while preserving data
//
// ## extract_1d_array
// Specialized parser for single-dimensional array strings with robust
// comma-separated value extraction and nested structure awareness.
//
// **Parsing Features**:
// - Quote-aware comma splitting prevents false separations
// - Bracket-level tracking handles nested array elements
// - Capacity pre-allocation based on comma counting optimization
// - Whitespace trimming ensures clean element extraction
//
// **Performance Optimizations**:
// - Single-pass parsing with state machine approach
// - Pre-allocated Vec capacity based on comma estimation
// - Efficient string building with minimal reallocations
// - Character-by-character processing for maximum control
//
// ## get_visual_width
// Unicode-aware width calculation function that properly handles multi-byte
// characters and mathematical symbols for accurate alignment.
//
// **Technical Details**:
// - Uses char count rather than byte length for Unicode safety
// - Handles mathematical symbols and special characters correctly
// - Provides consistent width calculation across different character sets
// - Essential for proper column alignment in formatted output
//
// ## format_matrix
// Advanced matrix formatter that creates professional mathematical notation
// with proper bracket symbols and column alignment.
//
// **Formatting Algorithm**:
// 1. Extract 2D data array from input value
// 2. Calculate optimal column widths for alignment
// 3. Determine appropriate bracket symbols based on row position
// 4. Generate formatted output with proper spacing and symbols
// 5. Return complete matrix representation with Unicode brackets
//
// **Symbol Selection Logic**:
// ```
// Single Row:    ⦅  1  2  3  ⦆
// Multi-Row:     ⎛  1  2  3  ⎞
//                │  4  5  6  │
//                ⎝  7  8  9  ⎠
// ```
//
// **Performance Characteristics**:
// - O(n·m) complexity where n=rows, m=columns
// - Memory pre-allocation based on estimated output size
// - Minimal string reallocations through capacity management
// - Efficient column width calculation with single pass
//
// ## format_determinant
// Specialized determinant formatter using vertical bar notation with
// validation for square matrices and proper mathematical presentation.
//
// **Validation Rules**:
// - Matrix must be square (n×n) for valid determinant
// - Minimum size of 2×2 required for meaningful determinants
// - Non-square matrices return descriptive error messages
// - Empty matrices handled gracefully with appropriate messages
//
// **Formatting Output**:
// ```
// │  1  2  │
// │  3  4  │
// ```
//
// # Mathematical Symbol Set
//
// ## Matrix Brackets
// - `⦅` `⦆` - Single row rounded brackets
// - `⎛` `⎞` - Top row curved brackets
// - `│` `│` - Middle row vertical bars
// - `⎝` `⎠` - Bottom row curved brackets
//
// ## Alignment Characters
// - Space padding for right-alignment within columns
// - Double spaces between columns for visual separation
// - Consistent spacing around mathematical symbols
//
// # Error Handling Strategy
//
// ## Graceful Degradation
// The module handles various error conditions without panicking:
// - **Empty Input**: Returns descriptive empty matrix messages
// - **Malformed Data**: Falls back to safe string representations
// - **Invalid Dimensions**: Provides clear validation error messages
// - **Parse Failures**: Returns original Debug output when possible
//
// ## Validation Messages
// - "Determinant undefined (empty matrix)" for empty inputs
// - "Determinant undefined (non-square or too small matrix)" for invalid dimensions
// - "[Empty Matrix]" for empty but valid matrix structures
//
// # Performance Optimizations
//
// ## Memory Management
// - **Capacity Pre-allocation**: Estimates final string size to minimize reallocations
// - **Vec Pre-sizing**: Allocates collection capacity based on element counting
// - **String Reuse**: Efficient string building patterns throughout
// - **Minimal Copying**: References and slices used where possible
//
// ## Algorithmic Efficiency
// - **Single-Pass Parsing**: Most operations complete in one iteration
// - **Early Termination**: Invalid inputs detected quickly without full processing
// - **Optimized Width Calculation**: Column widths calculated efficiently
// - **State Machine Parsing**: Character processing uses efficient state tracking
//
// # Integration with Format System
//
// ## Format Specifier Integration
// - `:m` specifier triggers `format_matrix()` function
// - `:d` specifier triggers `format_determinant()` function
// - Both integrate seamlessly with the macro code generation system
// - Color and style formatting can be applied to mathematical output
//
// ## Type System Compatibility
// Works with any type implementing `std::fmt::Debug`:
// - **Vec<Vec<T>>**: Multi-dimensional vectors
// - **Arrays**: Fixed-size mathematical arrays
// - **Custom Types**: User-defined mathematical structures
// - **Mixed Types**: Heterogeneous numerical data

fn extract_2d_array<T: std::fmt::Debug>(value: &T) -> Vec<Vec<String>> {
    let debug_str = format!("{:?}", value);
    if !debug_str.starts_with('[') || !debug_str.ends_with(']') { return Vec::new(); }
    if !debug_str.contains("[[") {
        let elements = extract_1d_array(&debug_str);
        if !elements.is_empty() { return vec![elements]; }
        return Vec::new();
    }
    let content = &debug_str[1..debug_str.len()-1];
    let brackets = find_first_level_brackets(content);
    let mut result = Vec::with_capacity(brackets.len());
    for (start, end) in brackets {
        let row_str = &content[start..=end];
        let elements = extract_1d_array(row_str);
        if !elements.is_empty() { result.push(elements); }
    }
    for row in &mut result {
        for cell in row.iter_mut() {
            *cell = clean_string_quotes(cell);
        }
    }
    result
}
fn extract_1d_array(array_str: &str) -> Vec<String> {
    if !array_str.starts_with('[') || !array_str.ends_with(']') { return Vec::new(); }
    let content = &array_str[1..array_str.len()-1];
    let estimated_elements = content.chars().filter(|&c| c == ',').count() + 1;
    let mut elements = Vec::with_capacity(estimated_elements);
    let mut current = String::new();
    let mut in_quotes = false;
    let mut bracket_level = 0;
    for c in content.chars() {
        match c {
            '"' => { in_quotes = !in_quotes; current.push(c); },
            '[' if !in_quotes => { bracket_level += 1; current.push(c); },
            ']' if !in_quotes => { bracket_level -= 1; current.push(c); },
            ',' if !in_quotes && bracket_level == 0 => {
                if !current.is_empty() {
                    elements.push(current.trim().to_string());
                    current = String::new();
                }
            },
            _ => current.push(c)
        }
    }
    if !current.trim().is_empty() { elements.push(current.trim().to_string()); }
    elements
}
fn get_visual_width(s: &str) -> usize {
    s.chars().count()
}
fn format_matrix<T: std::fmt::Debug>(value: &T) -> String {
    let data = extract_2d_array(value);
    if data.is_empty() { return "[Empty Matrix]".to_string(); }
    let nrows = data.len();
    let ncols = data.get(0).map_or(0, |row| row.len());
    if ncols == 0 { return "[Empty Matrix]".to_string(); }
    let mut col_widths = vec![0; ncols];
    for row in &data {
        for (j, val) in row.iter().enumerate() {
            if j < ncols { col_widths[j] = col_widths[j].max(get_visual_width(val)); }
        }
    }
    let estimated_size = nrows * (ncols * 8 + 10);
    let mut result = String::with_capacity(estimated_size);
    for (i, row) in data.iter().enumerate() {
        let (left, right) = match (nrows, i) {
            (1, _) => ("⦅", "⦆"),
            (_, 0) => ("⎛", "⎞"),
            (_, x) if x == nrows - 1 => ("⎝", "⎠"),
            _ => ("│", "│"),
        };
        result.push_str(left);
        result.push_str("  ");
        for (j, val) in row.iter().enumerate() {
            if j < ncols {
                let val_width = get_visual_width(val);
                let padding = col_widths[j] - val_width;
                result.push_str(val);
                result.push_str(&" ".repeat(padding));
                if j < ncols - 1 { result.push_str("  "); }
            }
        }
        result.push_str("  ");
        result.push_str(right);
        result.push('\n');
    }
    result
}
fn format_determinant<T: std::fmt::Debug>(value: &T) -> String {
    let data = extract_2d_array(value);
    if data.is_empty() { return "Determinant undefined (empty matrix)".to_string(); }
    let nrows = data.len();
    let ncols = data.get(0).map_or(0, |row| row.len());
    if nrows != ncols || nrows < 2 {
        return "Determinant undefined (non-square or too small matrix)".to_string();
    }
    let mut col_widths = vec![0; ncols];
    for row in &data {
        for (j, val) in row.iter().enumerate() {
            if j < ncols { col_widths[j] = col_widths[j].max(get_visual_width(val)); }
        }
    }
    let estimated_size = nrows * (ncols * 8 + 10);
    let mut result = String::with_capacity(estimated_size);
    for row in data.iter() {
        result.push_str("│  ");
        for (j, val) in row.iter().enumerate() {
            if j < ncols {
                let val_width = get_visual_width(val);
                let padding = col_widths[j] - val_width;
                result.push_str(val);
                if padding > 0 { result.push_str(&" ".repeat(padding)); }
                if j < ncols - 1 { result.push_str("  "); }
            }
        }
        result.push_str("  │\n");
    }
    result
}