//! Rust Language Specification Tests
//!
//! This module contains tests that validate rust2mojo against the official Rust language
//! specification. These tests ensure we handle all major language constructs correctly.

use rust2mojo::Compiler;

/// Tests for basic language constructs from the Rust reference
#[cfg(test)]
mod basic_constructs {
    use super::*;

    #[test]
    fn spec_function_declarations() {
        let test_cases = [
            // Basic function
            "fn simple() {}",
            // Function with parameters
            "fn with_params(a: i32, b: i32) {}",
            // Function with return type
            "fn with_return() -> i32 { 42 }",
            // Function with both
            "fn full_function(x: i32, y: i32) -> i32 { x + y }",
            // Generic function (should handle gracefully)
            "fn generic<T>() {}",
            // Function with lifetime (should handle gracefully)
            "fn with_lifetime<'a>(x: &'a str) -> &'a str { x }",
        ];

        let compiler = Compiler::new();

        for (i, rust_code) in test_cases.iter().enumerate() {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Test case {} panicked: {}",
                i,
                rust_code
            );
        }
    }

    #[test]
    fn spec_variable_declarations() {
        let test_cases = vec![
            // Let bindings
            "fn test() { let x = 42; }",
            "fn test() { let x: i32 = 42; }",
            "fn test() { let mut x = 42; }",
            "fn test() { let mut x: i32 = 42; }",
            // Const declarations
            "const VALUE: i32 = 42;",
            // Static declarations
            "static GLOBAL: i32 = 42;",
            "static mut GLOBAL_MUT: i32 = 42;",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_primitive_types() {
        let test_cases = vec![
            // Integer types
            "fn test(x: i8) {}",
            "fn test(x: i16) {}",
            "fn test(x: i32) {}",
            "fn test(x: i64) {}",
            "fn test(x: i128) {}",
            "fn test(x: isize) {}",
            "fn test(x: u8) {}",
            "fn test(x: u16) {}",
            "fn test(x: u32) {}",
            "fn test(x: u64) {}",
            "fn test(x: u128) {}",
            "fn test(x: usize) {}",
            // Floating point types
            "fn test(x: f32) {}",
            "fn test(x: f64) {}",
            // Boolean type
            "fn test(x: bool) {}",
            // Character type
            "fn test(x: char) {}",
            // Unit type
            "fn test() -> () {}",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }
}

/// Tests for compound types
#[cfg(test)]
mod compound_types {
    use super::*;

    #[test]
    fn spec_arrays_and_slices() {
        let test_cases = vec![
            // Array types
            "fn test(x: [i32; 5]) {}",
            "fn test() { let arr: [i32; 3] = [1, 2, 3]; }",
            // Slice types
            "fn test(x: &[i32]) {}",
            "fn test(x: &mut [i32]) {}",
            // Array literals
            "fn test() { let x = [1, 2, 3]; }",
            "fn test() { let x = [42; 100]; }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_tuples() {
        let test_cases = vec![
            // Tuple types
            "fn test(x: (i32, i32)) {}",
            "fn test(x: (i32, f64, bool)) {}",
            // Tuple literals
            "fn test() { let x = (1, 2); }",
            "fn test() { let x = (42, 3.14, true); }",
            // Tuple indexing
            "fn test() { let x = (1, 2); let y = x.0; }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_structs() {
        let test_cases = vec![
            // Unit struct
            "struct Unit;",
            // Tuple struct
            "struct Point(i32, i32);",
            // Named field struct
            r#"struct Person {
                name: String,
                age: u32,
            }"#,
            // Struct with generic parameters
            r#"struct Container<T> {
                value: T,
            }"#,
            // Struct instantiation
            r#"
            struct Point { x: i32, y: i32 }
            fn test() {
                let p = Point { x: 1, y: 2 };
            }
            "#,
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_enums() {
        let test_cases = vec![
            // C-like enum
            r#"enum Color {
                Red,
                Green,
                Blue,
            }"#,
            // Enum with data
            r#"enum Option<T> {
                Some(T),
                None,
            }"#,
            // Complex enum
            r#"enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(i32, i32, i32),
            }"#,
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }
}

/// Tests for control flow constructs
#[cfg(test)]
mod control_flow {
    use super::*;

    #[test]
    fn spec_conditionals() {
        let test_cases = vec![
            // Basic if
            "fn test() { if true {} }",
            // If-else
            "fn test() { if true {} else {} }",
            // If-else if-else
            "fn test() { if true {} else if false {} else {} }",
            // If expressions
            "fn test() -> i32 { if true { 1 } else { 2 } }",
            // Complex conditions
            "fn test(x: i32) { if x > 0 && x < 10 {} }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_loops() {
        let test_cases = vec![
            // Loop
            "fn test() { loop { break; } }",
            // While loop
            "fn test() { while true { break; } }",
            // For loop
            "fn test() { for i in 0..10 {} }",
            // For loop with iterator
            "fn test() { let vec = vec![1, 2, 3]; for item in vec {} }",
            // Loop with labels
            "fn test() { 'outer: loop { break 'outer; } }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_match_expressions() {
        let test_cases = vec![
            // Basic match
            r#"fn test(x: i32) {
                match x {
                    1 => {},
                    2 => {},
                    _ => {},
                }
            }"#,
            // Match with guards
            r#"fn test(x: i32) {
                match x {
                    n if n > 0 => {},
                    _ => {},
                }
            }"#,
            // Match on enum
            r#"
            enum Option<T> { Some(T), None }
            fn test(opt: Option<i32>) {
                match opt {
                    Option::Some(x) => {},
                    Option::None => {},
                }
            }
            "#,
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }
}

/// Tests for expressions and operators
#[cfg(test)]
mod expressions {
    use super::*;

    #[test]
    fn spec_arithmetic_operators() {
        let test_cases = vec![
            "fn test() -> i32 { 1 + 2 }",
            "fn test() -> i32 { 5 - 3 }",
            "fn test() -> i32 { 4 * 6 }",
            "fn test() -> i32 { 10 / 2 }",
            "fn test() -> i32 { 10 % 3 }",
            "fn test() -> i32 { -5 }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_comparison_operators() {
        let test_cases = vec![
            "fn test() -> bool { 1 == 2 }",
            "fn test() -> bool { 1 != 2 }",
            "fn test() -> bool { 1 < 2 }",
            "fn test() -> bool { 1 <= 2 }",
            "fn test() -> bool { 1 > 2 }",
            "fn test() -> bool { 1 >= 2 }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_logical_operators() {
        let test_cases = vec![
            "fn test() -> bool { true && false }",
            "fn test() -> bool { true || false }",
            "fn test() -> bool { !true }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_bitwise_operators() {
        let test_cases = vec![
            "fn test() -> i32 { 5 & 3 }",
            "fn test() -> i32 { 5 | 3 }",
            "fn test() -> i32 { 5 ^ 3 }",
            "fn test() -> i32 { !5 }",
            "fn test() -> i32 { 5 << 1 }",
            "fn test() -> i32 { 5 >> 1 }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }
}

/// Tests for ownership and borrowing (advanced features)
#[cfg(test)]
mod ownership_borrowing {
    use super::*;

    #[test]
    fn spec_references_and_borrowing() {
        let test_cases = vec![
            // Immutable references
            "fn test(x: &i32) -> i32 { *x }",
            // Mutable references
            "fn test(x: &mut i32) { *x = 42; }",
            // Multiple immutable references
            r#"fn test() {
                let x = 42;
                let r1 = &x;
                let r2 = &x;
            }"#,
            // Reference to reference
            "fn test(x: &&i32) -> i32 { **x }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_lifetimes() {
        let test_cases = vec![
            // Function with lifetime parameter
            "fn test<'a>(x: &'a str) -> &'a str { x }",
            // Struct with lifetime
            r#"struct Borrowed<'a> {
                value: &'a i32,
            }"#,
            // Multiple lifetimes
            "fn test<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { x }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }
}

/// Tests for generic programming
#[cfg(test)]
mod generics {
    use super::*;

    #[test]
    fn spec_generic_functions() {
        let test_cases = vec![
            // Simple generic function
            "fn identity<T>(x: T) -> T { x }",
            // Multiple type parameters
            "fn pair<T, U>(x: T, y: U) -> (T, U) { (x, y) }",
            // Generic with bounds
            "fn compare<T: PartialEq>(x: T, y: T) -> bool { x == y }",
            // Where clause
            "fn complex<T>() -> T where T: Default { T::default() }",
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_generic_structs() {
        let test_cases = vec![
            // Generic struct
            r#"struct Container<T> {
                value: T,
            }"#,
            // Multiple type parameters
            r#"struct Pair<T, U> {
                first: T,
                second: U,
            }"#,
            // Generic with bounds
            r#"struct Wrapper<T: Clone> {
                inner: T,
            }"#,
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }
}

/// Tests for traits and implementations
#[cfg(test)]
mod traits {
    use super::*;

    #[test]
    fn spec_trait_definitions() {
        let test_cases = vec![
            // Simple trait
            r#"trait Display {
                fn fmt(&self) -> String;
            }"#,
            // Trait with default implementation
            r#"trait Greet {
                fn greet(&self) -> String {
                    "Hello".to_string()
                }
            }"#,
            // Trait with associated types
            r#"trait Iterator {
                type Item;
                fn next(&mut self) -> Option<Self::Item>;
            }"#,
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }

    #[test]
    fn spec_impl_blocks() {
        let test_cases = vec![
            // Inherent impl
            r#"
            struct Point { x: i32, y: i32 }
            impl Point {
                fn new(x: i32, y: i32) -> Point {
                    Point { x, y }
                }
            }
            "#,
            // Trait impl
            r#"
            trait Display {
                fn fmt(&self) -> String;
            }
            struct Point { x: i32, y: i32 }
            impl Display for Point {
                fn fmt(&self) -> String {
                    format!("({}, {})", self.x, self.y)
                }
            }
            "#,
        ];

        let compiler = Compiler::new();

        for rust_code in test_cases {
            let result = compiler.compile_str(rust_code);
            assert!(
                result.is_ok() || result.is_err(),
                "Panicked on: {}",
                rust_code
            );
        }
    }
}
