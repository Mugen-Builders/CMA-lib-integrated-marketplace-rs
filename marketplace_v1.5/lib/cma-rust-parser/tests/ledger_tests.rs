use cma_rust_parser::{
    Ledger,
    LedgerError,
    *,
};

/// Helper function to create a test token address
fn test_token_address() -> TokenAddress {
    let mut bytes = [0u8; 20];
    bytes[0] = 0x01;
    bytes[19] = 0xFF;
    TokenAddress::new(bytes)
}

/// Helper function to create a test account address
fn test_account_address() -> Address {
    let mut bytes = [0u8; 20];
    bytes[0] = 0xAA;
    bytes[19] = 0xBB;
    Address::new(bytes)
}

#[test]
fn test_ledger_initialization() {
    let ledger = Ledger::new();
    assert!(ledger.is_ok(), "Ledger initialization should succeed");
}

#[test]
fn test_ledger_reset() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    let result = ledger.reset();
    assert!(result.is_ok(), "Ledger reset should succeed");
}

#[test]
fn test_create_asset_by_token_address() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    let token_addr = test_token_address();
    let token_id = U256::from_u64(1);
    
    // Create an asset using token address
    let asset_id = ledger.retrieve_asset(
        None, // No existing asset_id
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Create,
    );
    
    assert!(asset_id.is_ok(), "Asset creation should succeed");
    let asset_id = asset_id.unwrap();
    assert!(asset_id.0 > 0, "Asset ID should be non-zero");
}

#[test]
fn test_find_or_create_asset() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    let token_addr = test_token_address();
    let token_id = U256::from_u64(2);
    
    // First call should create the asset
    let asset_id1 = ledger.retrieve_asset(
        None,
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::FindOrCreate,
    ).expect("Should create asset");
    
    // Second call with same parameters should find the existing asset
    let asset_id2 = ledger.retrieve_asset(
        None,
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::FindOrCreate,
    ).expect("Should find existing asset");
    
    assert_eq!(asset_id1, asset_id2, "Should return the same asset ID");
}

#[test]
fn test_create_account_by_wallet_address() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    let wallet_addr = test_account_address();
    
    // Create an account using wallet address
    let account_id = ledger.retrieve_account(
        None, // No existing account_id
        AccountType::WalletAddress,
        RetrieveOperation::Create,
        Some(wallet_addr.as_bytes()),
    );
    
    assert!(account_id.is_ok(), "Account creation should succeed");
    let account_id = account_id.unwrap();
    assert!(account_id.0 > 0, "Account ID should be non-zero");
}

#[test]
fn test_find_or_create_account() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    let wallet_addr = test_account_address();
    
    // First call should create the account
    let account_id1 = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::FindOrCreate,
        Some(wallet_addr.as_bytes()),
    ).expect("Should create account");
    
    // Second call with same address should find the existing account
    let account_id2 = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::FindOrCreate,
        Some(wallet_addr.as_bytes()),
    ).expect("Should find existing account");
    
    assert_eq!(account_id1, account_id2, "Should return the same account ID");
}

#[test]
fn test_deposit_and_balance() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    // Create an asset
    let token_addr = test_token_address();
    let token_id = U256::from_u64(100);
    let asset_id = ledger.retrieve_asset(
        None,
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Create,
    ).expect("Should create asset");
    
    // Create an account
    let wallet_addr = test_account_address();
    let account_id = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::Create,
        Some(wallet_addr.as_bytes()),
    ).expect("Should create account");
    
    // Check initial balance (should be zero)
    let initial_balance = ledger.get_balance(asset_id, account_id)
        .expect("Should get balance");
    assert_eq!(initial_balance, U256::zero(), "Initial balance should be zero");
    
    // Deposit 1000 tokens
    let deposit_amount = U256::from_u64(1000);
    let result = ledger.deposit(asset_id, account_id, deposit_amount);
    assert!(result.is_ok(), "Deposit should succeed");
    
    // Check balance after deposit
    let balance = ledger.get_balance(asset_id, account_id)
        .expect("Should get balance");
    assert_eq!(balance, deposit_amount, "Balance should match deposit amount");
    
    // Check total supply
    let total_supply = ledger.get_total_supply(asset_id)
        .expect("Should get total supply");
    assert_eq!(total_supply, deposit_amount, "Total supply should match deposit");
}

#[test]
fn test_withdraw() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    // Create asset and account
    let token_addr = test_token_address();
    let token_id = U256::from_u64(200);
    let asset_id = ledger.retrieve_asset(
        None,
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Create,
    ).expect("Should create asset");
    
    let wallet_addr = test_account_address();
    let account_id = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::Create,
        Some(wallet_addr.as_bytes()),
    ).expect("Should create account");
    
    // Deposit first
    let deposit_amount = U256::from_u64(5000);
    ledger.deposit(asset_id, account_id, deposit_amount)
        .expect("Deposit should succeed");
    
    // Withdraw 2000 tokens
    let withdraw_amount = U256::from_u64(2000);
    let result = ledger.withdraw(asset_id, account_id, withdraw_amount);
    assert!(result.is_ok(), "Withdraw should succeed");
    
    // Check balance
    let balance = ledger.get_balance(asset_id, account_id)
        .expect("Should get balance");
    let expected_balance = U256::from_u64(3000);
    assert_eq!(balance, expected_balance, "Balance should be reduced by withdrawal");
}

#[test]
fn test_insufficient_funds_error() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    // Create asset and account
    let token_addr = test_token_address();
    let token_id = U256::from_u64(300);
    let asset_id = ledger.retrieve_asset(
        None,
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Create,
    ).expect("Should create asset");
    
    let wallet_addr = test_account_address();
    let account_id = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::Create,
        Some(wallet_addr.as_bytes()),
    ).expect("Should create account");
    
    // Try to withdraw without depositing first
    let withdraw_amount = U256::from_u64(100);
    let result = ledger.withdraw(asset_id, account_id, withdraw_amount);
    
    assert!(result.is_err(), "Withdraw should fail with insufficient funds");
    match result.unwrap_err() {
        LedgerError::InsufficientFunds => {
            // Expected error
        }
        e => panic!("Expected InsufficientFunds error, got {:?}", e),
    }
}

#[test]
fn test_transfer() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    // Create asset
    let token_addr = test_token_address();
    let token_id = U256::from_u64(400);
    let asset_id = ledger.retrieve_asset(
        None,
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Create,
    ).expect("Should create asset");
    
    // Create two accounts
    let mut wallet1_bytes = [0u8; 20];
    wallet1_bytes[0] = 0x11;
    let wallet1 = Address::new(wallet1_bytes);
    let account1 = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::Create,
        Some(wallet1.as_bytes()),
    ).expect("Should create account 1");
    
    let mut wallet2_bytes = [0u8; 20];
    wallet2_bytes[0] = 0x22;
    let wallet2 = Address::new(wallet2_bytes);
    let account2 = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::Create,
        Some(wallet2.as_bytes()),
    ).expect("Should create account 2");
    
    // Deposit to account1
    let deposit_amount = U256::from_u64(10000);
    ledger.deposit(asset_id, account1, deposit_amount)
        .expect("Deposit should succeed");
    
    // Transfer 3000 from account1 to account2
    let transfer_amount = U256::from_u64(3000);
    let result = ledger.transfer(asset_id, account1, account2, transfer_amount);
    assert!(result.is_ok(), "Transfer should succeed");
    
    // Check balances
    let balance1 = ledger.get_balance(asset_id, account1)
        .expect("Should get balance 1");
    let balance2 = ledger.get_balance(asset_id, account2)
        .expect("Should get balance 2");
    
    let expected_balance1 = U256::from_u64(7000);
    assert_eq!(balance1, expected_balance1, "Account 1 balance should be reduced");
    assert_eq!(balance2, transfer_amount, "Account 2 balance should match transfer amount");
    
    // Total supply should remain the same
    let total_supply = ledger.get_total_supply(asset_id)
        .expect("Should get total supply");
    assert_eq!(total_supply, deposit_amount, "Total supply should remain unchanged");
}

#[test]
fn test_multiple_assets_and_accounts() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    // Create two different assets
    let mut token1_bytes = [0u8; 20];
    token1_bytes[0] = 0xA1;
    let token1_addr = TokenAddress::new(token1_bytes);
    let token1_id = U256::from_u64(1);
    let asset1 = ledger.retrieve_asset(
        None,
        Some(token1_addr),
        Some(token1_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Create,
    ).expect("Should create asset 1");
    
    let mut token2_bytes = [0u8; 20];
    token2_bytes[0] = 0xB2;
    let token2_addr = TokenAddress::new(token2_bytes);
    let token2_id = U256::from_u64(2);
    let asset2 = ledger.retrieve_asset(
        None,
        Some(token2_addr),
        Some(token2_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Create,
    ).expect("Should create asset 2");
    
    // Create an account
    let wallet_addr = test_account_address();
    let account_id = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::Create,
        Some(wallet_addr.as_bytes()),
    ).expect("Should create account");
    
    // Deposit different amounts to the same account for different assets
    let amount1 = U256::from_u64(100);
    let amount2 = U256::from_u64(200);
    
    ledger.deposit(asset1, account_id, amount1)
        .expect("Deposit asset1 should succeed");
    ledger.deposit(asset2, account_id, amount2)
        .expect("Deposit asset2 should succeed");
    
    // Check balances are independent
    let balance1 = ledger.get_balance(asset1, account_id)
        .expect("Should get balance 1");
    let balance2 = ledger.get_balance(asset2, account_id)
        .expect("Should get balance 2");
    
    assert_eq!(balance1, amount1, "Balance 1 should be correct");
    assert_eq!(balance2, amount2, "Balance 2 should be correct");
    
    // Check total supplies
    let supply1 = ledger.get_total_supply(asset1)
        .expect("Should get supply 1");
    let supply2 = ledger.get_total_supply(asset2)
        .expect("Should get supply 2");
    
    assert_eq!(supply1, amount1, "Supply 1 should be correct");
    assert_eq!(supply2, amount2, "Supply 2 should be correct");
}

#[test]
fn test_account_not_found_error() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    // Create asset
    let token_addr = test_token_address();
    let token_id = U256::from_u64(500);
    let asset_id = ledger.retrieve_asset(
        None,
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Create,
    ).expect("Should create asset");
    
    // Try to get balance for non-existent account
    let fake_account_id = LedgerAccountId(99999);
    let result = ledger.get_balance(asset_id, fake_account_id);
    
    assert!(result.is_err(), "Should fail for non-existent account");
    match result.unwrap_err() {
        LedgerError::AccountNotFound => {
            // Expected error
        }
        e => panic!("Expected AccountNotFound error, got {:?}", e),
    }
}

#[test]
fn test_asset_not_found_error() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    // Create account
    let wallet_addr = test_account_address();
    let account_id = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::Create,
        Some(wallet_addr.as_bytes()),
    ).expect("Should create account");
    
    // Try to get balance for non-existent asset
    let fake_asset_id = LedgerAssetId(99999);
    let result = ledger.get_balance(fake_asset_id, account_id);
    
    assert!(result.is_err(), "Should fail for non-existent asset");
    match result.unwrap_err() {
        LedgerError::AssetNotFound => {
            // Expected error
        }
        e => panic!("Expected AssetNotFound error, got {:?}", e),
    }
}

#[test]
fn test_find_nonexistent_asset() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    let token_addr = test_token_address();
    let token_id = U256::from_u64(600);
    
    // Try to find an asset that doesn't exist (without creating)
    let result = ledger.retrieve_asset(
        None,
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Find, // Only find, don't create
    );
    
    assert!(result.is_err(), "Should fail to find non-existent asset");
    match result.unwrap_err() {
        LedgerError::AssetNotFound => {
            // Expected error
        }
        e => panic!("Expected AssetNotFound error, got {:?}", e),
    }
}

#[test]
fn test_find_nonexistent_account() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    let wallet_addr = test_account_address();
    
    // Try to find an account that doesn't exist (without creating)
    let result = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::Find, // Only find, don't create
        Some(wallet_addr.as_bytes()),
    );
    
    assert!(result.is_err(), "Should fail to find non-existent account");
    match result.unwrap_err() {
        LedgerError::AccountNotFound => {
            // Expected error
        }
        e => panic!("Expected AccountNotFound error, got {:?}", e),
    }
}

#[test]
fn test_large_amounts() {
    let mut ledger = Ledger::new().expect("Failed to initialize ledger");
    
    // Create asset and account
    let token_addr = test_token_address();
    let token_id = U256::from_u64(700);
    let asset_id = ledger.retrieve_asset(
        None,
        Some(token_addr),
        Some(token_id),
        AssetType::TokenAddressId,
        RetrieveOperation::Create,
    ).expect("Should create asset");
    
    let wallet_addr = test_account_address();
    let account_id = ledger.retrieve_account(
        None,
        AccountType::WalletAddress,
        RetrieveOperation::Create,
        Some(wallet_addr.as_bytes()),
    ).expect("Should create account");
    
    // Test with a large u64 value
    let large_amount = U256::from_u64(u64::MAX);
    ledger.deposit(asset_id, account_id, large_amount)
        .expect("Should handle large amounts");
    
    let balance = ledger.get_balance(asset_id, account_id)
        .expect("Should get balance");
    assert_eq!(balance, large_amount, "Large amount should be handled correctly");
}
