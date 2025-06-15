#!/usr/bin/env bash
#
# MSRV Analysis Script
# 
# This script analyzes all dependencies to find the minimum Rust version
# required by the project based on actual dependency requirements.
#
# Usage: ./scripts/analyze-msrv.sh
# 
# Exit codes:
#   0 - Success
#   1 - General failure

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

# Extract MSRV from a Cargo.toml file
extract_msrv() {
    local cargo_toml="$1"
    local msrv=""
    
    if [[ -f "$cargo_toml" ]]; then
        # Try to extract rust-version field
        msrv=$(grep -E '^rust-version\s*=' "$cargo_toml" 2>/dev/null | sed 's/.*"\([^"]*\)".*/\1/' || echo "")
        
        # If no rust-version, try to extract from minimum supported Rust comment
        if [[ -z "$msrv" ]]; then
            msrv=$(grep -i "minimum.*rust.*version\|msrv" "$cargo_toml" 2>/dev/null | head -1 | sed 's/.*\([0-9]\+\.[0-9]\+\).*/\1/' || echo "")
        fi
    fi
    
    echo "$msrv"
}

# Compare version strings (returns 0 if v1 >= v2, 1 if v1 < v2)
version_gte() {
    local v1="$1"
    local v2="$2"
    
    # Convert versions to comparable format (remove dots and pad)
    local v1_num=$(echo "$v1" | sed 's/\.//g' | sed 's/^/1/' | head -c 6 | sed 's/$/000000/' | head -c 6)
    local v2_num=$(echo "$v2" | sed 's/\.//g' | sed 's/^/1/' | head -c 6 | sed 's/$/000000/' | head -c 6)
    
    [[ "$v1_num" -ge "$v2_num" ]]
}

# Analyze all dependencies
analyze_dependencies() {
    log_info "Analyzing dependency MSRV requirements..."
    
    local highest_msrv="1.0"
    local highest_crate=""
    local total_deps=0
    local deps_with_msrv=0
    
    # Create temporary file for dependency analysis
    local temp_file=$(mktemp)
    local results_file=$(mktemp)
    
    # Get all dependencies with their Cargo.toml locations
    cargo metadata --format-version 1 | \
        jq -r '.packages[] | select(.name != "rust2mojo") | "\(.name) \(.manifest_path)"' > "$temp_file"
    
    echo "Crate,MSRV,Source" > "$results_file"
    
    while IFS=' ' read -r crate_name manifest_path; do
        total_deps=$((total_deps + 1))
        local msrv=$(extract_msrv "$manifest_path")
        
        if [[ -n "$msrv" && "$msrv" != "1.0" ]]; then
            deps_with_msrv=$((deps_with_msrv + 1))
            echo "$crate_name,$msrv,$manifest_path" >> "$results_file"
            
            # Track highest MSRV
            if version_gte "$msrv" "$highest_msrv"; then
                highest_msrv="$msrv"
                highest_crate="$crate_name"
            fi
        fi
    done < "$temp_file"
    
    # Display results
    echo
    log_info "=== MSRV Analysis Results ==="
    echo "Total dependencies analyzed: $total_deps"
    echo "Dependencies with explicit MSRV: $deps_with_msrv"
    echo
    
    if [[ "$deps_with_msrv" -gt 0 ]]; then
        log_success "Highest MSRV requirement: $highest_msrv (from $highest_crate)"
        echo
        
        log_info "Dependencies with MSRV requirements:"
        sort -t',' -k2,2V "$results_file" | while IFS=',' read -r crate msrv source; do
            [[ "$crate" == "Crate" ]] && continue  # Skip header
            printf "  %-30s %s\n" "$crate" "$msrv"
        done
        
        echo
        log_info "Current project configuration:"
        local current_toolchain=$(grep 'channel = ' rust-toolchain.toml | sed 's/.*"\([^"]*\)".*/\1/')
        local current_cargo=$(grep 'rust-version = ' Cargo.toml | sed 's/.*"\([^"]*\)".*/\1/')
        local current_clippy=$(grep 'msrv = ' clippy.toml | sed 's/.*"\([^"]*\)".*/\1/')
        
        printf "  %-30s %s\n" "rust-toolchain.toml:" "$current_toolchain"
        printf "  %-30s %s\n" "Cargo.toml:" "$current_cargo"  
        printf "  %-30s %s\n" "clippy.toml:" "$current_clippy"
        
        echo
        if version_gte "$current_toolchain" "$highest_msrv"; then
            log_success "Current MSRV ($current_toolchain) is compatible with all dependencies"
            echo
            log_info "Recommendations:"
            echo "  - Could potentially lower MSRV to: $highest_msrv"
            echo "  - This would save $(echo "$current_toolchain $highest_msrv" | awk '{print $1 - $2}') Rust versions"
            echo "  - Requires CI testing to validate compatibility"
        else
            log_error "Current MSRV ($current_toolchain) is LOWER than required ($highest_msrv)"
            echo "  This should not happen - please investigate dependencies"
        fi
    else
        log_warning "No dependencies specify explicit MSRV requirements"
        log_info "This means most dependencies are compatible with very old Rust versions"
    fi
    
    # Cleanup
    rm -f "$temp_file" "$results_file"
    
    # Return the highest MSRV found
    echo "$highest_msrv"
}

# Generate CI configuration for MSRV testing
generate_msrv_ci() {
    local target_msrv="$1"
    
    log_info "Generating CI configuration for MSRV testing..."
    
    cat > .github/workflows/msrv.yml << EOF
name: MSRV Check

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1
  CARGO_INCREMENTAL: 0

jobs:
  msrv:
    name: Minimum Supported Rust Version
    runs-on: ubuntu-latest
    steps:
    - name: Checkout sources
      uses: actions/checkout@v4

    - name: Install MSRV toolchain
      uses: dtolnay/rust-toolchain@${target_msrv}
      with:
        components: rustfmt, clippy

    - name: Cache cargo registry
      uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/registry/index/
          ~/.cargo/registry/cache/
          ~/.cargo/git/db/
        key: \${{ runner.os }}-cargo-\${{ hashFiles('**/Cargo.lock') }}

    - name: Cache cargo build
      uses: actions/cache@v4
      with:
        path: target/
        key: \${{ runner.os }}-msrv-\${{ hashFiles('**/Cargo.lock') }}

    - name: Check formatting (MSRV)
      run: cargo fmt --all --check

    - name: Run clippy (MSRV)
      run: cargo clippy --all-targets --all-features -- -D warnings

    - name: Run tests (MSRV)
      run: cargo test --all-features

    - name: Check examples compile (MSRV)
      run: cargo check --examples --all-features

    - name: Verify Cargo.toml consistency
      run: |
        CARGO_MSRV=\$(grep 'rust-version' Cargo.toml | sed 's/.*"\([^"]*\)".*/\1/')
        EXPECTED_MSRV="${target_msrv}"
        if [[ "\$CARGO_MSRV" != "\$EXPECTED_MSRV" ]]; then
          echo "ERROR: Cargo.toml rust-version (\$CARGO_MSRV) doesn't match MSRV (\$EXPECTED_MSRV)"
          exit 1
        fi
        echo "✅ Cargo.toml rust-version matches MSRV: \$CARGO_MSRV"
EOF

    log_success "Created .github/workflows/msrv.yml for Rust $target_msrv"
}

# Main execution
main() {
    echo "========================================="
    echo "rust2mojo MSRV Analysis"
    echo "========================================="
    echo
    
    check_project_root
    
    # Perform dependency analysis
    local recommended_msrv
    recommended_msrv=$(analyze_dependencies)
    
    echo
    log_info "=== Recommendations ==="
    
    if [[ "$recommended_msrv" != "1.0" ]]; then
        echo "1. Update rust-version in Cargo.toml to: $recommended_msrv"
        echo "2. Update msrv in clippy.toml to: $recommended_msrv" 
        echo "3. Add MSRV CI testing workflow"
        echo "4. Test compatibility across the supported version range"
        echo
        
        read -p "Would you like to apply these changes? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            log_info "Applying MSRV optimizations..."
            
            # Update Cargo.toml
            sed -i "s/rust-version = \"[^\"]*\"/rust-version = \"$recommended_msrv\"/" Cargo.toml
            log_success "Updated Cargo.toml rust-version to $recommended_msrv"
            
            # Update clippy.toml  
            sed -i "s/msrv = \"[^\"]*\"/msrv = \"$recommended_msrv\"/" clippy.toml
            log_success "Updated clippy.toml msrv to $recommended_msrv"
            
            # Generate MSRV CI
            generate_msrv_ci "$recommended_msrv"
            
            echo
            log_success "MSRV optimization complete!"
            echo
            echo "Next steps:"
            echo "  1. Test locally: cargo check && cargo test"
            echo "  2. Commit changes and push for CI validation"
            echo "  3. Monitor CI to ensure MSRV compatibility"
        else
            log_info "Skipping automatic updates"
        fi
    else
        log_warning "No specific MSRV requirements found in dependencies"
        log_info "Current configuration appears to be appropriate"
    fi
}

# Handle command line arguments
case "${1:-}" in
    -h|--help)
        echo "rust2mojo MSRV Analysis Tool"
        echo
        echo "USAGE:"
        echo "    $0 [OPTIONS]"
        echo
        echo "OPTIONS:"
        echo "    -h, --help      Show this help message"
        echo "    --analyze-only  Only analyze, don't prompt for changes"
        echo
        echo "DESCRIPTION:"
        echo "    Analyzes all project dependencies to determine the minimum"
        echo "    supported Rust version (MSRV) and provides recommendations"
        echo "    for optimization."
        exit 0
        ;;
    --analyze-only)
        check_project_root
        analyze_dependencies > /dev/null
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