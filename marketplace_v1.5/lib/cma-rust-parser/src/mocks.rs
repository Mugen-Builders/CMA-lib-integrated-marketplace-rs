#![cfg(feature = "native")]

use crate::bindings;
use std::collections::HashMap;
use std::cell::RefCell;
use std::ptr;

// Simple in-memory storage for mock ledger
struct MockLedgerState {
    next_asset_id: u64,
    next_account_id: u64,
    assets: HashMap<u64, AssetInfo>,
    accounts: HashMap<u64, AccountInfo>,
    balances: HashMap<(u64, u64), [u8; 32]>, // (asset_id, account_id) -> balance
    asset_lookup: HashMap<([u8; 20], [u8; 32]), u64>, // (token_addr, token_id) -> asset_id
    account_lookup: HashMap<[u8; 20], u64>, // wallet_address -> account_id
}

#[allow(dead_code)]
struct AssetInfo {
    token_address: Option<[u8; 20]>,
    token_id: Option<[u8; 32]>,
}

#[allow(dead_code)]
struct AccountInfo {
    wallet_address: Option<[u8; 20]>,
    account_id_bytes: Option<[u8; 32]>,
}

// Use thread-local storage so each test thread has its own state
thread_local! {
    static MOCK_STATE: RefCell<MockLedgerState> = RefCell::new(MockLedgerState {
        next_asset_id: 1,
        next_account_id: 1,
        assets: HashMap::new(),
        accounts: HashMap::new(),
        balances: HashMap::new(),
        asset_lookup: HashMap::new(),
        account_lookup: HashMap::new(),
    });
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_init(ledger: *mut bindings::cma_ledger_t) -> i32 {
    if ledger.is_null() {
        return bindings::CMA_LEDGER_ERROR_UNKNOWN as i32;
    }
    // Reset the mock state
    MOCK_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.next_asset_id = 1;
        s.next_account_id = 1;
        s.assets.clear();
        s.accounts.clear();
        s.balances.clear();
        s.asset_lookup.clear();
        s.account_lookup.clear();
    });
    bindings::CMA_LEDGER_SUCCESS as i32
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_fini(_ledger: *mut bindings::cma_ledger_t) -> i32 {
    bindings::CMA_LEDGER_SUCCESS as i32
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_reset(_ledger: *mut bindings::cma_ledger_t) -> i32 {
    MOCK_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.next_asset_id = 1;
        s.next_account_id = 1;
        s.assets.clear();
        s.accounts.clear();
        s.balances.clear();
        s.asset_lookup.clear();
        s.account_lookup.clear();
    });
    bindings::CMA_LEDGER_SUCCESS as i32
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_retrieve_asset(
    _ledger: *mut bindings::cma_ledger_t,
    asset_id: *mut u64,
    token_address: *mut bindings::cmt_abi_address_t,
    token_id: *mut bindings::cmt_abi_u256_t,
    asset_type: bindings::cma_ledger_asset_type_t,
    operation: bindings::cma_ledger_retrieve_operation_t,
) -> i32 {
    if asset_id.is_null() {
        return bindings::CMA_LEDGER_ERROR_UNKNOWN as i32;
    }

    MOCK_STATE.with(|state| {
        let mut s = state.borrow_mut();

        match asset_type {
            t if t == bindings::cma_ledger_asset_type_t_CMA_LEDGER_ASSET_TYPE_ID => {
                if !token_address.is_null() && !token_id.is_null() {
                    let addr = (*token_address).data;
                    let id = (*token_id).data;
                    let lookup_key = (addr, id);
                    
                    if let Some(&found_id) = s.asset_lookup.get(&lookup_key) {
                        *asset_id = found_id;
                        return bindings::CMA_LEDGER_SUCCESS as i32;
                    }
                }
            }
            t if t == bindings::cma_ledger_asset_type_t_CMA_LEDGER_ASSET_TYPE_TOKEN_ADDRESS => {
                if !token_address.is_null() {
                    let addr = (*token_address).data;
                    // Simple lookup by address only
                    for (&id, info) in &s.assets {
                        if let Some(ref stored_addr) = info.token_address {
                            if stored_addr == &addr {
                                *asset_id = id;
                                return bindings::CMA_LEDGER_SUCCESS as i32;
                            }
                        }
                    }
                }
            }
            t if t == bindings::cma_ledger_asset_type_t_CMA_LEDGER_ASSET_TYPE_TOKEN_ADDRESS_ID => {
                if !token_address.is_null() && !token_id.is_null() {
                    let addr = (*token_address).data;
                    let id = (*token_id).data;
                    let lookup_key = (addr, id);
                    
                    if let Some(&found_id) = s.asset_lookup.get(&lookup_key) {
                        *asset_id = found_id;
                        return bindings::CMA_LEDGER_SUCCESS as i32;
                    }
                }
            }
            _ => {}
        }

        match operation {
            op if op == bindings::cma_ledger_retrieve_operation_t_CMA_LEDGER_OP_FIND => {
                return bindings::CMA_LEDGER_ERROR_ASSET_NOT_FOUND as i32;
            }
            op if op == bindings::cma_ledger_retrieve_operation_t_CMA_LEDGER_OP_CREATE
                || op == bindings::cma_ledger_retrieve_operation_t_CMA_LEDGER_OP_FIND_OR_CREATE =>
            {
                let new_id = s.next_asset_id;
                s.next_asset_id += 1;

                let token_addr = if !token_address.is_null() {
                    Some((*token_address).data)
                } else {
                    None
                };

                let token_id_bytes = if !token_id.is_null() {
                    Some((*token_id).data)
                } else {
                    None
                };

                s.assets.insert(
                    new_id,
                    AssetInfo {
                        token_address: token_addr,
                        token_id: token_id_bytes,
                    },
                );

                if let (Some(addr), Some(id)) = (token_addr, token_id_bytes) {
                    s.asset_lookup.insert((addr, id), new_id);
                }

                *asset_id = new_id;
                bindings::CMA_LEDGER_SUCCESS as i32
            }
            _ => bindings::CMA_LEDGER_ERROR_UNKNOWN as i32,
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_retrieve_account(
    _ledger: *mut bindings::cma_ledger_t,
    account_id: *mut u64,
    _account: *mut bindings::cma_ledger_account_t,
    addr_or_id: *const std::ffi::c_void,
    account_type: bindings::cma_ledger_account_type_t,
    operation: bindings::cma_ledger_retrieve_operation_t,
) -> i32 {
    if account_id.is_null() {
        return bindings::CMA_LEDGER_ERROR_UNKNOWN as i32;
    }

    MOCK_STATE.with(|state| {
        let mut s = state.borrow_mut();

        if account_type == bindings::cma_ledger_account_type_t_CMA_LEDGER_ACCOUNT_TYPE_WALLET_ADDRESS
            && !addr_or_id.is_null()
        {
            let addr_bytes = std::slice::from_raw_parts(addr_or_id as *const u8, 20);
            let mut addr = [0u8; 20];
            addr.copy_from_slice(addr_bytes);

            if let Some(&found_id) = s.account_lookup.get(&addr) {
                *account_id = found_id;
                return bindings::CMA_LEDGER_SUCCESS as i32;
            }

            match operation {
                op if op == bindings::cma_ledger_retrieve_operation_t_CMA_LEDGER_OP_FIND => {
                    return bindings::CMA_LEDGER_ERROR_ACCOUNT_NOT_FOUND as i32;
                }
                op if op == bindings::cma_ledger_retrieve_operation_t_CMA_LEDGER_OP_CREATE
                    || op == bindings::cma_ledger_retrieve_operation_t_CMA_LEDGER_OP_FIND_OR_CREATE =>
                {
                    let new_id = s.next_account_id;
                    s.next_account_id += 1;

                    s.accounts.insert(
                        new_id,
                        AccountInfo {
                            wallet_address: Some(addr),
                            account_id_bytes: None,
                        },
                    );
                    s.account_lookup.insert(addr, new_id);

                    *account_id = new_id;
                    return bindings::CMA_LEDGER_SUCCESS as i32;
                }
                _ => {}
            }
        }

        bindings::CMA_LEDGER_ERROR_UNKNOWN as i32
    })
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_deposit(
    _ledger: *mut bindings::cma_ledger_t,
    asset_id: u64,
    to_account_id: u64,
    deposit: *const bindings::cmt_abi_u256_t,
) -> i32 {
    if deposit.is_null() {
        return bindings::CMA_LEDGER_ERROR_UNKNOWN as i32;
    }

    MOCK_STATE.with(|state| {
        let mut s = state.borrow_mut();

        if !s.assets.contains_key(&asset_id) {
            return bindings::CMA_LEDGER_ERROR_ASSET_NOT_FOUND as i32;
        }

        if !s.accounts.contains_key(&to_account_id) {
            return bindings::CMA_LEDGER_ERROR_ACCOUNT_NOT_FOUND as i32;
        }

        let deposit_amount = (*deposit).data;
        let key = (asset_id, to_account_id);
        let current_balance = s.balances.get(&key).copied().unwrap_or([0u8; 32]);

        // Simple addition (big-endian)
        let mut new_balance = [0u8; 32];
        let mut carry = 0u16;
        for i in (0..32).rev() {
            let sum = current_balance[i] as u16 + deposit_amount[i] as u16 + carry;
            new_balance[i] = sum as u8;
            carry = sum >> 8;
        }

        s.balances.insert(key, new_balance);
        bindings::CMA_LEDGER_SUCCESS as i32
    })
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_withdraw(
    _ledger: *mut bindings::cma_ledger_t,
    asset_id: u64,
    from_account_id: u64,
    withdrawal: *const bindings::cmt_abi_u256_t,
) -> i32 {
    if withdrawal.is_null() {
        return bindings::CMA_LEDGER_ERROR_UNKNOWN as i32;
    }

    MOCK_STATE.with(|state| {
        let mut s = state.borrow_mut();

        if !s.assets.contains_key(&asset_id) {
            return bindings::CMA_LEDGER_ERROR_ASSET_NOT_FOUND as i32;
        }

        if !s.accounts.contains_key(&from_account_id) {
            return bindings::CMA_LEDGER_ERROR_ACCOUNT_NOT_FOUND as i32;
        }

        let withdrawal_amount = (*withdrawal).data;
        let key = (asset_id, from_account_id);
        let current_balance = s.balances.get(&key).copied().unwrap_or([0u8; 32]);

        // Simple subtraction (big-endian) with overflow check
        let mut new_balance = [0u8; 32];
        let mut borrow = 0i16;
        for i in (0..32).rev() {
            let diff = current_balance[i] as i16 - withdrawal_amount[i] as i16 - borrow;
            if diff < 0 {
                borrow = 1;
                new_balance[i] = (diff + 256) as u8;
            } else {
                borrow = 0;
                new_balance[i] = diff as u8;
            }
        }

        if borrow > 0 {
            return bindings::CMA_LEDGER_ERROR_INSUFFICIENT_FUNDS as i32;
        }

        s.balances.insert(key, new_balance);
        bindings::CMA_LEDGER_SUCCESS as i32
    })
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_transfer(
    _ledger: *mut bindings::cma_ledger_t,
    asset_id: u64,
    from_account_id: u64,
    to_account_id: u64,
    amount: *const bindings::cmt_abi_u256_t,
) -> i32 {
    if amount.is_null() {
        return bindings::CMA_LEDGER_ERROR_UNKNOWN as i32;
    }

    // Withdraw from source
    let withdraw_result = cma_ledger_withdraw(_ledger, asset_id, from_account_id, amount);
    if withdraw_result < 0 {
        return withdraw_result;
    }

    // Deposit to destination
    cma_ledger_deposit(_ledger, asset_id, to_account_id, amount)
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_get_balance(
    _ledger: *const bindings::cma_ledger_t,
    asset_id: u64,
    account_id: u64,
    out_balance: *mut bindings::cmt_abi_u256_t,
) -> i32 {
    if out_balance.is_null() {
        return bindings::CMA_LEDGER_ERROR_UNKNOWN as i32;
    }

    MOCK_STATE.with(|state| {
        let s = state.borrow();

        if !s.assets.contains_key(&asset_id) {
            return bindings::CMA_LEDGER_ERROR_ASSET_NOT_FOUND as i32;
        }

        if !s.accounts.contains_key(&account_id) {
            return bindings::CMA_LEDGER_ERROR_ACCOUNT_NOT_FOUND as i32;
        }

        let key = (asset_id, account_id);
        let balance = s.balances.get(&key).copied().unwrap_or([0u8; 32]);
        (*out_balance).data = balance;

        bindings::CMA_LEDGER_SUCCESS as i32
    })
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_get_total_supply(
    _ledger: *const bindings::cma_ledger_t,
    asset_id: u64,
    out_total_supply: *mut bindings::cmt_abi_u256_t,
) -> i32 {
    if out_total_supply.is_null() {
        return bindings::CMA_LEDGER_ERROR_UNKNOWN as i32;
    }

    MOCK_STATE.with(|state| {
        let s = state.borrow();

        if !s.assets.contains_key(&asset_id) {
            return bindings::CMA_LEDGER_ERROR_ASSET_NOT_FOUND as i32;
        }

        // Sum all balances for this asset
        let mut total = [0u8; 32];
        for ((a_id, _), balance) in &s.balances {
            if *a_id == asset_id {
                // Add balance to total (big-endian addition)
                let mut carry = 0u16;
                for i in (0..32).rev() {
                    let sum = total[i] as u16 + balance[i] as u16 + carry;
                    total[i] = sum as u8;
                    carry = sum >> 8;
                }
            }
        }

        (*out_total_supply).data = total;
        bindings::CMA_LEDGER_SUCCESS as i32
    })
}

#[no_mangle]
pub unsafe extern "C" fn cma_ledger_get_last_error_message() -> *const std::ffi::c_char {
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn cma_parser_decode_advance(
    _input_type: bindings::cma_parser_input_type_t,
    _input: *const bindings::cmt_rollup_advance_t,
    out: *mut bindings::cma_parser_input_t,
) -> i32 {
    if !out.is_null() {
        (*out).type_ =
            bindings::cma_parser_input_type_t_CMA_PARSER_INPUT_TYPE_ETHER_DEPOSIT;

        // union access generated by bindgen:
        let dep = &mut (*out).__bindgen_anon_1.ether_deposit;

        dep.sender.data.fill(0);
        dep.sender.data[19] = 1;

        dep.amount.data.fill(0);
        dep.amount.data[31] = 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn cma_parser_get_last_error_message() -> *const std::ffi::c_char {
    ptr::null()
}
