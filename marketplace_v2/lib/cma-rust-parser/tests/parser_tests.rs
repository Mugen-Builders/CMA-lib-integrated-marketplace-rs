use ethers_core::abi::{AbiParser, encode, FixedBytes, Token};
use cma_rust_parser::parser::{
    cma_decode_advance, cma_decode_inspect, cma_encode_voucher, CmaParserErc20VoucherFields,
    CmaParserErc721VoucherFields, CmaParserEtherVoucherFields, CmaParserInputData,
    CmaParserInputType, CmaParserVoucherType, CmaVoucherFieldType,
};
use cma_rust_parser::helpers::{PortalMatcher, Portals, CARTESI_ADDRESSES};
use ethers_core::types::{Address, U256};
use ethers_core::utils::{id};
use json::JsonValue;

// Helper to create a basic JsonValue input structure
fn create_test_input(msg_sender: &str, payload: &str) -> JsonValue {
    let mut input = JsonValue::new_object();
    input["data"]["metadata"]["msg_sender"] = msg_sender.into();
    input["data"]["payload"] = payload.into();
    input
}

pub fn abi_encode_call(
    signature: &str,
    args: Vec<Token>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let function = AbiParser::default().parse_function(signature)?;
    let calldata = function.encode_input(&args)?;
    Ok(calldata)
}

#[test]
fn test_ether_deposit_success() {
    let sender: Address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        .parse()
        .unwrap();
    let amount = U256::from_dec_str("2000000000000000000").unwrap(); // 2 Ether in wei
    let payload = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb922660000000000000000000000000000000000000000000000001bc16d674ec80000".to_string(); // Sample payload from the ether portal

    let input = create_test_input(
        CARTESI_ADDRESSES
            .get_portal_address(Portals::EtherPortal)
            .unwrap(),
        &payload,
    );

    match cma_decode_advance(CmaParserInputType::CmaParserInputTypeEtherDeposit, input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeEtherDeposit;
            let is_correct_sender = if let CmaParserInputData::EtherDeposit(deposit) = result.input
            {
                // println!("Deposit sender: {:?}, amount: {}, input_amount: {amount}, input_address: {sender}", deposit.sender, deposit.amount);
                if deposit.amount == amount && deposit.sender == sender {
                    true
                } else {
                    false
                }
            } else {
                false
            };

            assert_eq!(true, is_correct_method, "Expected Ether Deposit method");
            assert_eq!(
                true, is_correct_sender,
                "Expected correct sender and amount"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_erc20_deposit_success() {
    let sender: Address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        .parse()
        .unwrap();
    let amount = U256::from_dec_str("300000000000000000000").unwrap(); // 300 ERC20 tokens in wei
    let token_address: Address = "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C"
        .parse()
        .unwrap(); // TEST token address
    let payload = "0xfbdb734ef6a23ad76863cba6f10d0c5cbbd8342cf39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000000000000000000000000001043561a8829300000".to_string(); // Sample payload from the ERC20 portal

    let input = create_test_input(
        CARTESI_ADDRESSES
            .get_portal_address(Portals::ERC20Portal)
            .unwrap(),
        &payload,
    );

    match cma_decode_advance(CmaParserInputType::CmaParserInputTypeErc20Deposit, input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeErc20Deposit;
            let is_correct_sender = if let CmaParserInputData::Erc20Deposit(deposit) = result.input
            {
                if deposit.amount == amount
                    && deposit.sender == sender
                    && deposit.token == token_address
                {
                    true
                } else {
                    false
                }
            } else {
                false
            };

            assert_eq!(true, is_correct_method, "Expected ERC20 Deposit method");
            assert_eq!(
                true, is_correct_sender,
                "Expected correct sender, token address and amount"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_erc721_deposit_success() {
    let sender: Address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        .parse()
        .unwrap();
    let token_id = U256::from_dec_str("1").unwrap(); // Sample token ID
    let token_address: Address = "0xBa46623aD94AB45850c4ecbA9555D26328917c3B"
        .parse()
        .unwrap(); // Sample ERC721 token address
    let payload = "0xba46623ad94ab45850c4ecba9555d26328917c3bf39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string(); // Sample payload from the ERC721 portal

    let input = create_test_input(
        CARTESI_ADDRESSES
            .get_portal_address(Portals::ERC721Portal)
            .unwrap(),
        &payload,
    );
    match cma_decode_advance(CmaParserInputType::CmaParserInputTypeErc721Deposit, input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeErc721Deposit;
            let is_correct_sender = if let CmaParserInputData::Erc721Deposit(deposit) = result.input
            {
                if deposit.token_id == token_id
                    && deposit.sender == sender
                    && deposit.token == token_address
                {
                    true
                } else {
                    false
                }
            } else {
                false
            };

            assert_eq!(true, is_correct_method, "Expected ERC721 Deposit method");
            assert_eq!(
                true, is_correct_sender,
                "Expected correct sender, token address and token ID"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_ethers_withdrawal_success() {
    let recipient = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    let amount = U256::from_dec_str("1500000000000000000").unwrap(); // 1.5 Ether in wei
    // let payload =   r#"{"function_type": "EtherWithdrawal", "amount": "1500000000000000000", "exec_layer_data": "0x"}"#.to_string();

    let abi_encoded_input = abi_encode_call("WithdrawEther(uint256, bytes)", vec![Token::Uint(amount), Token::Bytes(vec![0x0])]).unwrap();
    let hex_input = format!("0x{}", hex::encode(&abi_encoded_input));

    let input = create_test_input(recipient, format!("{}", hex_input).as_str());

    match cma_decode_advance(CmaParserInputType::CmaParserInputTypeAuto, input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeEtherWithdrawal;
            let is_correct_recipient = if let CmaParserInputData::EtherWithdrawal(withdrawal) =
                result.input
            {
                if withdrawal.amount == amount && withdrawal.receiver == recipient.parse().unwrap()
                {
                    println!("withdrawal response is {}, {}", withdrawal.amount, withdrawal.receiver);
                    true
                } else {
                    false
                }
            } else {
                false
            };

            assert_eq!(true, is_correct_method, "Expected Ether Withdrawal method");
            assert_eq!(
                true, is_correct_recipient,
                "Expected correct recipient and amount"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_erc20_withdrawal_success() {
    let recipient = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    let amount = U256::from_dec_str("50000000000000000000").unwrap(); // 50 ERC20 tokens in wei
    let token_address: Address = "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C"
        .parse()
        .unwrap(); // TEST token address

    let abi_encoded_input = abi_encode_call("WithdrawErc20(address,uint256,bytes)", vec![Token::Address(token_address), Token::Uint(amount), Token::Bytes(vec![0x0])]).unwrap();
    let hex_input = format!("0x{}", hex::encode(&abi_encoded_input));

    let input = create_test_input(recipient, format!("{}", hex_input).as_str());
    match cma_decode_advance(CmaParserInputType::CmaParserInputTypeAuto, input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeErc20Withdrawal;
            let is_correct_recipient =
                if let CmaParserInputData::Erc20Withdrawal(withdrawal) = result.input {
                    if withdrawal.amount == amount
                        && withdrawal.receiver == recipient.parse().unwrap()
                        && withdrawal.token == token_address
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };

            assert_eq!(true, is_correct_method, "Expected ERC20 Withdrawal method");
            assert_eq!(
                true, is_correct_recipient,
                "Expected correct recipient, token address and amount"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_erc721_withdrawal_success() {
    let recipient = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    let token_id = U256::from_dec_str("1").unwrap(); // Sample token ID
    let token_address: Address = "0xBa46623aD94AB45850c4ecbA9555D26328917c3B"
        .parse()
        .unwrap(); // Sample ERC721 token address

    let abi_encoded_input = abi_encode_call("WithdrawErc721(address,uint256,bytes)", vec![Token::Address(token_address), Token::Uint(token_id), Token::Bytes(vec![0x0])]).unwrap();
    let hex_input = format!("0x{}", hex::encode(&abi_encoded_input));

    let input = create_test_input(recipient, format!("{}", hex_input).as_str());
    match cma_decode_advance(CmaParserInputType::CmaParserInputTypeAuto, input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeErc721Withdrawal;
            let is_correct_recipient =
                if let CmaParserInputData::Erc721Withdrawal(withdrawal) = result.input {
                    if withdrawal.token_id == token_id
                        && withdrawal.receiver == recipient.parse().unwrap()
                        && withdrawal.token == token_address
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };

            assert_eq!(true, is_correct_method, "Expected ERC721 Withdrawal method");
            assert_eq!(
                true, is_correct_recipient,
                "Expected correct recipient, token address and token ID"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_ether_transfer_success() {
    let sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    let mut recipient_bytes = [0u8; 32];
    recipient_bytes[31] = 120;
    let recipient: FixedBytes = FixedBytes::from(recipient_bytes);

    let expected_receipient: U256 = U256::from_big_endian(&recipient);

    let amount = U256::from_dec_str("1500000000000000000").unwrap(); // 1.5 Ether in wei
    
    let abi_encoded_input = abi_encode_call("TransferEther(uint256,bytes32,bytes)", vec![Token::Uint(amount), Token::FixedBytes(recipient), Token::Bytes(vec![0x0])]).unwrap();
    let hex_input = format!("0x{}", hex::encode(&abi_encoded_input));

    let input = create_test_input(&sender.to_string(), format!("{}", hex_input).as_str());
    match cma_decode_advance(CmaParserInputType::CmaParserInputTypeAuto, input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeEtherTransfer;
            let is_correct_transfer =
                if let CmaParserInputData::EtherTransfer(transfer) = result.input {
                    if transfer.amount == amount
                        && transfer.receiver == expected_receipient
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };

            assert_eq!(true, is_correct_method, "Expected Ether Transfer method");
            assert_eq!(
                true, is_correct_transfer,
                "Expected correct sender, recipient and amount"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_erc20_transfer_success() {
    let sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    let token_address: Address = "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C"
        .parse()
        .unwrap(); // TEST token address

    let mut recipient_bytes = [0u8; 32];
    recipient_bytes[31] = 120;
    let recipient: FixedBytes = FixedBytes::from(recipient_bytes);

    let expected_receipient: U256 = U256::from_big_endian(&recipient);

    let amount = U256::from_dec_str("1500000000000000000").unwrap(); // 1.5 Ether in wei
    
    let abi_encoded_input = abi_encode_call("TransferErc20(address,bytes32,uint256,bytes)", vec![Token::Address(token_address), Token::FixedBytes(recipient), Token::Uint(amount), Token::Bytes(vec![0x0])]).unwrap();
    let hex_input = format!("0x{}", hex::encode(&abi_encoded_input));

    let input = create_test_input(&sender.to_string(), format!("{}", hex_input).as_str());
    match cma_decode_advance(CmaParserInputType::CmaParserInputTypeAuto, input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeErc20Transfer;
            let is_correct_transfer =
                if let CmaParserInputData::Erc20Transfer(transfer) = result.input {
                    if transfer.amount == amount
                        && transfer.receiver == expected_receipient
                        && transfer.token == token_address
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
            assert_eq!(true, is_correct_method, "Expected ERC20 Transfer method");
            assert_eq!(
                true, is_correct_transfer,
                "Expected correct sender, recipient, token address and amount"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_erc721_transfer_success() {
    let token_id = U256::from_dec_str("1").unwrap(); // Sample token ID
    let sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    let token_address: Address = "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C"
        .parse()
        .unwrap(); // TEST token address

    let mut recipient_bytes = [0u8; 32];
    recipient_bytes[31] = 120;
    let recipient: FixedBytes = FixedBytes::from(recipient_bytes);

    let expected_receipient: U256 = U256::from_big_endian(&recipient);
    
    let abi_encoded_input = abi_encode_call("TransferErc721(address,bytes32,uint256,bytes)", vec![Token::Address(token_address), Token::FixedBytes(recipient), Token::Uint(token_id), Token::Bytes(vec![0x0])]).unwrap();
    let hex_input = format!("0x{}", hex::encode(&abi_encoded_input));

    let input = create_test_input(&sender.to_string(), format!("{}", hex_input).as_str());
    match cma_decode_advance(CmaParserInputType::CmaParserInputTypeAuto, input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeErc721Transfer;
            let is_correct_transfer =
                if let CmaParserInputData::Erc721Transfer(transfer) = result.input {
                    if transfer.token_id == token_id
                        && transfer.receiver == expected_receipient
                        && transfer.token == token_address
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
            assert_eq!(true, is_correct_method, "Expected ERC721 Transfer method");
            assert_eq!(
                true, is_correct_transfer,
                "Expected correct sender, recipient, token address and token ID"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_ether_voucher_encoding_success() {
    let receipient_string = "0x3e157927fb178490941bb18adcdc4144e442e32a".to_string();
    let recipient: Address = "0x3e157927fb178490941bb18adcdc4144e442e32a"
        .parse()
        .unwrap();
    let amount = U256::from_dec_str("1500000000000000000").unwrap(); // 1.5 Ether in wei
    let mut expected_value_bytes = [0u8; 32];
    amount.to_big_endian(&mut expected_value_bytes);

    let request =  CmaVoucherFieldType::EtherVoucherFields(CmaParserEtherVoucherFields {
            receiver: recipient,
            amount,
        });

    match cma_encode_voucher(CmaParserVoucherType::CmaParserVoucherTypeEther, request) {
        Ok(voucher) => {
            // Basic checks on the voucher structure
            assert!(
                voucher.destination.to_lowercase() == receipient_string,
                "Incorrect destination address in voucher"
            );
            assert!(
                voucher.value == format!("0x{}", hex::encode(expected_value_bytes)),
                "Incorrect value in voucher"
            );
            assert!(
                voucher.payload == "0x",
                "Payload should be empty for Ether transfer"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_erc20_voucher_encoding_success() {
    let destination_string = "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C".to_string();
    let recipient: Address = "0x3e157927fb178490941bb18adcdc4144e442e32a"
        .parse()
        .unwrap();
    let token_address: Address = "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C"
        .parse()
        .unwrap(); // TEST token address
    let amount = U256::from_dec_str("50000000000000000000").unwrap(); // 50 ERC20 tokens in wei
    let value = U256::zero(); // Value is typically zero for ERC20 transfers

    let mut expected_value_bytes = [0u8; 32];
    value.to_big_endian(&mut expected_value_bytes);

    // Create expected voucher payload
    let args: Vec<Token> = vec![Token::Address(recipient), Token::Uint(amount)];

    let function_sig = "transfer(address,uint256)";
    let selector = &id(function_sig)[..4];

    let encoded_args = encode(&args);
    let mut payload_bytes = Vec::new();
    payload_bytes.extend_from_slice(selector);
    payload_bytes.extend_from_slice(&encoded_args);
    let payload = format!("0x{}", hex::encode(payload_bytes));

    let request = CmaVoucherFieldType::Erc20VoucherFields(CmaParserErc20VoucherFields {
            token: token_address,
            amount,
            receiver: recipient,
            value: U256::zero(), // Value is typically zero for ERC20 transfers
        });
    match cma_encode_voucher(CmaParserVoucherType::CmaParserVoucherTypeErc20, request) {
        Ok(voucher) => {
            // Basic checks on the voucher structure
            assert!(
                voucher.destination.to_lowercase() == destination_string.to_lowercase(),
                "Incorrect destination address in voucher"
            );
            assert!(
                voucher.value == format!("0x{}", hex::encode(expected_value_bytes)),
                "Incorrect value in voucher"
            );
            assert!(
                voucher.payload.to_lowercase() == payload,
                "Incorrect payload in voucher"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_erc721_voucher_encoding_success() {
    let destination_string = "0xBa46623aD94AB45850c4ecbA9555D26328917c3B".to_string();
    let recipient: Address = "0x3e157927fb178490941bb18adcdc4144e442e32a"
        .parse()
        .unwrap();
    let application_address: Address = "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C"
        .parse()
        .unwrap();
    let token_address: Address = "0xBa46623aD94AB45850c4ecbA9555D26328917c3B"
        .parse()
        .unwrap(); // Sample ERC721 token address
    let token_id = U256::from_dec_str("1").unwrap(); // Sample token ID
    let value = U256::zero(); // Value is typically zero for ERC721 transfers

    let mut expected_value_bytes = [0u8; 32];
    value.to_big_endian(&mut expected_value_bytes);

    // Create expected voucher payload
    let args: Vec<Token> = vec![
        Token::Address(application_address),
        Token::Address(recipient),
        Token::Uint(token_id),
    ];

    let function_sig = "transferFrom(address,address,uint256)";
    let selector = &id(function_sig)[..4];

    let encoded_args = encode(&args);
    let mut payload_bytes = Vec::new();
    payload_bytes.extend_from_slice(selector);
    payload_bytes.extend_from_slice(&encoded_args);
    let payload = format!("0x{}", hex::encode(payload_bytes));

    let request =  CmaVoucherFieldType::Erc721VoucherFields(CmaParserErc721VoucherFields {
            token: token_address,
            token_id,
            receiver: recipient,
            value: U256::zero(), // Value is typically zero for ERC721 transfers
            application_address,
        });
    match cma_encode_voucher(CmaParserVoucherType::CmaParserVoucherTypeErc721, request) {
        Ok(voucher) => {
            // Basic checks on the voucher structure
            assert!(
                voucher.destination.to_lowercase() == destination_string.to_lowercase(),
                "Incorrect destination address in voucher"
            );
            assert!(
                voucher.value == format!("0x{}", hex::encode(expected_value_bytes)),
                "Incorrect value in voucher"
            );
            assert!(
                voucher.payload.to_lowercase() == payload,
                "Incorrect payload in voucher"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_ledger_get_balance_success() {
    let address: Address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        .parse()
        .unwrap();
    let erc20_token: Address = "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C"
        .parse()
        .unwrap(); // TEST token address
    let payload = r#"{"method": "ledgerGetBalance", "params": ["0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266", "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C", [1,2]]}"#.to_string();
    let payload_hex = hex::encode(payload.as_bytes());

    let input = create_test_input(&address.to_string(), &payload_hex);

    match cma_decode_inspect(input) {
        Ok(result) => {
            let is_correct_method =
                result.req_type == CmaParserInputType::CmaParserInputTypeBalance;
            let is_correct_address = if let CmaParserInputData::Balance(data) = result.input {
                if data.account == address
                    && data.token == erc20_token
                    && data.token_ids == Some(vec![U256::from(1), U256::from(2)])
                {
                    true
                } else {
                    false
                }
            } else {
                false
            };

            assert_eq!(
                true, is_correct_method,
                "Expected Ledger Get Balance method"
            );
            assert_eq!(true, is_correct_address, "Expected correct address");
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

#[test]
fn test_ledger_get_total_supply_success() {
    let address: Address = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266"
        .parse()
        .unwrap();
    let erc20_token: Address = "0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C"
        .parse()
        .unwrap(); // TEST token address
    let payload = r#"{"method": "ledgerGetTotalSupply", "params": ["0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C", [1,2]]}"#.to_string();
    let payload_hex = hex::encode(payload.as_bytes());
    let input = create_test_input(&address.to_string(), &payload_hex);

    match cma_decode_inspect(input) {
        Ok(result) => {
            let is_correct_method = result.req_type == CmaParserInputType::CmaParserInputTypeSupply;
            let is_correct_address = if let CmaParserInputData::Supply(data) = result.input {
                if data.token == erc20_token && data.token_ids == vec![U256::from(1), U256::from(2)]
                {
                    true
                } else {
                    false
                }
            } else {
                false
            };

            assert_eq!(
                true, is_correct_method,
                "Expected Ledger Get Total Supply method"
            );
            assert_eq!(
                true, is_correct_address,
                "Expected correct address and ID's"
            );
        }
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}

// #[test]
// fn test_ether_deposit_invalid_payload_length() {
//     // Payload too short

// }

// #[test]
// fn test_invalid_msg_sender() {

// }

// #[test]
// fn test_missing_payload() {

// }

// #[test]
// fn test_missing_msg_sender() {

// }
