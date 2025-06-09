# Makefile for rust2mojo development tasks

.PHONY: help install check test test-contracts test-spec test-fuzz test-integration lint format audit bench clean docs ci

# Default target
help:
	@echo "Available targets:"
	@echo "  install        - Install development dependencies"
	@echo "  check          - Run cargo check"
	@echo "  test           - Run all tests"
	@echo "  test-contracts - Run contract tests"
	@echo "  test-spec      - Run language specification tests"
	@echo "  test-fuzz      - Run fuzzing/property-based tests"
	@echo "  test-integration - Run integration tests"
	@echo "  lint           - Run clippy lints"
	@echo "  format         - Format code with rustfmt"
	@echo "  audit          - Run security audit"
	@echo "  bench          - Run benchmarks"
	@echo "  clean          - Clean build artifacts"
	@echo "  docs           - Generate documentation"
	@echo "  ci             - Run full CI pipeline"

# Install development dependencies
install:
	@echo "Installing development dependencies..."
	cargo install cargo-audit cargo-deny
	@echo "Dependencies installed!"

# Quick check
check:
	cargo check --all-targets --all-features

# Run all tests
test:
	cargo test --all-features

# Run contract tests
test-contracts:
	cargo test --test contracts

# Run language specification tests  
test-spec:
	cargo test --test rust_language_spec

# Run fuzzing/property-based tests
test-fuzz:
	PROPTEST_CASES=1000 cargo test --test fuzzing

# Run integration tests
test-integration:
	cargo test --test integration_tests

# Run clippy lints
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Format code
format:
	cargo fmt --all

# Check formatting
format-check:
	cargo fmt --all -- --check

# Security audit
audit:
	cargo audit

# License and dependency check
deny:
	cargo deny check

# Run benchmarks
bench:
	cargo bench

# Clean build artifacts
clean:
	cargo clean

# Generate documentation
docs:
	cargo doc --all-features --no-deps --open

# Full CI pipeline
ci: format-check lint test test-contracts test-spec test-fuzz test-integration audit deny
	@echo "All CI checks passed!"

# Development setup
dev-setup: install
	@echo "Setting up development environment..."
	git config core.hooksPath .husky
	@echo "Development environment ready!"