// extensions.rs

/// Extension module providing helper functions for advanced formatting.
///
/// This module contains utility functions for formatting container types
/// like arrays, matrices, and multidimensional data structures.
/// It provides the implementation for format specifiers :a, :m, and :d.

pub fn get_helper_functions() -> &'static str {
    r#"
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
            fn format_container<T: std::fmt::Debug + Sized>(value: &T) -> String {
                let debug_str = format!("{:?}", value);
                if debug_str.starts_with('[') {
                    let depth = count_nesting_depth(&debug_str);
                    match depth {
                        0 | 1 => debug_str, // 1D array or non-array
                        2 => format_2d_array(&debug_str),
                        _ => format_nd_array(&debug_str),
                    }
                } else {
                    debug_str
                }
            }
            fn format_2d_array(debug_str: &str) -> String {
                debug_str
                    .replace("[[", "[\n  [")
                    .replace("]]", "]\n]")
                    .replace("], [", "],\n  [")
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
                            if level == 0 {
                                start_pos = i;
                            }
                            level += 1;
                        },
                        ']' if !in_quotes => {
                            level -= 1;
                            if level == 0 {
                                brackets.push((start_pos, i));
                            }
                        },
                        _ => {}
                    }
                }
                brackets
            }
            fn format_nd_array(debug_str: &str) -> String {
                let content = &debug_str[1..debug_str.len()-1];
                let brackets = find_first_level_brackets(content);
                let mut result = String::from("[\n");
                for (i, (start, end)) in brackets.iter().enumerate() {
                    let sub_array = &content[*start..*end+1];
                    result.push_str("  ");
                    let formatted = format_sub_array(sub_array, 1);
                    result.push_str(&formatted);
                    if i < brackets.len() - 1 {
                        result.push_str(",\n");
                    }
                }
                result.push_str("\n]");
                result
            }
            fn format_sub_array(sub_array: &str, indent_level: usize) -> String {
                if !sub_array.contains("[[") {
                    return sub_array.to_string();
                }
                let content = &sub_array[1..sub_array.len()-1];
                let brackets = find_first_level_brackets(content);
                let mut result = String::from("[\n");
                for (i, (start, end)) in brackets.iter().enumerate() {
                    let sub_sub_array = &content[*start..*end+1];
                    result.push_str(&"  ".repeat(indent_level + 1));
                    let formatted = format_sub_array(sub_sub_array, indent_level + 1);
                    result.push_str(&formatted);
                    if i < brackets.len() - 1 {
                        result.push_str(",\n");
                    }
                }
                result.push_str(&format!("\n{}", "  ".repeat(indent_level)));
                result.push_str("]");
                result
            }
            fn parse_2d_array<T: std::fmt::Debug + Sized>(value: &T) -> Option<Vec<Vec<String>>> {
                let debug_str = format!("{:?}", value);
                if !debug_str.starts_with('[') || !debug_str.ends_with(']') {
                    return None;
                }
                let content = &debug_str[1..debug_str.len()-1];
                let mut result = Vec::new();
                let mut level = 0;
                let mut in_quotes = false;
                let mut row_start = 0;
                let mut rows = Vec::new();
                for (i, c) in content.chars().enumerate() {
                    match c {
                        '"' => in_quotes = !in_quotes,
                        '[' if !in_quotes => {
                            if level == 0 {
                                row_start = i;
                            }
                            level += 1;
                        },
                        ']' if !in_quotes => {
                            level -= 1;
                            if level == 0 {
                                rows.push(&content[row_start..=i]);
                            }
                        },
                        _ => {}
                    }
                }
                for row_str in rows {
                    let row_content = &row_str[1..row_str.len()-1];
                    let mut row = Vec::new();
                    let mut element = String::new();
                    let mut in_element_quotes = false;
                    let mut bracket_level = 0;
                    for c in row_content.chars() {
                        match c {
                            '"' => {
                                in_element_quotes = !in_element_quotes;
                                element.push(c);
                            },
                            '[' if !in_element_quotes => {
                                bracket_level += 1;
                                element.push(c);
                            },
                            ']' if !in_element_quotes => {
                                bracket_level -= 1;
                                element.push(c);
                            },
                            ',' if !in_element_quotes && bracket_level == 0 => {
                                row.push(element.trim().to_string());
                                element = String::new();
                            },
                            _ => {
                                element.push(c);
                            }
                        }
                    }
                    if !element.trim().is_empty() {
                        row.push(element.trim().to_string());
                    }
                    result.push(row);
                }
                Some(result)
            }
            fn format_matrix<T: std::fmt::Debug + Sized>(value: &T) -> String {
                if let Some(matrix) = parse_2d_array(value) {
                    let nrows = matrix.len();
                    if nrows == 0 {
                        return "[Empty Matrix]".to_string();
                    }
                    let ncols = matrix.get(0).map_or(0, |row| row.len());
                    let mut col_widths = vec![0; ncols];
                    for row in &matrix {
                        for (j, val) in row.iter().enumerate() {
                            if j < ncols {
                                col_widths[j] = col_widths[j].max(val.len());
                            }
                        }
                    }
                    let mut result = String::new();
                    for (i, row) in matrix.iter().enumerate() {
                        let (left, right) = match (nrows, i) {
                            (1, _) => ("⦅", "⦆"),
                            (_, 0) => ("⎛", "⎞"),
                            (_, x) if x == nrows - 1 => ("⎝", "⎠"),
                            _ => ("│", "│"),
                        };
                        result.push_str(left);
                        result.push_str("  ");
                        for (j, val) in row.iter().enumerate() {
                            result.push_str(&format!("{:<width$}", val, width = col_widths[j]));
                            if j < ncols - 1 {
                                result.push_str("  "); // Space between columns
                            }
                        }
                        result.push_str(&format!("  {}\n", right));
                    }
                    result
                } else {
                    // Fallback to standard format
                    format!("{:?}", value)
                }
            }
            fn format_determinant<T: std::fmt::Debug + Sized>(value: &T) -> String {
                if let Some(matrix) = parse_2d_array(value) {
                    let nrows = matrix.len();
                    let ncols = matrix.get(0).map_or(0, |row| row.len());
                    if nrows != ncols || nrows < 2 {
                        return "Determinant undefined (non-square or too small matrix)".to_string();
                    }
                    let mut col_widths = vec![0; ncols];
                    for row in &matrix {
                        for (j, val) in row.iter().enumerate() {
                            if j < ncols {
                                col_widths[j] = col_widths[j].max(val.len());
                            }
                        }
                    }
                    let mut result = String::new();
                    for (i, row) in matrix.iter().enumerate() {
                        let left = "│";
                        let right = "│";
                        result.push_str(left);
                        result.push_str("  "); // Space before first column
                        for (j, val) in row.iter().enumerate() {
                            result.push_str(&format!("{:<width$}", val, width = col_widths[j]));
                            if j < ncols - 1 {
                                result.push_str("  "); // Space between columns
                            }
                        }
                        result.push_str(&format!("  {}\n", right));
                    }
                    result
                } else {
                    "Determinant undefined (invalid matrix)".to_string()
                }
            }
    "#
}