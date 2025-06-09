//! Rust to Mojo Compiler Library
//!
//! This library provides functionality to parse Rust source code and transpile it to Mojo.
//! It supports both programmatic usage as a library and command-line interface usage.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use rust2mojo::Compiler;
//!
//! let compiler = Compiler::new();
//! let rust_code = r#"
//!     fn hello_world() {
//!         println!("Hello, world!");
//!     }
//! "#;
//!
//! match compiler.compile_str(rust_code) {
//!     Ok(mojo_code) => println!("Generated Mojo:\n{}", mojo_code),
//!     Err(e) => eprintln!("Compilation failed: {}", e),
//! }
//! ```
//!
//! Compiling from a file:
//!
//! ```no_run
//! use rust2mojo::Compiler;
//! use std::path::Path;
//!
//! let compiler = Compiler::new();
//! let input_path = Path::new("example.rs");
//!
//! match compiler.compile_file(input_path) {
//!     Ok(mojo_code) => {
//!         // Save to output file or process further
//!         std::fs::write("example.mojo", mojo_code).unwrap();
//!     },
//!     Err(e) => eprintln!("Failed to compile {}: {}", input_path.display(), e),
//! }
//! ```

pub mod ast;
pub mod codegen;
pub mod comparison;
pub mod error;
pub mod parser;

pub use error::{Error, Result};

/// Main compiler interface for converting Rust code to Mojo
///
/// The `Compiler` struct provides the primary interface for transpiling Rust source code
/// into equivalent Mojo code. It handles the full compilation pipeline from parsing
/// Rust syntax to generating idiomatic Mojo output.
///
/// # Examples
///
/// ```
/// use rust2mojo::Compiler;
///
/// let compiler = Compiler::new();
/// let result = compiler.compile_str("fn add(a: i32, b: i32) -> i32 { a + b }");
/// assert!(result.is_ok());
/// ```
pub struct Compiler {
    // Configuration options will be added here
}

impl Compiler {
    /// Create a new compiler instance with default configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2mojo::Compiler;
    ///
    /// let compiler = Compiler::new();
    /// // Compiler is ready to use
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Compile a Rust source string to Mojo code
    ///
    /// Takes a string containing valid Rust source code and returns the equivalent
    /// Mojo code as a string. This is the primary method for programmatic compilation.
    ///
    /// # Arguments
    ///
    /// * `rust_code` - A string slice containing valid Rust source code
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Generated Mojo source code on successful compilation
    /// * `Err(Error)` - Compilation error with details about what went wrong
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2mojo::Compiler;
    ///
    /// let compiler = Compiler::new();
    /// let rust_code = "fn greet(name: &str) { println!(\"Hello, {}!\", name); }";
    ///
    /// match compiler.compile_str(rust_code) {
    ///     Ok(mojo_code) => {
    ///         assert!(mojo_code.contains("fn greet"));
    ///         assert!(mojo_code.contains("# Generated Mojo code"));
    ///     },
    ///     Err(e) => panic!("Compilation failed: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The input Rust code has syntax errors
    /// - The Rust code contains unsupported language features
    /// - Internal compilation errors occur
    pub fn compile_str(&self, rust_code: &str) -> Result<String> {
        let ast = parser::parse_rust_code(rust_code)?;
        let mojo_code = codegen::generate_mojo(&ast)?;
        Ok(mojo_code)
    }

    /// Compile a Rust source file to Mojo code
    ///
    /// Reads a Rust source file from disk and compiles it to Mojo code.
    /// This is a convenience method that combines file I/O with compilation.
    ///
    /// # Arguments
    ///
    /// * `input_path` - Path to the Rust source file to compile
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Generated Mojo source code on successful compilation
    /// * `Err(Error)` - I/O or compilation error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust2mojo::Compiler;
    /// use std::path::Path;
    ///
    /// let compiler = Compiler::new();
    /// let input_path = Path::new("src/main.rs");
    ///
    /// match compiler.compile_file(input_path) {
    ///     Ok(mojo_code) => {
    ///         // Write to output file
    ///         std::fs::write("src/main.mojo", mojo_code).unwrap();
    ///         println!("Compilation successful!");
    ///     },
    ///     Err(e) => eprintln!("Failed to compile {}: {}", input_path.display(), e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The input file cannot be read (doesn't exist, permission denied, etc.)
    /// - The file contains invalid Rust syntax
    /// - The compilation process fails for any reason
    pub fn compile_file(&self, input_path: &std::path::Path) -> Result<String> {
        let rust_code = std::fs::read_to_string(input_path)
            .map_err(|e| Error::IoError(format!("Failed to read input file: {}", e)))?;
        self.compile_str(&rust_code)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_creation() {
        let compiler = Compiler::new();
        let default_compiler = Compiler::default();

        // Just ensure they can be created without panicking
        assert_eq!(
            std::mem::size_of_val(&compiler),
            std::mem::size_of_val(&default_compiler)
        );
    }

    #[test]
    fn test_simple_function() {
        let compiler = Compiler::new();
        let rust_code = "fn test() {}";
        let result = compiler.compile_str(rust_code);
        assert!(result.is_ok());
    }
}
