use crate::bindings;
use crate::error::LedgerError;
use crate::types::*;
use std::ptr;

/// Safe wrapper around the C ledger
pub struct Ledger {
    inner: bindings::cma_ledger_t,
}

impl Ledger {
    /// Initialize a new ledger
    pub fn new() -> Result<Self, LedgerError> {
        unsafe {
            let mut ledger = std::mem::zeroed::<bindings::cma_ledger_t>();
            let result = bindings::cma_ledger_init(&mut ledger);
            if result < 0 {
                return Err(LedgerError::from_code(result));
            }
            Ok(Ledger { inner: ledger })
        }
    }

    /// Reset the ledger
    pub fn reset(&mut self) -> Result<(), LedgerError> {
        unsafe {
            let result = bindings::cma_ledger_reset(&mut self.inner);
            if result < 0 {
                return Err(LedgerError::from_code(result));
            }
            Ok(())
        }
    }

    /// Retrieve or create an asset
    pub fn retrieve_asset(
        &mut self,
        asset_id: Option<LedgerAssetId>,
        token_address: Option<TokenAddress>,
        token_id: Option<TokenId>,
        asset_type: AssetType,
        operation: RetrieveOperation,
    ) -> Result<LedgerAssetId, LedgerError> {
        unsafe {
            let mut out_asset_id = asset_id.map(|id| id.0).unwrap_or(0);
            let mut out_token_address = token_address
                .map(|addr| addr.to_c())
                .unwrap_or_else(|| bindings::cmt_abi_address_t { data: [0u8; 20] });
            let mut out_token_id = token_id
                .map(|id| id.to_c())
                .unwrap_or_else(|| bindings::cmt_abi_u256_t { data: [0u8; 32] });

            let result = bindings::cma_ledger_retrieve_asset(
                &mut self.inner,
                &mut out_asset_id,
                if token_address.is_some() {
                    &mut out_token_address
                } else {
                    ptr::null_mut()
                },
                if token_id.is_some() {
                    &mut out_token_id
                } else {
                    ptr::null_mut()
                },
                asset_type.to_c(),
                operation.to_c(),
            );

            if result < 0 {
                return Err(LedgerError::from_code(result));
            }

            Ok(LedgerAssetId(out_asset_id))
        }
    }

    pub fn retrieve_erc20_asset_via_address(&mut self, token_address: Address) -> Result<LedgerAssetId, LedgerError> {
        let asset_id = self.retrieve_asset(
            None,
            Some(token_address),
            None,
            AssetType::TokenAddress,
            RetrieveOperation::FindOrCreate,
        ).map_err(|e| e)?;

        return Ok(asset_id)
    }

    pub fn retrieve_erc721_assets_via_address(&mut self, token_address: Address, token_id: U256) -> Result<LedgerAssetId, LedgerError> {
        let asset_id = self.retrieve_asset(
            None,
            Some(token_address),
            Some(token_id),
            AssetType::TokenAddressId,
            RetrieveOperation::FindOrCreate,
        ).map_err(|e| e)?;

        return Ok(asset_id)
    }

    pub fn retrieve_ether_assets(&mut self) -> Result<LedgerAssetId, LedgerError> {
        let asset_id = self.retrieve_asset(
            None,
            None,
            None,
            AssetType::TokenAddressId,
            RetrieveOperation::FindOrCreate,
        ).map_err(|e| e)?;

        return Ok(asset_id)
    }

    /// Retrieve or create an account
    pub fn retrieve_account(
        &mut self,
        account_id: Option<LedgerAccountId>,
        account_type: AccountType,
        operation: RetrieveOperation,
        addr_or_id: Option<&[u8]>,
    ) -> Result<LedgerAccountId, LedgerError> {
        unsafe {
            let mut out_account_id = account_id.map(|id| id.0).unwrap_or(0);
            let mut c_account = std::mem::zeroed::<bindings::cma_ledger_account_t>();
            c_account.type_ = account_type.to_c();

            let addr_ptr = if let Some(addr) = addr_or_id {
                addr.as_ptr() as *const std::ffi::c_void
            } else {
                ptr::null()
            };

            let result = bindings::cma_ledger_retrieve_account(
                &mut self.inner,
                &mut out_account_id,
                &mut c_account,
                addr_ptr,
                account_type.to_c(),
                operation.to_c(),
            );

            if result < 0 {
                return Err(LedgerError::from_code(result));
            }

            Ok(LedgerAccountId(out_account_id))
        }
    }

    /// Simple Retrieve or create account
    pub fn retrieve_account_via_address(&mut self, account_address: Address) -> Result<LedgerAccountId, LedgerError> {
        let account_id = self.retrieve_account(
            None,
            AccountType::WalletAddress,
            RetrieveOperation::FindOrCreate,
            Some(account_address.as_bytes()),
        ).map_err(|e| e)?;

        return Ok(account_id)
    }



    /// Deposit assets to an account
    pub fn deposit(
        &mut self,
        asset_id: LedgerAssetId,
        to_account_id: LedgerAccountId,
        amount: U256,
    ) -> Result<(), LedgerError> {
        unsafe {
            let c_amount = amount.to_c();
            let result = bindings::cma_ledger_deposit(
                &mut self.inner,
                asset_id.0,
                to_account_id.0,
                &c_amount,
            );

            if result < 0 {
                return Err(LedgerError::from_code(result));
            }

            Ok(())
        }
    }

    /// Withdraw assets from an account
    pub fn withdraw(
        &mut self,
        asset_id: LedgerAssetId,
        from_account_id: LedgerAccountId,
        amount: U256,
    ) -> Result<(), LedgerError> {
        unsafe {
            let c_amount = amount.to_c();
            let result = bindings::cma_ledger_withdraw(
                &mut self.inner,
                asset_id.0,
                from_account_id.0,
                &c_amount,
            );

            if result < 0 {
                return Err(LedgerError::from_code(result));
            }

            Ok(())
        }
    }

    /// Transfer assets between accounts
    pub fn transfer(
        &mut self,
        asset_id: LedgerAssetId,
        from_account_id: LedgerAccountId,
        to_account_id: LedgerAccountId,
        amount: U256,
    ) -> Result<(), LedgerError> {
        unsafe {
            let c_amount = amount.to_c();
            let result = bindings::cma_ledger_transfer(
                &mut self.inner,
                asset_id.0,
                from_account_id.0,
                to_account_id.0,
                &c_amount,
            );

            if result < 0 {
                return Err(LedgerError::from_code(result));
            }

            Ok(())
        }
    }

    /// Get balance for an account
    pub fn get_balance(
        &self,
        asset_id: LedgerAssetId,
        account_id: LedgerAccountId,
    ) -> Result<U256, LedgerError> {
        unsafe {
            let mut out_balance = std::mem::zeroed::<bindings::cmt_abi_u256_t>();
            let result = bindings::cma_ledger_get_balance(
                &self.inner as *const _ as *mut _,
                asset_id.0,
                account_id.0,
                &mut out_balance,
            );

            if result < 0 {
                return Err(LedgerError::from_code(result));
            }

            Ok(U256::from_c(&out_balance))
        }
    }

    /// Get total supply for an asset
    pub fn get_total_supply(&self, asset_id: LedgerAssetId) -> Result<U256, LedgerError> {
        unsafe {
            let mut out_supply = std::mem::zeroed::<bindings::cmt_abi_u256_t>();
            let result = bindings::cma_ledger_get_total_supply(
                &self.inner as *const _ as *mut _,
                asset_id.0,
                &mut out_supply,
            );

            if result < 0 {
                return Err(LedgerError::from_code(result));
            }

            Ok(U256::from_c(&out_supply))
        }
    }
}

impl Drop for Ledger {
    fn drop(&mut self) {
        unsafe {
            bindings::cma_ledger_fini(&mut self.inner);
        }
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ledger")
    }
}
