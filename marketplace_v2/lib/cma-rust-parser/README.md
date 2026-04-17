# cma-rust-parser

Lightweight Rust utilities to parse Cartesi Machine Application (CMA) inputs and to build on-chain voucher payloads to be executed on the base layer.

This crate is useful for Cartesi dApp developers who need to:

- Decode deposit payloads coming from Cartesi portals (ETH, ERC20, ERC721, ERC1155).
- Decode withdrawal / transfer / ledger inspection requests encoded as JSON and hex.
- Encode voucher payloads for on-chain token transfers (Ether, ERC20, ERC721 — ERC1155 encoding TODO).

## Core public functions

- `cma_decode_advance(input: JsonValue) -> Result<CmaParserInput, CmaParserError>`  
  Detects whether an input is a portal deposit (based on the caller address) or a user instruction (withdrawal/transfer). Returns a typed `CmaParserInput` (enum + associated structs) with parsed fields (addresses, amounts, token ids, exec_layer data).
- `cma_decode_inspect(input: JsonValue) -> Result<CmaParserInput, CmaParserError>`  
  Parses inspection (ledger) requests like `ledgerGetBalance` and `ledgerGetTotalSupply` encoded as JSON-in-hex and returns structured arguments.
- `cma_encode_voucher(req_type: CmaParserVoucherType, voucher_request: CmaParserVoucherData) -> Result<CmaVoucher, CmaParserError>`  
  Builds a `CmaVoucher` (destination, value, payload) for sending to the chain. Implemented for Ether, ERC20 and ERC721 vouchers.

## Important types

- `CmaParserInputType` — enumerates recognized request types (deposits, withdrawals, transfers, ledger queries).
- `CmaParserInputData` — enum wrapping specific parsed payload structs, e.g. `EtherDeposit`, `Erc20Transfer`, `EtherWithdrawal`, etc.
- `CmaParserError` — parsing error variants and helper conversions.
- `CmaVoucher` — final voucher structure (destination, value, payload) ready for submission.

## Dependencies

- ethers-core = "1.0.0"
- hex = "0.4.3"
- json = "0.12"
- once_cell = "1.18"

## Notes and limitations

- ERC1155 voucher encoding is not implemented (placeholders return Not Implemented).
- Parser expects input JSONVALUE as received by the application, it handles necessary checks to determine the caller and possible method called.

## Contributing

- Open PRs for improvements, add missing voucher encodings, add more exhaustive parsing tests for edge cases.

## License

- MIT (see Cargo.toml)