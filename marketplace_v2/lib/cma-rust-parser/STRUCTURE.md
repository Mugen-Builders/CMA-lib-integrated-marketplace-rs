# Repository Structure

This document explains the structure of the `cma-rust-parser` crate and how it's organized for publishing to crates.io.

## Directory Layout

```
cma-rust-parser/
├── Cargo.toml              # Main crate configuration for publishing
├── build.rs                # Build script that generates Rust bindings from C headers
├── wrapper.h               # C header wrapper that includes all necessary headers
├── README.md               # User-facing documentation
├── .gitignore             # Git ignore rules
├── src/                    # Rust source code
│   ├── lib.rs             # Library root - exports all public APIs
│   ├── error.rs           # Error types (LedgerError, ParserError)
│   ├── ledger.rs          # Ledger implementation (main functionality)
│   ├── types.rs           # Type definitions (Address, U256, etc.)
│   └── mocks.rs           # Mock implementations (only compiled with "native" feature)
├── tests/                  # Integration tests
│   ├── ledger_tests.rs   # Comprehensive ledger tests
│   └── parser_tests.rs   # Comprehensive parser tests
└── lib/                    # Vendor dependencies (included in package)
    ├── cpp-build/         # C++ library
    │   ├── include/       # C/C++ headers
    │   └── lib/          # Compiled static libraries (.a files)
    └── rust-bindings/     # Original bindings (kept for reference)
```

## Key Design Decisions

### 1. Self-Contained Package
- The `lib/` directory contains both the C++ headers and compiled libraries
- This ensures the crate is self-contained and doesn't require external dependencies
- Users can simply `cargo add cma-rust-parser` without additional setup

### 2. Feature Flags
- **`native`** (default): Uses Rust mocks for testing on macOS/development
- **`riscv64`**: Uses the real C++ library for RISC-V targets (Cartesi production)

### 3. Build System
- `build.rs` uses `bindgen` to generate Rust bindings from C headers
- Automatically links the C++ library when not using native mocks
- Paths are relative to `CARGO_MANIFEST_DIR` for portability

## How It Works

1. **During `cargo build`**:
   - `build.rs` runs and generates `bindings.rs` from `wrapper.h`
   - If `native` feature is enabled, mocks are compiled instead of linking C++ library
   - If `riscv64` feature is enabled, the C++ static library is linked

2. **User imports**:
   ```rust
   use cma_rust_parser::{Ledger, LedgerError, U256, Address, ...};
   ```

3. **All functionality** is exposed through the main `lib.rs` file

## File Sizes

The `lib/cpp-build/lib/` directory contains compiled static libraries. These are:
- Platform-specific (RISC-V for Cartesi)
- Included in the package for users targeting RISC-V
- Not used when `native` feature is enabled (default)