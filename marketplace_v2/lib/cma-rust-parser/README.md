# cma-rust-parser

Rust bindings for Cartesi Machine Asset Tools (CMA) - a ledger and parser for managing assets on Cartesi rollups.

## Features

- **Ledger**: Manage assets, accounts, deposits, withdrawals, and transfers
- **Parser**: Decode rollup inputs and encode vouchers for various token types (Ether, ERC20, ERC721, ERC1155)
- **Type-safe**: Rust types for addresses, U256 values, and all ledger operations
- **Error handling**: Comprehensive error types for all operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cma-rust-parser = "0.1.0"
```

## Usage

### Type Compatibility

**Important**: `Address` and `U256` are re-exported from `ethers_core::types` for convenience. They are the **same types** as those in `ethers_core`, so you can import them from either place:

```rust
// Recommended: Import from cma_rust_parser for consistency
use cma_rust_parser::{Address, U256};

// Also works: Import directly from ethers_core
use ethers_core::types::{Address, U256};
```

Both approaches work, but importing from `cma_rust_parser` ensures you're using the same types that the library's extension traits (`AddressCBindingsExt`, `U256CBindingsExt`) are implemented for.

### Ledger Operations

```rust
use cma_rust_parser::{Ledger, LedgerError, AssetType, RetrieveOperation, AccountType, Address, U256};

// Initialize a ledger
let mut ledger = Ledger::new()?;

// Create an asset
let token_address = Address::new([0u8; 20]); // Your token address
let token_id = U256::from_u64(1);
let asset_id = ledger.retrieve_asset(
    None,
    Some(token_address),
    Some(token_id),
    AssetType::TokenAddressId,
    RetrieveOperation::Create,
)?;

// Create an account
let wallet_address = Address::new([0u8; 20]); // Wallet address
let account_id = ledger.retrieve_account(
    None,
    AccountType::WalletAddress,
    RetrieveOperation::Create,
    Some(wallet_address.as_bytes()),
)?;

// Deposit tokens
let amount = U256::from_u64(1000);
ledger.deposit(asset_id, account_id, amount)?;

// Check balance
let balance = ledger.get_balance(asset_id, account_id)?;

// Transfer tokens
let recipient_id = ledger.retrieve_account(
    None,
    AccountType::WalletAddress,
    RetrieveOperation::Create,
    Some(recipient_address.as_bytes()),
)?;
ledger.transfer(asset_id, account_id, recipient_id, U256::from_u64(100))?;
```

### Parser Operations

```rust
use cma_rust_parser::{Parser, ParserInputType, RollupAdvance};

let parser = Parser::new();

// Decode an advance input
let input_type = ParserInputType::EtherDeposit;
let parsed = parser.decode_advance(input_type, &advance_input)?;

// Encode a voucher
let voucher = parser.encode_voucher(voucher_type, app_address, voucher_data)?;
```

## Features

- `native`: Use mock implementations for testing on macOS (default)
- `riscv64`: Use the real C++ library for RISC-V targets (Cartesi)

## Publishing to crates.io

To publish this crate to crates.io:

1. **Update metadata in `Cargo.toml`**:
   - Set the correct `authors` field
   - Update `repository` URL
   - Ensure `version` follows semantic versioning

2. **Create LICENSE files** (if not already present):
   ```bash
   # Add Apache-2.0 and/or MIT licenses
   ```

3. **Test the package**:
   ```bash
   cargo test
   cargo build --release
   ```

4. **Check the package**:
   ```bash
   cargo package
   cargo publish --dry-run
   ```

5. **Publish**:
   ```bash
   cargo publish
   ```

## Repository Structure

```
cma-rust-parser/
├── Cargo.toml          # Main crate configuration
├── build.rs            # Build script for bindgen
├── wrapper.h           # C header wrapper
├── src/                # Rust source code
│   ├── lib.rs         # Library root
│   ├── error.rs       # Error types
│   ├── ledger.rs      # Ledger implementation
│   ├── types.rs       # Type definitions
│   └── mocks.rs       # Mock implementations (native feature)
├── tests/             # Integration tests
├── lib/               # Vendor dependencies
│   ├── cpp-build/     # C++ library headers and binaries
│   └── rust-bindings/ # Original bindings (for reference)
└── README.md          # This file
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
