//! Integration tests for the rust2mojo compiler

use rust2mojo::Compiler;

#[test]
fn test_basic_function_compilation() {
    let rust_code = r#"
        fn hello_world() {
            println!("Hello, world!");
        }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile_str(rust_code);

    assert!(result.is_ok());
    let mojo_code = result.unwrap();
    assert!(mojo_code.contains("fn hello_world():"));
}

#[test]
fn test_main_function_compilation() {
    let rust_code = r#"
        fn main() {
            let x = 42;
        }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile_str(rust_code);

    assert!(result.is_ok());
    let mojo_code = result.unwrap();
    assert!(mojo_code.contains("fn main():"));
}

#[test]
fn test_struct_compilation() {
    let rust_code = r#"
        struct Point {
            x: i32,
            y: i32,
        }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile_str(rust_code);

    assert!(result.is_ok());
    let mojo_code = result.unwrap();
    assert!(mojo_code.contains("struct Point:"));
}

#[test]
fn test_invalid_rust_code() {
    let rust_code = r#"
        fn invalid syntax {
            // This is invalid Rust code
        }
    "#;

    let compiler = Compiler::new();
    let result = compiler.compile_str(rust_code);

    assert!(result.is_err());
}
