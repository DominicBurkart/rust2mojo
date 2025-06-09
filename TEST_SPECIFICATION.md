# rust2mojo Test Specification

This document defines the comprehensive testing strategy for the rust2mojo compiler, ensuring correctness, safety, and robustness through multiple layers of validation.

## Testing Philosophy

The rust2mojo compiler follows a **Test-Driven Development (TDD)** approach with multiple validation layers:

1. **Contracts**: Executable specifications that define what the compiler MUST do
2. **Language Specification**: Compliance tests against the Rust language reference
3. **Property-based Testing**: Automated discovery of edge cases through fuzzing
4. **Integration Testing**: End-to-end validation with real-world code
5. **Performance Testing**: Ensuring compilation speed and memory efficiency

## Test Categories

### 1. Contract Tests (`tests/contracts.rs`)

These tests define the fundamental invariants and contracts that the compiler must satisfy:

#### Compilation Contracts
- **Deterministic Compilation**: Same input always produces same output
- **Generated Code Validity**: All output must be syntactically valid Mojo
- **Error Handling Quality**: Meaningful error messages for invalid input
- **Empty Input Handling**: Graceful handling of edge cases

#### Type System Contracts
- **Type Mapping Correctness**: Rust types map to appropriate Mojo types
- **Function Signature Preservation**: Function interfaces are correctly translated
- **Generic Handling**: Graceful handling of Rust generics (even if not fully supported)

#### Memory Safety Contracts
- **Reference Safety**: Safe handling of Rust references and borrowing
- **Ownership Preservation**: Maintain ownership semantics where possible

#### Performance Contracts
- **Compilation Speed**: Small functions compile in <100ms
- **Memory Efficiency**: Reasonable memory usage during compilation

### 2. Language Specification Tests (`tests/rust_language_spec.rs`)

Systematic validation against the Rust language reference:

#### Basic Constructs
- Function declarations (all variations)
- Variable declarations (let, const, static)
- Primitive types (all integer, float, bool, char types)

#### Compound Types
- Arrays and slices
- Tuples
- Structs (unit, tuple, named fields)
- Enums (C-like, with data, complex variants)

#### Control Flow
- Conditionals (if, if-else, if-else-if)
- Loops (loop, while, for)
- Match expressions

#### Expressions and Operators
- Arithmetic operators (+, -, *, /, %, unary -)
- Comparison operators (==, !=, <, <=, >, >=)
- Logical operators (&&, ||, !)
- Bitwise operators (&, |, ^, !, <<, >>)

#### Advanced Features
- References and borrowing
- Lifetimes
- Generic functions and structs
- Traits and implementations

### 3. Property-Based/Fuzzing Tests (`tests/fuzzing.rs`)

Automated testing to discover edge cases and ensure robustness:

#### Parser Robustness
- **No-Panic Guarantee**: Parser never panics on any input
- **Valid Input Handling**: All syntactically valid Rust compiles without panic
- **Output Consistency**: Generated code has consistent structure
- **Deterministic Compilation**: Multiple runs produce identical results

#### Stress Testing
- Large function names (1000+ characters)
- Many parameters (100+ parameters)
- Deeply nested expressions
- Unicode identifiers

#### Error Handling Quality
- Non-empty, meaningful error messages
- Proper error type classification
- Graceful handling of malformed syntax

#### Performance Testing
- Compilation timeout limits (<5 seconds for any input)
- Memory efficiency validation

### 4. Integration Tests (`tests/integration_tests.rs`)

End-to-end testing with realistic Rust code:

- Basic function compilation
- Main function special handling
- Struct definition and instantiation
- Complex multi-item programs
- Error propagation through full pipeline

## Test Data Organization

```
tests/
├── contracts.rs           # Fundamental compiler contracts
├── rust_language_spec.rs  # Rust language compliance tests
├── fuzzing.rs            # Property-based and fuzzing tests
├── integration_tests.rs  # End-to-end integration tests
└── fixtures/             # Test data files
    ├── rust/             # Example Rust input files
    │   ├── simple/       # Basic language constructs
    │   ├── complex/      # Advanced features
    │   └── invalid/      # Invalid Rust for error testing
    └── mojo/             # Expected Mojo output files
        ├── simple/
        └── complex/
```

## Running Tests

### All Tests
```bash
cargo test --all-features
```

### Test Categories
```bash
# Contract tests
cargo test --test contracts

# Language specification tests
cargo test --test rust_language_spec

# Fuzzing tests (with increased iterations)
cargo test --test fuzzing

# Integration tests
cargo test --test integration_tests

# Doctests (examples in documentation)
cargo test --doc
```

### Continuous Integration
```bash
# Full CI pipeline
make ci

# Quick development checks
make check lint test
```

## Property-Based Testing Strategy

Using the `proptest` crate, we generate random but valid Rust code to test:

1. **Parser Robustness**: Never panic on any input
2. **Output Consistency**: Same input → same output
3. **Structural Invariants**: All output has proper header/imports/formatting
4. **Error Quality**: Meaningful errors for invalid input

### Generators
- `identifier()`: Valid Rust identifiers
- `primitive_type()`: All Rust primitive types
- `literal()`: Integer, float, bool, string literals
- `function_definition()`: Complete function definitions
- `struct_definition()`: Struct definitions
- `enum_definition()`: Enum definitions
- `rust_program()`: Multi-item programs

## Success Criteria

### Phase 1: Basic Functionality
- ✅ All contract tests pass
- ✅ Basic language constructs compile
- ✅ No panics on any input
- ✅ Meaningful error messages

### Phase 2: Language Coverage
- [ ] 80%+ of Rust language specification tests pass
- [ ] Complex programs compile successfully
- [ ] Advanced features handled gracefully

### Phase 3: Production Readiness
- [ ] 95%+ specification compliance
- [ ] Performance benchmarks met
- [ ] Memory efficiency validated
- [ ] Real-world code compatibility

## Test Maintenance

### Adding New Tests
1. **Identify the category**: Contract, specification, fuzzing, or integration
2. **Write the test**: Follow existing patterns and naming conventions
3. **Document the requirement**: Update this specification
4. **Verify CI passes**: Ensure all tests pass in CI environment

### Test Data Management
- Keep test cases focused and minimal
- Use property-based testing for broad coverage
- Maintain clear separation between test categories
- Regular review and cleanup of test suites

### Performance Monitoring
- Track compilation speed regression
- Monitor memory usage trends
- Set up alerts for test timeout increases
- Regular benchmark runs in CI

## Error Testing Strategy

### Invalid Input Categories
1. **Syntax Errors**: Malformed Rust syntax
2. **Semantic Errors**: Valid syntax but invalid semantics
3. **Unsupported Features**: Valid Rust not yet supported by rust2mojo
4. **Edge Cases**: Boundary conditions and unusual inputs

### Error Quality Metrics
- Error messages must be non-empty
- Errors must be properly categorized
- Error locations should be helpful
- Suggestions for fixes when possible

## Coverage Goals

- **Line Coverage**: >90% for core compiler logic
- **Branch Coverage**: >85% for all control flow paths
- **Property Coverage**: 100% of defined contracts must pass
- **Specification Coverage**: Systematic coverage of Rust language features

This comprehensive testing strategy ensures that rust2mojo is reliable, correct, and robust enough for production use while maintaining the safety guarantees that make Rust valuable.