# Claude Development Guidelines for rust2mojo

This document provides context and guidelines for Claude Code when working on the rust2mojo project.

## Project Overview

**rust2mojo** is a compiler that converts valid Rust code into valid Mojo code. The project aims to bridge Rust's superior safety guarantees with Mojo's better interoperability and performance, particularly for AI/ML applications.

### Core Architecture

- **Parser Module** (`src/parser.rs`): Uses `syn` crate to parse Rust AST into intermediate representation
- **AST Module** (`src/ast.rs`): Defines intermediate representation types
- **Codegen Module** (`src/codegen.rs`): Converts IR to Mojo source code
- **Error Module** (`src/error.rs`): Centralized error handling
- **Main/Lib** (`src/main.rs`, `src/lib.rs`): CLI interface and library API

### Key Design Principles

1. **Safety First**: Preserve Rust's safety guarantees in generated Mojo code where possible
2. **Correctness**: Every transformation must be semantically correct
3. **Performance**: Generated Mojo should be performant and idiomatic
4. **Modularity**: Clear separation between parsing, transformation, and generation phases

## Development Workflow

### Before Making Changes

1. **Run checks**: `make ci` to run full CI pipeline locally
2. **Read tests**: Understand existing test coverage for the area you're modifying
3. **Check TODOs**: Look for `TODO` comments related to your changes

### Testing Strategy

- **Unit tests**: Test individual functions and modules in isolation
- **Integration tests**: Test the full compilation pipeline with real Rust code
- **Property-based tests**: Use `proptest` for testing invariants
- **Benchmarks**: Measure performance impact of changes

### Code Style

- **Follow rustfmt**: Code is auto-formatted with our custom `rustfmt.toml`
- **Use clippy**: All clippy warnings must be addressed (`clippy.toml` configures lints)
- **Document everything**: Public APIs must have comprehensive documentation
- **Error handling**: Use our custom `Result<T>` type consistently

## Common Tasks

### Adding a New Rust Language Feature

1. **Update AST** (`src/ast.rs`): Add new AST node types if needed
2. **Update Parser** (`src/parser.rs`): Add parsing logic for the feature
3. **Update Codegen** (`src/codegen.rs`): Add Mojo generation logic
4. **Add Tests**: Both unit and integration tests
5. **Update Documentation**: If it affects the public API

### Debugging Compilation Issues

1. **Check AST**: Use `cargo run -- compile --debug-ast input.rs` to dump AST
2. **Check IR**: Our intermediate representation should be inspectable
3. **Incremental testing**: Start with minimal examples and build up complexity

### Performance Optimization

1. **Benchmark first**: Use `cargo bench` to establish baseline
2. **Profile**: Use tools like `perf` or `cargo flamegraph`
3. **Measure impact**: Ensure optimizations don't break correctness

## Testing Guidelines

### Test Categories

1. **Parser tests**: Test Rust parsing edge cases
2. **Codegen tests**: Test Mojo output correctness
3. **End-to-end tests**: Full Rust → Mojo compilation
4. **Error tests**: Ensure proper error handling and messages

### Test Data Organization

- `tests/fixtures/rust/`: Example Rust input files
- `tests/fixtures/mojo/`: Expected Mojo output files
- `tests/integration_tests.rs`: Main integration test suite

## Error Handling

- Use `anyhow::Result` for recoverable errors with context
- Use `thiserror::Error` for typed errors in library code
- Provide helpful error messages with source location information
- Test error paths as thoroughly as success paths

## Dependencies and Licensing

- **License enforcement**: `cargo deny` ensures only Apache-2.0/MIT licensed dependencies
- **Security auditing**: `cargo audit` checks for known vulnerabilities
- **MSRV**: Currently Rust 1.81.0, defined in `rust-toolchain.toml`

## Release Process

1. **Version bump**: Update `Cargo.toml` version
2. **Changelog**: Document changes in `CHANGELOG.md`
3. **Tag release**: Create Git tag following semver
4. **GitHub Actions**: Automated builds and publishing

## Common Pitfalls

- **Incomplete parsing**: Always handle all Rust language constructs gracefully
- **Mojo incompatibility**: Not all Rust features translate directly to Mojo
- **Memory safety**: Ensure generated Mojo maintains safety properties where possible
- **Performance**: Generated code should be reasonably efficient

## Debugging Tools

- `cargo expand`: See macro expansions
- `cargo tree`: Analyze dependency tree
- `cargo audit`: Security vulnerability scanning
- `cargo deny`: License and dependency policy enforcement

## Feature Priorities

1. **Core language features**: Functions, structs, basic control flow
2. **Type system**: Generics, traits, lifetimes (where applicable)
3. **Memory management**: References, ownership (Mojo equivalents)
4. **Advanced features**: Macros, async/await, specialized constructs

## Compatibility Notes

- **Rust editions**: Currently targeting Rust 2021 edition
- **Mojo version**: Targeting Mojo 24.5+
- **Platform support**: Linux, macOS, Windows (via CI)

## Git Workflow

- **Branch protection**: Main branch requires PR and passing CI
- **Commit messages**: Use conventional commits format
- **Pre-commit hooks**: Automatically run formatting and basic checks
- **Squash merging**: Keep history clean with squash merges for features

## Performance Benchmarks

- **Parsing speed**: Target <1ms for typical source files
- **Memory usage**: Keep peak memory under 100MB for large files
- **Compilation time**: Full pipeline should complete in seconds, not minutes

When in doubt, prioritize correctness over performance, and safety over convenience. The goal is to create a robust, reliable tool that developers can trust with their Rust code.