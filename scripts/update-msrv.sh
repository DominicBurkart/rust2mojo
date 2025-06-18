#!/usr/bin/env bash
#
# MSRV Update Script using cargo-msrv
# 
# This script uses the community-standard cargo-msrv tool to find
# and update the project's Minimum Supported Rust Version.
#
# Usage: ./scripts/update-msrv.sh [OPTIONS]
# 
# Exit codes:
#   0 - Success
#   1 - General failure
#   2 - cargo-msrv not installed

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

# Check if cargo-msrv is installed
check_cargo_msrv() {
    if ! command -v cargo-msrv &> /dev/null; then
        log_error "cargo-msrv is not installed. Please run:"
        echo "  cargo install cargo-msrv"
        echo "  OR"
        echo "  ./scripts/setup-dev-env.sh"
        exit 2
    fi
    
    log_info "Using cargo-msrv version: $(cargo msrv --version)"
}

# Find MSRV using cargo-msrv
find_msrv() {
    log_info "Finding MSRV using cargo-msrv (this may take a few minutes)..."
    
    # Use cargo msrv find with bisect strategy for faster results
    local msrv_result
    if msrv_result=$(cargo msrv find --bisect 2>&1); then
        # Extract MSRV from output
        local msrv=$(echo "$msrv_result" | grep -o "Minimum Supported Rust Version: [0-9]\+\.[0-9]\+\.[0-9]\+" | cut -d' ' -f5 || echo "")
        
        if [[ -n "$msrv" ]]; then
            log_success "Found MSRV: $msrv"
            echo "$msrv"
        else
            log_error "Could not parse MSRV from cargo-msrv output"
            log_info "cargo-msrv output:"
            echo "$msrv_result"
            exit 1
        fi
    else
        log_error "cargo msrv find failed:"
        echo "$msrv_result"
        exit 1
    fi
}

# Update configuration files with new MSRV
update_config_files() {
    local new_msrv="$1"
    
    log_info "Updating configuration files to MSRV: $new_msrv"
    
    # Update Cargo.toml
    if sed -i "s/rust-version = \"[^\"]*\"/rust-version = \"$new_msrv\"/" Cargo.toml; then
        log_success "Updated Cargo.toml rust-version to $new_msrv"
    else
        log_error "Failed to update Cargo.toml"
        exit 1
    fi
    
    # Update clippy.toml
    if sed -i "s/msrv = \"[^\"]*\"/msrv = \"$new_msrv\"/" clippy.toml; then
        log_success "Updated clippy.toml msrv to $new_msrv"
    else
        log_error "Failed to update clippy.toml"
        exit 1
    fi
    
    # Update rust-toolchain.toml
    if sed -i "s/channel = \"[^\"]*\"/channel = \"$new_msrv\"/" rust-toolchain.toml; then
        log_success "Updated rust-toolchain.toml channel to $new_msrv"
    else
        log_error "Failed to update rust-toolchain.toml"
        exit 1
    fi
}

# Verify MSRV works
verify_msrv() {
    local msrv="$1"
    
    log_info "Verifying MSRV $msrv works with cargo-msrv..."
    
    if cargo msrv verify --rust-version "$msrv"; then
        log_success "MSRV $msrv verification passed!"
    else
        log_error "MSRV $msrv verification failed"
        exit 1
    fi
}

# Show current configuration
show_current_config() {
    log_info "Current project configuration:"
    local current_toolchain=$(grep 'channel = ' rust-toolchain.toml | sed 's/.*"\([^"]*\)".*/\1/')
    local current_cargo=$(grep 'rust-version = ' Cargo.toml | sed 's/.*"\([^"]*\)".*/\1/')
    local current_clippy=$(grep 'msrv = ' clippy.toml | sed 's/.*"\([^"]*\)".*/\1/')
    
    printf "  %-25s %s\n" "rust-toolchain.toml:" "$current_toolchain"
    printf "  %-25s %s\n" "Cargo.toml:" "$current_cargo"  
    printf "  %-25s %s\n" "clippy.toml:" "$current_clippy"
    echo
}

# Main execution
main() {
    echo "========================================="
    echo "rust2mojo MSRV Update (using cargo-msrv)"
    echo "========================================="
    echo
    
    check_project_root
    check_cargo_msrv
    show_current_config
    
    # Find optimal MSRV
    local optimal_msrv
    optimal_msrv=$(find_msrv)
    
    echo
    log_info "=== MSRV Analysis Results ==="
    log_success "Optimal MSRV found: $optimal_msrv"
    echo
    
    read -p "Would you like to update all configuration files to MSRV $optimal_msrv? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        update_config_files "$optimal_msrv"
        
        echo
        log_info "Verifying the new MSRV..."
        verify_msrv "$optimal_msrv"
        
        echo
        log_success "MSRV update complete!"
        echo
        echo "Next steps:"
        echo "  1. Test locally: cargo check && cargo test"
        echo "  2. Commit changes and push for CI validation"
        echo "  3. Monitor CI to ensure MSRV compatibility"
    else
        log_info "Skipping configuration updates"
    fi
}

# Handle command line arguments
case "${1:-}" in
    -h|--help)
        echo "rust2mojo MSRV Update Tool (using cargo-msrv)"
        echo
        echo "USAGE:"
        echo "    $0 [OPTIONS]"
        echo
        echo "OPTIONS:"
        echo "    -h, --help      Show this help message"
        echo "    --find-only     Only find MSRV, don't update files"
        echo "    --verify MSRV   Verify a specific MSRV version"
        echo
        echo "DESCRIPTION:"
        echo "    Uses the community-standard cargo-msrv tool to find the"
        echo "    optimal Minimum Supported Rust Version and update all"
        echo "    project configuration files."
        echo
        echo "REQUIRES:"
        echo "    cargo-msrv (install with: cargo install cargo-msrv)"
        exit 0
        ;;
    --find-only)
        check_project_root
        check_cargo_msrv
        find_msrv
        exit 0
        ;;
    --verify)
        if [[ -z "${2:-}" ]]; then
            log_error "Please specify MSRV version to verify"
            exit 1
        fi
        check_project_root
        check_cargo_msrv
        verify_msrv "$2"
        exit 0
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