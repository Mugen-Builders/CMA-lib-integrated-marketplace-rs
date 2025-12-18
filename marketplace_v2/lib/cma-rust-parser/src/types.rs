use crate::bindings;
// use std::fmt;
pub use ethers_core::types::{Address, U256};
use std::str::FromStr;

pub const ADDRESS_LENGTH: usize = 20;
pub const U256_LENGTH: usize = 32;

pub trait AddressCBindingsExt {
    fn new(bytes: [u8; ADDRESS_LENGTH]) -> Self;
    fn from_str_hex(s: &str) -> Result<Self, String>
    where
        Self: Sized;
    fn from_slice(slice: &[u8]) -> Result<Self, String>
    where
        Self: Sized;
    fn as_array(&self) -> [u8; ADDRESS_LENGTH];
    fn as_bytes(&self) -> &[u8; ADDRESS_LENGTH];
    fn to_c(&self) -> bindings::cmt_abi_address_t;
    fn from_c(c_addr: &bindings::cmt_abi_address_t) -> Self;
}

impl AddressCBindingsExt for Address {
    fn new(bytes: [u8; ADDRESS_LENGTH]) -> Self {
        Address::from(bytes)
    }

    fn from_str_hex(s: &str) -> Result<Self, String> {
        Address::from_str(s).map_err(|e| e.to_string())
    }

    fn from_slice(slice: &[u8]) -> Result<Self, String> {
        if slice.len() != ADDRESS_LENGTH {
            return Err(format!(
                "Address must be {} bytes, got {}",
                ADDRESS_LENGTH,
                slice.len()
            ));
        }
        Ok(Address::from_slice(slice))
    }

    fn as_array(&self) -> [u8; ADDRESS_LENGTH] {
        self.0
    }

    fn as_bytes(&self) -> &[u8; ADDRESS_LENGTH] {
        &self.0
    }

    fn to_c(&self) -> bindings::cmt_abi_address_t {
        bindings::cmt_abi_address_t { data: self.as_array() }
    }

    fn from_c(c_addr: &bindings::cmt_abi_address_t) -> Self {
        Address::from(c_addr.data)
    }
}

pub trait U256CBindingsExt {
    fn new(bytes: [u8; U256_LENGTH]) -> Self
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized;
    fn from_u64(value: u64) -> Self
    where
        Self: Sized;
    fn from_be_bytes(bytes: [u8; U256_LENGTH]) -> Self
    where
        Self: Sized;
    fn from_slice(slice: &[u8]) -> Result<Self, String>
    where
        Self: Sized;
    fn as_be_bytes(&self) -> [u8; U256_LENGTH];
    fn to_u128_opt(&self) -> Option<u128>;
    fn to_c(&self) -> bindings::cmt_abi_u256_t;
    fn from_c(c_u256: &bindings::cmt_abi_u256_t) -> Self;
}

impl U256CBindingsExt for U256 {
    fn new(bytes: [u8; U256_LENGTH]) -> Self {
        U256::from_big_endian(&bytes)
    }

    fn zero() -> Self {
        U256::zero()
    }

    fn from_u64(value: u64) -> Self {
        U256::from(value)
    }

    fn from_be_bytes(bytes: [u8; U256_LENGTH]) -> Self {
        U256::from_big_endian(&bytes)
    }

    fn from_slice(slice: &[u8]) -> Result<Self, String> {
        if slice.len() != U256_LENGTH {
            return Err(format!("U256 must be 32 bytes, got {}", slice.len()));
        }
        Ok(U256::from_big_endian(slice))
    }

    fn as_be_bytes(&self) -> [u8; U256_LENGTH] {
        let mut out = [0u8; U256_LENGTH];
        self.to_big_endian(&mut out);
        out
    }

    fn to_u128_opt(&self) -> Option<u128> {
        if self.bits() > 128 {
            None
        } else {
            Some(self.low_u128())
        }
    }

    fn to_c(&self) -> bindings::cmt_abi_u256_t {
        bindings::cmt_abi_u256_t { data: self.as_be_bytes() }
    }

    fn from_c(c_u256: &bindings::cmt_abi_u256_t) -> Self {
        U256::from_big_endian(&c_u256.data)
    }
}

// /// Ethereum address (20 bytes)
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Address([u8; 20]);

// impl Address {
//     pub const LENGTH: usize = 20;

//     pub fn from_str(s: &str) -> Result<Self, String> {
//         let s = s.strip_prefix("0x").unwrap_or(s);
//         if s.len() != 40 {
//             return Err(format!("Address hex string must be 40 characters, got {}", s.len()));
//         }
//         let bytes = hex::decode(s).map_err(|e| format!("Invalid hex string: {}", e))?;
//         Self::from_slice(&bytes)
//     }

//     pub fn new(bytes: [u8; 20]) -> Self {
//         Address(bytes)
//     }

//     pub fn as_bytes(&self) -> &[u8; 20] {
//         &self.0
//     }

//     pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
//         if slice.len() != 20 {
//             return Err(format!("Address must be 20 bytes, got {}", slice.len()));
//         }
//         let mut bytes = [0u8; 20];
//         bytes.copy_from_slice(slice);
//         Ok(Address(bytes))
//     }

//     pub fn to_c(&self) -> bindings::cmt_abi_address_t {
//         bindings::cmt_abi_address_t { data: self.0 }
//     }

//     pub fn from_c(c_addr: &bindings::cmt_abi_address_t) -> Self {
//         Address(c_addr.data)
//     }
// }

// impl fmt::Display for Address {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "0x{}", hex::encode(self.0))
//     }
// }

// /// U256 value (32 bytes, big-endian)
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct U256([u8; 32]);

// impl U256 {
//     pub const LENGTH: usize = 32;

//     pub fn new(bytes: [u8; 32]) -> Self {
//         U256(bytes)
//     }

//     pub fn zero() -> Self {
//         U256([0u8; 32])
//     }

//     pub fn from_u64(value: u64) -> Self {
//         let mut bytes = [0u8; 32];
//         // Big-endian encoding
//         bytes[24..32].copy_from_slice(&value.to_be_bytes());
//         U256(bytes)
//     }

//     pub fn to_u128(&self) -> Option<u128> {
//         // Check if the value fits in u128 (lower 16 bytes must be zero)
//         if self.0[..16].iter().any(|&b| b != 0) {
//             return None;
//         }
//         let mut bytes = [0u8; 16];
//         bytes.copy_from_slice(&self.0[16..32]);
//         Some(u128::from_be_bytes(bytes))
//     }

//     pub fn as_bytes(&self) -> &[u8; 32] {
//         &self.0
//     }

//     pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
//         if slice.len() != 32 {
//             return Err(format!("U256 must be 32 bytes, got {}", slice.len()));
//         }
//         let mut bytes = [0u8; 32];
//         bytes.copy_from_slice(slice);
//         Ok(U256(bytes))
//     }

//     pub fn to_c(&self) -> bindings::cmt_abi_u256_t {
//         bindings::cmt_abi_u256_t { data: self.0 }
//     }

//     pub fn from_c(c_u256: &bindings::cmt_abi_u256_t) -> Self {
//         U256(c_u256.data)
//     }
// }

// impl fmt::Display for U256 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "0x{}", hex::encode(self.0))
//     }
// }

/// Account ID (32 bytes)
pub type AccountId = U256;

/// Token ID (32 bytes)
pub type TokenId = U256;

/// Token address (20 bytes)
pub type TokenAddress = Address;

/// Ledger account ID (64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LedgerAccountId(pub u64);

/// Ledger asset ID (64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LedgerAssetId(pub u64);

/// Ledger retrieve operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetrieveOperation {
    Find,
    Create,
    FindOrCreate,
}

impl RetrieveOperation {
    pub fn to_c(&self) -> bindings::cma_ledger_retrieve_operation_t {
        match self {
            RetrieveOperation::Find => bindings::cma_ledger_retrieve_operation_t_CMA_LEDGER_OP_FIND,
            RetrieveOperation::Create => {
                bindings::cma_ledger_retrieve_operation_t_CMA_LEDGER_OP_CREATE
            }
            RetrieveOperation::FindOrCreate => {
                bindings::cma_ledger_retrieve_operation_t_CMA_LEDGER_OP_FIND_OR_CREATE
            }
        }
    }
}

/// Asset type for ledger operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Id,
    TokenAddress,
    TokenAddressId,
}

impl AssetType {
    pub fn to_c(&self) -> bindings::cma_ledger_asset_type_t {
        match self {
            AssetType::Id => bindings::cma_ledger_asset_type_t_CMA_LEDGER_ASSET_TYPE_ID,
            AssetType::TokenAddress => {
                bindings::cma_ledger_asset_type_t_CMA_LEDGER_ASSET_TYPE_TOKEN_ADDRESS
            }
            AssetType::TokenAddressId => {
                bindings::cma_ledger_asset_type_t_CMA_LEDGER_ASSET_TYPE_TOKEN_ADDRESS_ID
            }
        }
    }
}

/// Account type for ledger operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
    Id,
    WalletAddress,
    AccountId,
}

impl AccountType {
    pub fn to_c(&self) -> bindings::cma_ledger_account_type_t {
        match self {
            AccountType::Id => bindings::cma_ledger_account_type_t_CMA_LEDGER_ACCOUNT_TYPE_ID,
            AccountType::WalletAddress => {
                bindings::cma_ledger_account_type_t_CMA_LEDGER_ACCOUNT_TYPE_WALLET_ADDRESS
            }
            AccountType::AccountId => {
                bindings::cma_ledger_account_type_t_CMA_LEDGER_ACCOUNT_TYPE_ACCOUNT_ID
            }
        }
    }
}

/// Helper to convert bytes to C bytes struct
pub fn bytes_to_c_bytes(data: &[u8]) -> bindings::cmt_abi_bytes_t {
    bindings::cmt_abi_bytes_t {
        data: data.as_ptr() as *mut std::ffi::c_void,
        length: data.len(),
    }
}

/// Helper to safely extract bytes from C bytes struct
pub unsafe fn c_bytes_to_vec(c_bytes: &bindings::cmt_abi_bytes_t) -> Vec<u8> {
    if c_bytes.data.is_null() || c_bytes.length == 0 {
        return Vec::new();
    }
    std::slice::from_raw_parts(c_bytes.data as *const u8, c_bytes.length).to_vec()
}

pub type CmaAccountId = String;
