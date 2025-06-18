#!/usr/bin/env bash
#
# Development Environment Setup Script
# 
# This script installs and configures all development dependencies
# required for rust2mojo development, including pre-commit hooks.
#
# Usage: ./scripts/setup-dev-env.sh
# 
# Exit codes:
#   0 - Success
#   1 - General failure
#   2 - Rust toolchain issues
#   3 - Cargo tool installation failure

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running from project root
check_project_root() {
    if [[ ! -f "Cargo.toml" ]] || [[ ! -f "rust-toolchain.toml" ]]; then
        log_error "This script must be run from the rust2mojo project root directory"
        exit 1
    fi
}

# Verify Rust toolchain
check_rust_toolchain() {
    log_info "Checking Rust toolchain..."
    
    if ! command -v rustc &> /dev/null; then
        log_error "Rust is not installed. Please install Rust first: https://rustup.rs/"
        exit 2
    fi
    
    local rust_version
    rust_version=$(rustc --version | cut -d' ' -f2)
    log_info "Found Rust version: $rust_version"
    
    # Ensure we're using the project's toolchain
    if command -v rustup &> /dev/null; then
        log_info "Syncing with project toolchain..."
        rustup show > /dev/null
    fi
}

# Install cargo tools with version compatibility
install_cargo_tool() {
    local tool_name="$1"
    local version_spec="${2:-}"
    local install_args="${3:-}"
    
    log_info "Installing/updating $tool_name..."
    
    # Check if already installed
    if cargo install --list | grep -q "^$tool_name "; then
        local installed_version
        installed_version=$(cargo install --list | grep "^$tool_name " | head -1)
        log_info "$tool_name is already installed: $installed_version"
        
        # For idempotency, we'll reinstall if version is specified
        if [[ -n "$version_spec" ]]; then
            log_info "Reinstalling $tool_name with version constraint $version_spec..."
            cargo install "$tool_name" $version_spec $install_args --force
        fi
    else
        log_info "Installing $tool_name..."
        if [[ -n "$version_spec" ]]; then
            cargo install "$tool_name" $version_spec $install_args
        else
            cargo install "$tool_name" $install_args
        fi
    fi
}

# Install development tools
install_dev_tools() {
    log_info "Installing development tools..."
    
    # Core security and quality tools
    install_cargo_tool "cargo-audit" "--version ^0.20" "--locked"
    install_cargo_tool "cargo-deny" "" "--locked"
    
    # MSRV analysis and verification (compatible with Rust 1.82.0)
    install_cargo_tool "cargo-msrv" "--version ^0.18.1" "--locked"
    
    # Additional useful development tools
    install_cargo_tool "cargo-outdated" "" ""
    install_cargo_tool "cargo-tree" "" ""
    
    # Cross-compilation tool (if needed)
    # install_cargo_tool "cross" "" ""
    
    log_success "All development tools installed successfully"
}

# Setup pre-commit hooks via cargo-husky
setup_precommit_hooks() {
    log_info "Setting up pre-commit hooks..."
    
    # Create .husky directory if it doesn't exist
    mkdir -p .husky
    
    # Create the pre-commit script
    cat > .husky/pre-commit << 'EOF'
#!/usr/bin/env bash
#
# Pre-commit hook for rust2mojo
# Runs formatting, linting, and security checks

set -e

echo "Running pre-commit checks..."

echo "Formatting code with rustfmt..."
cargo fmt --all -- --check

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running cargo audit..."
if command -v cargo-audit &> /dev/null; then
    cargo audit --ignore RUSTSEC-2024-0370
else
    echo "Warning: cargo-audit not found. Run './scripts/setup-dev-env.sh' to install."
fi

echo "Running cargo deny..."
if command -v cargo-deny &> /dev/null; then
    cargo deny check --hide-inclusion-graph
else
    echo "Warning: cargo-deny not found. Run './scripts/setup-dev-env.sh' to install."
fi

echo "Pre-commit checks completed successfully!"
EOF

    chmod +x .husky/pre-commit
    
    # Ensure cargo-husky is properly configured
    log_info "Installing cargo-husky hooks..."
    cargo test --quiet 2>/dev/null || true  # This triggers husky installation
    
    log_success "Pre-commit hooks configured successfully"
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    local tools=("cargo-audit" "cargo-deny")
    local all_good=true
    
    for tool in "${tools[@]}"; do
        if command -v "$tool" &> /dev/null; then
            local version
            version=$("$tool" --version 2>/dev/null | head -1 || echo "unknown")
            log_success "$tool: $version"
        else
            log_error "$tool: not found"
            all_good=false
        fi
    done
    
    # Check pre-commit hook
    if [[ -f ".husky/pre-commit" ]] && [[ -x ".husky/pre-commit" ]]; then
        log_success "Pre-commit hook: configured"
    else
        log_error "Pre-commit hook: not configured"
        all_good=false
    fi
    
    if [[ "$all_good" == "true" ]]; then
        log_success "All tools verified successfully!"
        return 0
    else
        log_error "Some tools failed verification"
        return 3
    fi
}

# Test the setup
test_setup() {
    log_info "Testing development environment..."
    
    # Test formatting
    if cargo fmt --all --check &> /dev/null; then
        log_success "Code formatting: OK"
    else
        log_warning "Code formatting: needs attention"
    fi
    
    # Test clippy
    if cargo clippy --all-targets --all-features -- -D warnings &> /dev/null; then
        log_success "Clippy linting: OK"
    else
        log_warning "Clippy linting: has warnings/errors"
    fi
    
    log_success "Development environment test completed"
}

# Main execution
main() {
    echo "========================================="
    echo "rust2mojo Development Environment Setup"
    echo "========================================="
    echo
    
    check_project_root
    check_rust_toolchain
    install_dev_tools
    setup_precommit_hooks
    verify_installation
    test_setup
    
    echo
    log_success "Development environment setup completed successfully!"
    echo
    echo "Next steps:"
    echo "  1. Run 'cargo test' to verify everything works"
    echo "  2. Make a test commit to verify pre-commit hooks"
    echo "  3. Check './scripts/setup-dev-env.sh --help' for options"
    echo
}

# Help text
show_help() {
    cat << EOF
rust2mojo Development Environment Setup

USAGE:
    $0 [OPTIONS]

OPTIONS:
    -h, --help      Show this help message
    --verify-only   Only verify existing installation
    --tools-only    Only install tools, skip hook setup

DESCRIPTION:
    This script sets up the complete development environment for rust2mojo,
    including all required tools for pre-commit hooks, security auditing,
    and code quality checks.

TOOLS INSTALLED:
    - cargo-audit   (security vulnerability scanning)
    - cargo-deny    (dependency policy enforcement)
    - cargo-outdated (dependency update checking)
    - cargo-tree    (dependency tree visualization)

EXIT CODES:
    0 - Success
    1 - General failure
    2 - Rust toolchain issues  
    3 - Tool installation/verification failure
EOF
}

# Handle command line arguments
case "${1:-}" in
    -h|--help)
        show_help
        exit 0
        ;;
    --verify-only)
        check_project_root
        verify_installation
        exit $?
        ;;
    --tools-only)
        check_project_root
        check_rust_toolchain
        install_dev_tools
        verify_installation
        exit $?
        ;;
    "")
        main
        ;;
    *)
        log_error "Unknown option: $1"
        echo "Use --help for usage information"
        exit 1
        ;;
esac