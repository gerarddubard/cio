/// Formatting coordination module for the PyRust library.
///
/// This module serves as the central orchestrator for all formatting operations,
/// delegating to specialized formatting modules and coordinating the global
/// formatting system. It acts as a bridge between the macro system and the
/// actual formatting implementations.
///
/// # Architecture
///
/// The module follows a modular architecture where each formatting category
/// is handled by dedicated sub-modules:
///
/// - **Common Functions**: Shared utilities used across all formatters
/// - **Basic Formatting**: Standard container and structure formatting 
/// - **Mathematical Formatting**: Matrix, determinant, and mathematical notation
/// - **Table Formatting**: Advanced table rendering with hierarchical colors
///
/// # Module Organization
///
/// Each specialized formatter is implemented as an `.inc` file:
/// - `format/common.inc` - Shared helper functions and utilities
/// - `format/basic.inc` - Basic formatters (`:a`, `:c`, `:j`)
/// - `format/math.inc` - Mathematical formatters (`:m`, `:d`)
/// - `format/table.inc` - Table formatters (`:t`, `:t(headers)`)
///
/// # Design Philosophy
///
/// The extension system is designed with several key principles:
///
/// ## Modularity
/// Each formatting category is self-contained, allowing for easy maintenance
/// and testing of individual components without affecting others.
///
/// ## Performance
/// All helper functions are included at compile-time using `include_str!`,
/// eliminating runtime overhead and ensuring optimal performance.
///
/// ## Extensibility  
/// New formatting capabilities can be added by creating new `.inc` files
/// and registering them in the coordination function.
///
/// ## Type Safety
/// All formatters work with Rust's type system, providing compile-time
/// guarantees about formatting operations.
///
/// # Usage Pattern
///
/// The module is used internally by the `formatext` module:
/// 1. Format tokens are parsed and classified
/// 2. Appropriate formatters are selected based on format specifiers
/// 3. Helper functions are injected into the generated code
/// 4. Final output is assembled and rendered
///
/// # Integration Points
///
/// The module integrates with:
/// - **Macro System**: Provides functions for code generation
/// - **Color System**: Coordinates with ANSI color formatting
/// - **Extension Points**: Allows for future formatting additions
/// - **Error Handling**: Provides consistent error reporting across formatters
///
/// # Performance Characteristics
///
/// - **Zero Runtime Cost**: All functions included at compile-time
/// - **Minimal Allocations**: Efficient string building strategies
/// - **Type Erasure**: Generic implementations for maximum flexibility
/// - **SIMD Friendly**: Designed for potential vectorization optimizations
///
/// # Future Extensions
///
/// The architecture supports additional formatting categories:
/// - Custom user-defined formatters
/// - Domain-specific formatting (dates, currencies, etc.)
/// - Interactive formatting with user input
/// - Export formatters (CSV, JSON, XML output)

pub fn get_helper_functions() -> &'static str {
    concat!(
    include_str!("format/common.inc"),
    include_str!("format/basic.inc"),
    include_str!("format/math.inc"),
    include_str!("format/table.inc")
    )
}