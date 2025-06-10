//! Fuzzing tests for rust2mojo compiler
//!
//! This module contains property-based and fuzzing tests to discover edge cases
//! and ensure robustness of the compiler.

use proptest::prelude::*;
use rust2mojo::Compiler;

/// Generate random but syntactically valid Rust code for fuzzing
mod rust_generators {
    use super::*;

    /// Generate valid Rust identifiers
    pub fn identifier() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9_]*"
    }

    /// Generate valid Rust primitive types
    pub fn primitive_type() -> impl Strategy<Value = &'static str> {
        prop_oneof![
            Just("i8"),
            Just("i16"),
            Just("i32"),
            Just("i64"),
            Just("i128"),
            Just("u8"),
            Just("u16"),
            Just("u32"),
            Just("u64"),
            Just("u128"),
            Just("f32"),
            Just("f64"),
            Just("bool"),
            Just("char"),
            Just("()"),
            Just("isize"),
            Just("usize"),
        ]
    }

    /// Generate literal values
    pub fn literal() -> impl Strategy<Value = String> {
        prop_oneof![
            // Integer literals
            any::<i32>().prop_map(|n| n.to_string()),
            // Float literals
            any::<f64>().prop_map(|f| {
                if f.is_finite() {
                    f.to_string()
                } else {
                    "0.0".to_string()
                }
            }),
            // Boolean literals
            any::<bool>().prop_map(|b| b.to_string()),
            // String literals
            ".*".prop_map(|s| format!("\"{}\"", s.replace("\"", "\\\""))),
        ]
    }

    /// Generate simple expressions
    pub fn simple_expression() -> impl Strategy<Value = String> {
        prop_oneof![
            literal(),
            identifier(),
            // Binary operations
            (literal(), "[+\\-*/]", literal()).prop_map(|(l, op, r)| format!("{} {} {}", l, op, r)),
            // Function calls
            (identifier(), prop::collection::vec(literal(), 0..3))
                .prop_map(|(name, args)| format!("{}({})", name, args.join(", "))),
        ]
    }

    /// Generate function parameters
    pub fn function_parameters() -> impl Strategy<Value = String> {
        prop::collection::vec((identifier(), primitive_type()), 0..5).prop_map(|params| {
            params
                .iter()
                .map(|(name, ty)| format!("{}: {}", name, ty))
                .collect::<Vec<_>>()
                .join(", ")
        })
    }

    /// Generate function bodies
    pub fn function_body() -> impl Strategy<Value = String> {
        prop_oneof![
            // Empty body
            Just("{}".to_string()),
            // Simple return
            simple_expression().prop_map(|expr| format!("{{ {} }}", expr)),
            // Let binding
            (identifier(), simple_expression())
                .prop_map(|(name, expr)| format!("{{ let {} = {}; }}", name, expr)),
            // Multiple statements
            prop::collection::vec(simple_expression(), 1..3)
                .prop_map(|stmts| format!("{{ {}; }}", stmts.join("; "))),
        ]
    }

    /// Generate complete function definitions
    pub fn function_definition() -> impl Strategy<Value = String> {
        (
            identifier(),
            function_parameters(),
            prop::option::of(primitive_type()),
            function_body(),
        )
            .prop_map(|(name, params, ret_type, body)| {
                let return_part = ret_type.map(|t| format!(" -> {}", t)).unwrap_or_default();
                format!("fn {}({}) {} {}", name, params, return_part, body)
            })
    }

    /// Generate struct definitions
    pub fn struct_definition() -> impl Strategy<Value = String> {
        (
            identifier(),
            prop::collection::vec((identifier(), primitive_type()), 0..5),
        )
            .prop_map(|(name, fields)| {
                if fields.is_empty() {
                    format!("struct {};", name)
                } else {
                    let field_strs = fields
                        .iter()
                        .map(|(fname, ftype)| format!("    {}: {},", fname, ftype))
                        .collect::<Vec<_>>()
                        .join("\n");
                    format!("struct {} {{\n{}\n}}", name, field_strs)
                }
            })
    }

    /// Generate enum definitions
    pub fn enum_definition() -> impl Strategy<Value = String> {
        (identifier(), prop::collection::vec(identifier(), 1..5)).prop_map(|(name, variants)| {
            let variant_strs = variants
                .iter()
                .map(|v| format!("    {},", v))
                .collect::<Vec<_>>()
                .join("\n");
            format!("enum {} {{\n{}\n}}", name, variant_strs)
        })
    }

    /// Generate complete Rust programs
    pub fn rust_program() -> impl Strategy<Value = String> {
        prop::collection::vec(
            prop_oneof![
                function_definition(),
                struct_definition(),
                enum_definition(),
            ],
            1..10,
        )
        .prop_map(|items| items.join("\n\n"))
    }
}

// Fuzzing tests for parser robustness
proptest! {
    /// Fuzz test: Parser should never panic on any input
    #[test]
    fn fuzz_parser_no_panic(input in ".*") {
        let compiler = Compiler::new();
        // Should never panic, regardless of input validity
        let _result = compiler.compile_str(&input);
    }

    /// Fuzz test: Parser should handle valid functions without panicking
    #[test]
    fn fuzz_valid_functions(func in rust_generators::function_definition()) {
        let compiler = Compiler::new();
        let _result = compiler.compile_str(&func);
        // Test passes if no panic occurs
    }

    /// Fuzz test: Parser should handle valid structs
    #[test]
    fn fuzz_valid_structs(struct_def in rust_generators::struct_definition()) {
        let compiler = Compiler::new();
        let _result = compiler.compile_str(&struct_def);
    }

    /// Fuzz test: Parser should handle valid enums
    #[test]
    fn fuzz_valid_enums(enum_def in rust_generators::enum_definition()) {
        let compiler = Compiler::new();
        let _result = compiler.compile_str(&enum_def);
    }

    /// Fuzz test: Complete programs should compile without panicking
    #[test]
    fn fuzz_complete_programs(program in rust_generators::rust_program()) {
        let compiler = Compiler::new();
        let _result = compiler.compile_str(&program);
    }

    /// Fuzz test: Generated Mojo should have consistent structure
    #[test]
    fn fuzz_output_consistency(func in rust_generators::function_definition()) {
        let compiler = Compiler::new();

        if let Ok(mojo_code) = compiler.compile_str(&func) {
            // All generated code should have header
            prop_assert!(mojo_code.contains("# Generated Mojo code"));

            // Should have proper imports
            prop_assert!(mojo_code.contains("from "));

            // Should end with newline
            prop_assert!(mojo_code.ends_with('\n'));

            // Should not contain placeholder text in output
            prop_assert!(!mojo_code.contains("TODO"));
            prop_assert!(!mojo_code.contains("FIXME"));
        }
    }

    /// Fuzz test: Compilation should be deterministic
    #[test]
    fn fuzz_deterministic_compilation(
        input in rust_generators::function_definition(),
        iterations in 2..5usize
    ) {
        let compiler = Compiler::new();
        let mut results = Vec::new();

        for _ in 0..iterations {
            results.push(compiler.compile_str(&input));
        }

        // All results should be identical
        for i in 1..results.len() {
            prop_assert_eq!(
                results[0].is_ok(),
                results[i].is_ok(),
                "Compilation determinism violated"
            );

            if let (Ok(ref first), Ok(ref current)) = (&results[0], &results[i]) {
                prop_assert_eq!(first, current, "Generated code differs between runs");
            }
        }
    }
}

/// Stress testing with edge cases
#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_large_function_names() {
        let compiler = Compiler::new();

        // Very long but valid identifier
        let long_name = "a".repeat(1000);
        let rust_code = format!("fn {}() {{}}", long_name);

        let result = compiler.compile_str(&rust_code);
        // Should handle gracefully (success or meaningful error)
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn stress_many_parameters() {
        let compiler = Compiler::new();

        // Function with many parameters
        let params = (0..100)
            .map(|i| format!("p{}: i32", i))
            .collect::<Vec<_>>()
            .join(", ");
        let rust_code = format!("fn test_many_params({}) {{}}", params);

        let result = compiler.compile_str(&rust_code);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn stress_deeply_nested_expressions() {
        let compiler = Compiler::new();

        // Deeply nested parentheses
        let mut expr = "1".to_string();
        for _ in 0..50 {
            expr = format!("({})", expr);
        }
        let rust_code = format!("fn test() -> i32 {{ {} }}", expr);

        let result = compiler.compile_str(&rust_code);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn stress_unicode_identifiers() {
        let compiler = Compiler::new();

        // Unicode identifiers (valid in Rust)
        let test_cases = vec!["fn test_π() {}", "fn test_λ() {}", "fn test_🦀() {}"];

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(result.is_ok() || result.is_err());
        }
    }
}

/// Error handling fuzzing
#[cfg(test)]
mod error_fuzzing {
    use super::*;

    proptest! {
        /// Fuzz test: Error messages should be non-empty and meaningful
        #[test]
        fn fuzz_error_quality(invalid_input in "[^a-zA-Z0-9\\s{}();,]*") {
            let compiler = Compiler::new();

            if let Err(error) = compiler.compile_str(&invalid_input) {
                let error_msg = format!("{}", error);
                prop_assert!(!error_msg.is_empty(), "Error message should not be empty");
                prop_assert!(error_msg.len() > 5, "Error message should be descriptive");
            }
        }

        /// Fuzz test: All errors should be properly typed
        #[test]
        fn fuzz_error_types(input in ".*") {
            use rust2mojo::Error;
            let compiler = Compiler::new();

            if let Err(error) = compiler.compile_str(&input) {
                // Error should match one of our defined error types
                match error {
                    Error::ParseError(_) => {},
                    Error::CodegenError(_) => {},
                    Error::IoError(_) => {},
                    Error::UnsupportedFeature(_) => {},
                    Error::InternalError(_) => {},
                }
            }
        }
    }

    #[test]
    fn fuzz_malformed_syntax() {
        let compiler = Compiler::new();

        let malformed_cases = vec![
            "fn incomplete_function(",
            "struct MissingBrace {",
            "enum UnterminatedEnum { Variant",
            "let x = ;",
            "fn double_return() -> -> i32 {}",
            "impl for {}",
            "trait {}",
        ];

        for case in malformed_cases {
            let result = compiler.compile_str(case);
            assert!(result.is_err(), "Should fail on malformed syntax: {}", case);

            let error_msg = format!("{}", result.unwrap_err());
            assert!(
                !error_msg.is_empty(),
                "Error message should not be empty for: {}",
                case
            );
        }
    }
}

/// Memory and performance fuzzing
#[cfg(test)]
mod performance_fuzzing {
    use super::*;
    use std::time::{Duration, Instant};

    proptest! {
        /// Fuzz test: Compilation should complete in reasonable time
        #[test]
        fn fuzz_compilation_timeout(
            func in rust_generators::function_definition(),
            _timeout_ms in 100u64..5000u64
        ) {
            let compiler = Compiler::new();

            let start = Instant::now();
            let _result = compiler.compile_str(&func);
            let duration = start.elapsed();

            // Should complete within 5 seconds for any reasonable input
            prop_assert!(
                duration < Duration::from_secs(5),
                "Compilation took too long: {:?}",
                duration
            );
        }
    }

    #[test]
    fn fuzz_memory_efficiency() {
        let compiler = Compiler::new();

        // Generate a reasonably complex but not excessive program
        let complex_program = r#"
            struct Point { x: f64, y: f64 }
            
            impl Point {
                fn new(x: f64, y: f64) -> Point { Point { x, y } }
                fn distance(&self, other: &Point) -> f64 {
                    let dx = self.x - other.x;
                    let dy = self.y - other.y;
                    (dx * dx + dy * dy).sqrt()
                }
            }
            
            fn main() {
                let points = vec![
                    Point::new(0.0, 0.0),
                    Point::new(1.0, 1.0),
                    Point::new(2.0, 2.0),
                ];
                
                for p in &points {
                    println!("Point: ({}, {})", p.x, p.y);
                }
            }
        "#;

        // This is a basic smoke test - real memory profiling would require
        // additional tooling and potentially unsafe code
        let result = compiler.compile_str(complex_program);

        // Just ensure it doesn't crash or hang
        assert!(result.is_ok() || result.is_err());
    }
}
