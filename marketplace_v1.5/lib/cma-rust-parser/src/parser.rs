use crate::helpers::{hex_to_string};
use ethers_core::abi::{ParamType, Token, decode, encode};
use ethers_core::types::{Address, Bytes, U256};
use ethers_core::utils::{id, to_checksum};

use hex;
use json::{JsonValue};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxHexCodes {
    // Bytecode for solidity WithdrawEther(uint256,bytes) = 8cf70f0b
    WithdrawEther = 0x8cf70f0b,
    // Bytecode for solidity WithdrawErc20(address,uint256,bytes) = 4f94d342
    WithdrawErc20 = 0x4f94d342,
    // Bytecode for solidity WithdrawErc721(address,uint256,bytes) = 33acf293
    WithdrawErc721 = 0x33acf293,
    // Bytecode for solidity WithdrawErc1155Single(address,uint256,uint256,bytes) = 8bb0a811
    WithdrawErc1155Single = 0x8bb0a811,
    // Bytecode for solidity WithdrawErc1155Batch(address,uint256[],uint256[],bytes) = 50c80019
    WithdrawErc1155Batch = 0x50c80019,

    // Bytecode for solidity TransferEther(uint256,bytes32,bytes) = 428c9c4d
    TransferEther = 0x428c9c4d,
    // Bytecode for solidity TransferErc20(address,bytes32,uint256,bytes) = 03d61dcd
    TransferErc20 = 0x03d61dcd,
    // Bytecode for solidity TransferErc721(address,bytes32,uint256,bytes) = af615a5a
    TransferErc721 = 0xaf615a5a,
    // Bytecode for solidity TransferErc1155Single(address,bytes32,uint256,uint256,bytes) = e1c913ed
    TransferErc1155Single = 0xe1c913ed,
    // Bytecode for solidity TransferErc1155Batch(address,bytes32,uint256[],uint256[],bytes) = 638ac6f9
    TransferErc1155Batch = 0x638ac6f9,
    Unidentified
}

impl TxHexCodes {
    pub fn to_string(&self) -> &str {
        match self {
            Self::WithdrawEther => "0x8cf70f0b",
            Self::WithdrawErc20 => "0x4f94d342",
            Self::WithdrawErc721 => "0x33acf293",
            Self::WithdrawErc1155Single => "0x8bb0a811",
            Self::WithdrawErc1155Batch => "0x50c80019",
            Self::TransferEther => "0x428c9c4d",
            Self::TransferErc20 => "0x03d61dcd",
            Self::TransferErc721 => "0xaf615a5a",
            Self::TransferErc1155Single => "0xe1c913ed",
            Self::TransferErc1155Batch => "0x638ac6f9",
            Self::Unidentified => "0x00000000"
        }
    }

    pub fn from_string(input: &str) -> Self {
        match input {
            "0x8cf70f0b" => Self::WithdrawEther,
            "0x4f94d342" => Self::WithdrawErc20,
            "0x33acf293" => Self::WithdrawErc721,
            "0x8bb0a811" => Self::WithdrawErc1155Single,
            "0x50c80019" => Self::WithdrawErc1155Batch,
            "0x428c9c4d" => Self::TransferEther,
            "0x03d61dcd" => Self::TransferErc20,
            "0xaf615a5a" => Self::TransferErc721,
            "0xe1c913ed" => Self::TransferErc1155Single,
            "0x638ac6f9" => Self::TransferErc1155Batch,
            _ => Self::Unidentified, // default fallback
        }
    }

    pub fn to_input_type(&self) -> CmaParserInputType {
        match self {
            Self::WithdrawEther => CmaParserInputType::CmaParserInputTypeEtherWithdrawal,
            Self::WithdrawErc20 => CmaParserInputType::CmaParserInputTypeErc20Withdrawal,
            Self::WithdrawErc721 => CmaParserInputType::CmaParserInputTypeErc721Withdrawal,
            Self::WithdrawErc1155Single => CmaParserInputType::CmaParserInputTypeErc1155SingleWithdrawal,
            Self::WithdrawErc1155Batch => CmaParserInputType::CmaParserInputTypeErc1155BatchWithdrawal,
            Self::TransferEther => CmaParserInputType::CmaParserInputTypeEtherTransfer,
            Self::TransferErc20 => CmaParserInputType::CmaParserInputTypeErc20Transfer,
            Self::TransferErc721 => CmaParserInputType::CmaParserInputTypeErc721Transfer,
            Self::TransferErc1155Single => CmaParserInputType::CmaParserInputTypeErc1155SingleTransfer,
            Self::TransferErc1155Batch => CmaParserInputType::CmaParserInputTypeErc1155BatchTransfer,
            Self::Unidentified => CmaParserInputType::CmaParserInputTypeUnidentified,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmaVoucherFieldType {
    EtherVoucherFields(CmaParserEtherVoucherFields),
    Erc20VoucherFields(CmaParserErc20VoucherFields),
    Erc721VoucherFields(CmaParserErc721VoucherFields),
    Erc1155SingleVoucherFields(CmaParserErc1155SingleVoucherFields),
    Erc1155BatchVoucherFields(CmaParserErc1155BatchVoucherFields),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenType {
    Erc20,
    Erc721,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmaParserInputType {
    CmaParserInputTypeNone,
    CmaParserInputTypeAuto,
    CmaParserInputTypeUnidentified,
    CmaParserInputTypeEtherDeposit,
    CmaParserInputTypeErc20Deposit,
    CmaParserInputTypeErc721Deposit,
    CmaParserInputTypeErc1155SingleDeposit,
    CmaParserInputTypeErc1155BatchDeposit,
    CmaParserInputTypeEtherWithdrawal,
    CmaParserInputTypeErc20Withdrawal,
    CmaParserInputTypeErc721Withdrawal,
    CmaParserInputTypeErc1155SingleWithdrawal,
    CmaParserInputTypeErc1155BatchWithdrawal,
    CmaParserInputTypeEtherTransfer,
    CmaParserInputTypeErc20Transfer,
    CmaParserInputTypeErc721Transfer,
    CmaParserInputTypeErc1155SingleTransfer,
    CmaParserInputTypeErc1155BatchTransfer,
    CmaParserInputTypeBalance,
    CmaParserInputTypeSupply,
}

impl CmaParserInputType {
    pub fn from_string(s: &str) -> Self {
        match s {
            "EtherDeposit" => CmaParserInputType::CmaParserInputTypeEtherDeposit,
            "Erc20Deposit" => CmaParserInputType::CmaParserInputTypeErc20Deposit,
            "Erc721Deposit" => CmaParserInputType::CmaParserInputTypeErc721Deposit,
            "Erc1155SingleDeposit" => CmaParserInputType::CmaParserInputTypeErc1155SingleDeposit,
            "Erc1155BatchDeposit" => CmaParserInputType::CmaParserInputTypeErc1155BatchDeposit,
            "EtherWithdrawal" => CmaParserInputType::CmaParserInputTypeEtherWithdrawal,
            "Erc20Withdrawal" => CmaParserInputType::CmaParserInputTypeErc20Withdrawal,
            "Erc721Withdrawal" => CmaParserInputType::CmaParserInputTypeErc721Withdrawal,
            "Erc1155SingleWithdrawal" => {
                CmaParserInputType::CmaParserInputTypeErc1155SingleWithdrawal
            }
            "Erc1155BatchWithdrawal" => {
                CmaParserInputType::CmaParserInputTypeErc1155BatchWithdrawal
            }
            "EtherTransfer" => CmaParserInputType::CmaParserInputTypeEtherTransfer,
            "Erc20Transfer" => CmaParserInputType::CmaParserInputTypeErc20Transfer,
            "Erc721Transfer" => CmaParserInputType::CmaParserInputTypeErc721Transfer,
            "Erc1155SingleTransfer" => CmaParserInputType::CmaParserInputTypeErc1155SingleTransfer,
            "Erc1155BatchTransfer" => CmaParserInputType::CmaParserInputTypeErc1155BatchTransfer,
            "ledgerGetBalance" => CmaParserInputType::CmaParserInputTypeBalance,
            "ledgerGetTotalSupply" => CmaParserInputType::CmaParserInputTypeSupply,
            _ => CmaParserInputType::CmaParserInputTypeUnidentified,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmaParserVoucherType {
    CmaParserVoucherTypeNone,
    CmaParserVoucherTypeEther,
    CmaParserVoucherTypeErc20,
    CmaParserVoucherTypeErc721,
    CmaParserVoucherTypeErc1155Single,
    CmaParserVoucherTypeErc1155Batch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmaParserError {
    Success,
    IncompatibleInput,
    MalformedInput,
    Unknown,
    Message(String),
}

impl CmaParserError {
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => Self::Success,
            -2001 => Self::IncompatibleInput,
            -2002 => Self::MalformedInput,
            -2003 => Self::Unknown,
            -2004 => Self::Message("Unknown error".to_string()),
            _ => Self::Unknown,
        }
    }

    pub fn to_code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::IncompatibleInput => -2001,
            Self::MalformedInput => -2002,
            Self::Unknown => -2003,
            Self::Message(_) => -2004,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaVoucher {
    pub destination: String,
    // pub value: String,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserUnidentifiedInput {
    pub abi_encoded_bytes: Vec<u8>,
    pub msg_sender: Address,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserEtherVoucherFields {
    pub amount: U256,
    pub receiver: Address,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc20VoucherFields {
    pub token: Address,
    pub receiver: Address,
    // pub value: U256,
    pub amount: U256,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc721VoucherFields {
    pub token: Address,
    pub token_id: U256,
    pub receiver: Address,
    // pub value: U256,
    pub application_address: Address,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc1155SingleVoucherFields {
    pub token: Address,
    pub token_id: U256,
    pub receiver: Address,
    pub value: U256,
    pub amount: U256,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc1155BatchVoucherFields {
    pub token: Address,
    pub receiver: Address,
    pub count: usize,
    pub token_ids: Vec<U256>,
    pub value: U256,
    pub amounts: Vec<U256>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserEtherDeposit {
    pub sender: Address,
    pub amount: U256,
    pub exec_layer_data: Bytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc20Deposit {
    pub sender: Address,
    pub token: Address,
    pub amount: U256,
    pub exec_layer_data: Bytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc721Deposit {
    pub sender: Address,
    pub token: Address,
    pub token_id: U256,
    pub exec_layer_data: Bytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc1155SingleDeposit {
    pub sender: Address,
    pub token: Address,
    pub token_id: U256,
    pub amount: U256,
    pub exec_layer_data: Bytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc1155BatchDeposit {
    pub sender: Address,
    pub token: Address,
    pub count: usize,
    pub token_ids: Vec<U256>,
    pub amounts: Vec<U256>,
    pub base_layer_data: Bytes,
    pub exec_layer_data: Bytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserEtherWithdrawal {
    pub receiver: Address,
    pub amount: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc20Withdrawal {
    pub receiver: Address,
    pub token: Address,
    pub amount: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc721Withdrawal {
    pub receiver: Address,
    pub token: Address,
    pub token_id: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc1155SingleWithdrawal {
    pub receiver: Address,
    pub token: Address,
    pub token_id: U256,
    pub amount: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc1155BatchWithdrawal {
    pub receiver: Address,
    pub token: Address,
    pub count: usize,
    pub token_ids: Vec<U256>,
    pub amounts: Vec<U256>,
    pub base_layer_data: String,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserEtherTransfer {
    pub receiver: U256,
    pub amount: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc20Transfer {
    pub receiver: U256,
    pub token: Address,
    pub amount: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc721Transfer {
    pub receiver: U256,
    pub token: Address,
    pub token_id: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc1155SingleTransfer {
    pub sender: Address,
    pub receiver: Address,
    pub token: Address,
    pub token_id: U256,
    pub amount: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserErc1155BatchTransfer {
    pub sender: Address,
    pub receiver: Address,
    pub token: Address,
    pub count: usize,
    pub token_ids: Vec<U256>,
    pub amounts: Vec<U256>,
    pub base_layer_data: String,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserBalance {
    pub account: Address,
    pub token: Address,
    pub token_ids: Option<Vec<U256>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserSupply {
    pub token: Address,
    pub token_ids: Vec<U256>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmaParserInputData {
    EtherDeposit(CmaParserEtherDeposit),
    Erc20Deposit(CmaParserErc20Deposit),
    Erc721Deposit(CmaParserErc721Deposit),
    Erc1155SingleDeposit(CmaParserErc1155SingleDeposit),
    Erc1155BatchDeposit(CmaParserErc1155BatchDeposit),
    EtherWithdrawal(CmaParserEtherWithdrawal),
    Erc20Withdrawal(CmaParserErc20Withdrawal),
    Erc721Withdrawal(CmaParserErc721Withdrawal),
    Erc1155SingleWithdrawal(CmaParserErc1155SingleWithdrawal),
    Erc1155BatchWithdrawal(CmaParserErc1155BatchWithdrawal),
    EtherTransfer(CmaParserEtherTransfer),
    Erc20Transfer(CmaParserErc20Transfer),
    Erc721Transfer(CmaParserErc721Transfer),
    Erc1155SingleTransfer(CmaParserErc1155SingleTransfer),
    Erc1155BatchTransfer(CmaParserErc1155BatchTransfer),
    Balance(CmaParserBalance),
    Supply(CmaParserSupply),
    Unidentified(CmaParserUnidentifiedInput),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmaParserInput {
    pub req_type: CmaParserInputType,
    pub input: CmaParserInputData,
}

fn handle_unidentified_method(input: JsonValue) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let msg_sender =
        input["data"]["metadata"]["msg_sender"]
            .as_str()
            .ok_or(CmaParserError::Message(String::from(
                "Invalid msg_sender address",
            )))?;
    let sender = Address::from_slice(
        &hex::decode(msg_sender.trim_start_matches("0x"))
            .map_err(|e| CmaParserError::Message(format!("hex decode error: {}", e)))?,
    );

    let bytes = hex::decode(payload_hex.trim_start_matches("0x")).expect("invalid hex");

    return Ok(CmaParserInputData::Unidentified(CmaParserUnidentifiedInput {
        abi_encoded_bytes: bytes,
        msg_sender: sender,
    }));
}

fn handle_parse_ether_deposit(input: JsonValue) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let payload = payload_hex.trim_start_matches("0x");

    let bytes = hex::decode(payload)
        .map_err(|e| CmaParserError::Message(format!("hex decode error: {}", e)))?;

    if bytes.len() < 20 + 32 {
        return Err(CmaParserError::Message(
            "Invalid payload length".to_string(),
        ));
    }

    let sender_bytes = &bytes[0..20];
    let sender = Address::from_slice(sender_bytes);

    let value_bytes = &bytes[20..52];
    let value = U256::from_big_endian(value_bytes);

    let exec_layer_data = Bytes::from(bytes[52..].to_vec());

    Ok(CmaParserInputData::EtherDeposit(CmaParserEtherDeposit {
        sender,
        amount: value,
        exec_layer_data,
    }))
}

fn handle_parse_erc20_deposit(
    input: JsonValue,
    t_type: TokenType,
) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let payload = payload_hex.trim_start_matches("0x");

    let bytes = hex::decode(payload)
        .map_err(|e| CmaParserError::Message(format!("hex decode error: {}", e)))?;

    if bytes.len() < 20 + 20 + 32 {
        return Err(CmaParserError::Message(
            "Invalid payload length".to_string(),
        ));
    }

    let token = &bytes[1..21];
    let sender = &bytes[21..41];
    let amount_bytes = &bytes[41..73];
    let amount = U256::from_big_endian(amount_bytes);
    let exec_layer_data = Bytes::from(bytes[73..].to_vec());

    match t_type {
        TokenType::Erc20 => Ok(CmaParserInputData::Erc20Deposit(CmaParserErc20Deposit {
            sender: Address::from_slice(sender),
            token: Address::from_slice(token),
            amount,
            exec_layer_data,
        })),
        TokenType::Erc721 => Ok(CmaParserInputData::Erc721Deposit(CmaParserErc721Deposit {
            sender: Address::from_slice(sender),
            token: Address::from_slice(token),
            token_id: amount,
            exec_layer_data,
        })),
    }
}

fn handle_parse_erc721_deposit(
    input: JsonValue,
    t_type: TokenType,
) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let payload = payload_hex.trim_start_matches("0x");

    let bytes = hex::decode(payload)
        .map_err(|e| CmaParserError::Message(format!("hex decode error: {}", e)))?;

    if bytes.len() < 20 + 20 + 32 {
        return Err(CmaParserError::Message(
            "Invalid payload length".to_string(),
        ));
    }

    let token = &bytes[0..20];
    let sender = &bytes[20..40];
    let amount_bytes = &bytes[40..72];
    let amount = U256::from_big_endian(amount_bytes);
    let exec_layer_data = Bytes::from(bytes[72..].to_vec());

    match t_type {
        TokenType::Erc20 => Ok(CmaParserInputData::Erc20Deposit(CmaParserErc20Deposit {
            sender: Address::from_slice(sender),
            token: Address::from_slice(token),
            amount,
            exec_layer_data,
        })),
        TokenType::Erc721 => Ok(CmaParserInputData::Erc721Deposit(CmaParserErc721Deposit {
            sender: Address::from_slice(sender),
            token: Address::from_slice(token),
            token_id: amount,
            exec_layer_data,
        })),
    }
}


fn handle_parse_erc1155_single_deposit(
    input: JsonValue,
) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let payload = payload_hex.trim_start_matches("0x");

    let bytes = hex::decode(payload)
        .map_err(|e| CmaParserError::Message(format!("hex decode error: {}", e)))?;

    if bytes.len() < 20 + 20 + 32 + 32 {
        return Err(CmaParserError::Message(
            "Invalid payload length".to_string(),
        ));
    }

    let token = &bytes[0..20];
    let sender = &bytes[20..40];
    let token_id_bytes = &bytes[40..72];
    let token_id = U256::from_big_endian(token_id_bytes);
    let amount_bytes = &bytes[72..104];
    let amount = U256::from_big_endian(amount_bytes);
    let _base_layer_data = Bytes::from(bytes[104..136].to_vec());
    let exec_layer_data = Bytes::from(bytes[136..].to_vec());

    Ok(CmaParserInputData::Erc1155SingleDeposit(
        CmaParserErc1155SingleDeposit {
            sender: Address::from_slice(sender),
            token: Address::from_slice(token),
            token_id,
            amount,
            exec_layer_data,
        },
    ))
}

fn handle_parse_erc1155_batch_deposit(
    input: JsonValue,
) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let payload = payload_hex.trim_start_matches("0x");

    let bytes = hex::decode(payload)
        .map_err(|e| CmaParserError::Message(format!("hex decode error: {}", e)))?;

    if bytes.len() < 20 + 20 + 32 + 32 + 32 {
        return Err(CmaParserError::Message(
            "Invalid payload length".to_string(),
        ));
    }

    let u256_from = |b: &[u8]| U256::from_big_endian(b);
    let as_addr = |b: &[u8]| Address::from_slice(&b[12..32]);

    let token = as_addr(&bytes[0..32]);
    let sender = as_addr(&bytes[32..64]);

    let token_ids_offset = u256_from(&bytes[64..96]).as_usize();
    let values_offset = u256_from(&bytes[96..128]).as_usize();
    let base_offset = u256_from(&bytes[128..160]).as_usize();
    let exec_offset = u256_from(&bytes[160..192]).as_usize();

    let token_ids_len = u256_from(&bytes[token_ids_offset..token_ids_offset + 32]).as_usize();
    let mut token_ids = Vec::with_capacity(token_ids_len);

    let mut cursor = token_ids_offset + 32;
    for _ in 0..token_ids_len {
        token_ids.push(u256_from(&bytes[cursor..cursor + 32]));
        cursor += 32;
    }

    let values_len = u256_from(&bytes[values_offset..values_offset + 32]).as_usize();
    let mut values = Vec::with_capacity(values_len);

    let mut cursor2 = values_offset + 32;
    for _ in 0..values_len {
        values.push(u256_from(&bytes[cursor2..cursor2 + 32]));
        cursor2 += 32;
    }

    let base_len = u256_from(&bytes[base_offset..base_offset + 32]).as_usize();
    let base_start = base_offset + 32;
    let base_end = base_start + base_len;
    let base_layer_data = Bytes::from(bytes[base_start..base_end].to_vec());

    let exec_len = u256_from(&bytes[exec_offset..exec_offset + 32]).as_usize();
    let exec_start = exec_offset + 32;
    let exec_end = exec_start + exec_len;
    let exec_layer_data = Bytes::from(bytes[exec_start..exec_end].to_vec());

    if token_ids_len == 0 || values_len == 0 || token_ids_len != values_len {
        return Err(CmaParserError::Message("Invalid payload data".to_string()));
    }

    Ok(CmaParserInputData::Erc1155BatchDeposit(
        CmaParserErc1155BatchDeposit {
            sender,
            token,
            count: token_ids_len,
            token_ids,
            amounts: values,
            base_layer_data,
            exec_layer_data,
        },
    ))
}

fn handle_ether_withdrawal(input: JsonValue) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let msg_sender =
        input["data"]["metadata"]["msg_sender"]
            .as_str()
            .ok_or(CmaParserError::Message(String::from(
                "Invalid msg_sender address",
            )))?;

    let bytes = hex::decode(payload_hex.trim_start_matches("0x")).expect("invalid hex");
    let (_first_4_bytes, encoded_args) = bytes.split_at(4);

    let param_types = vec![
        ParamType::Uint(256),
        ParamType::Bytes,
    ];

    let receiver = msg_sender
        .parse::<Address>()
        .map_err(|e| CmaParserError::Message(format!("msg_sender address error: {}", e)))?;
    

    let decoded = decode(&param_types, encoded_args).map_err(|e| CmaParserError::Message(format!("ABI decode failed: {}", e)))?;

    let amount = match &decoded[0] {
        Token::Uint(v) => v,
        _ => return Err(CmaParserError::Message(format!("Error decoding uint256 amount"))),
    };

    let exec_layer_byte = match &decoded[1] {
        Token::Bytes(b) => b,
        _ => return Err(CmaParserError::Message(format!("Error decoding bytes execution layer data"))),
    };

    let exec_layer_data = format!("0x{}", hex::encode(exec_layer_byte));

    return Ok(CmaParserInputData::EtherWithdrawal(
        CmaParserEtherWithdrawal {
            receiver,
            amount: amount.clone(),
            exec_layer_data,
        },
    ));

}

fn handle_erc20_withdrawal(input: JsonValue) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let msg_sender =
        input["data"]["metadata"]["msg_sender"]
            .as_str()
            .ok_or(CmaParserError::Message(String::from(
                "Invalid msg_sender address",
            )))?;

    let bytes = hex::decode(payload_hex.trim_start_matches("0x")).expect("invalid hex");
    let (_first_4_bytes, encoded_args) = bytes.split_at(4);

    let param_types = vec![
        ParamType::Address,
        ParamType::Uint(256),
        ParamType::Bytes,
    ];

    let receiver = msg_sender
        .parse::<Address>()
        .map_err(|e| CmaParserError::Message(format!("msg_sender address error: {}", e)))?;
    

    let decoded = decode(&param_types, encoded_args).map_err(|e| CmaParserError::Message(format!("ABI decode failed: {}", e)))?;

    let token = match  &decoded[0] {
        Token::Address(v) => *v,
        _ => return Err(CmaParserError::Message(format!("Error decoding receiver address"))),
    };

    let amount = match &decoded[1] {
        Token::Uint(v) => *v,
        _ => return Err(CmaParserError::Message(format!("Error decoding uint256 amount"))),
    };

    let exec_layer_byte = match &decoded[2] {
        Token::Bytes(b) => b,
        _ => return Err(CmaParserError::Message(format!("Error decoding bytes execution layer data"))),
    };

    let exec_layer_data = format!("0x{}", hex::encode(exec_layer_byte));

    return Ok(CmaParserInputData::Erc20Withdrawal(
        CmaParserErc20Withdrawal {
            receiver,
            token,
            amount,
            exec_layer_data,
        },
    ))
}

fn handle_erc721_withdrawal(input: JsonValue) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let msg_sender =
        input["data"]["metadata"]["msg_sender"]
            .as_str()
            .ok_or(CmaParserError::Message(String::from(
                "Invalid msg_sender address",
            )))?;

    let bytes = hex::decode(payload_hex.trim_start_matches("0x")).expect("invalid hex");
    let (_first_4_bytes, encoded_args) = bytes.split_at(4);

    let param_types = vec![
        ParamType::Address,
        ParamType::Uint(256),
        ParamType::Bytes,
    ];

    let receiver = msg_sender
        .parse::<Address>()
        .map_err(|e| CmaParserError::Message(format!("msg_sender address error: {}", e)))?;
    

    let decoded = decode(&param_types, encoded_args).map_err(|e| CmaParserError::Message(format!("ABI decode failed: {}", e)))?;

    let token = match  &decoded[0] {
        Token::Address(v) => *v,
        _ => return Err(CmaParserError::Message(format!("Error decoding receiver address"))),
    };

    let token_id = match &decoded[1] {
        Token::Uint(v) => *v,
        _ => return Err(CmaParserError::Message(format!("Error decoding uint256 token id"))),
    };

    let exec_layer_byte = match &decoded[2] {
        Token::Bytes(b) => b,
        _ => return Err(CmaParserError::Message(format!("Error decoding bytes execution layer data"))),
    };

    let exec_layer_data = format!("0x{}", hex::encode(exec_layer_byte));

    return Ok(CmaParserInputData::Erc721Withdrawal(
        CmaParserErc721Withdrawal {
            receiver,
            token,
            token_id,
            exec_layer_data,
        },
    ))
}

fn handle_ether_transfer(input: JsonValue) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;

    let bytes = hex::decode(payload_hex.trim_start_matches("0x")).expect("invalid hex");
    let (_first_4_bytes, encoded_args) = bytes.split_at(4);

    let param_types = vec![
        ParamType::Uint(256),
        ParamType::FixedBytes(32),
        ParamType::Bytes
    ];

    let decoded = decode(&param_types, encoded_args).map_err(|e| CmaParserError::Message(format!("ABI decode failed: {}", e)))?;

    let amount = match  &decoded[0] {
        Token::Uint(v) => *v,
        _ => return Err(CmaParserError::Message(format!("Error decoding u256 amount"))),
    };

    let receiver = match &decoded[1] {
        Token::FixedBytes(v) if v.len() == 32 => {
            U256::from_big_endian(v)
        }
        _ => {
            return Err(CmaParserError::Message(
                "Error decodeing bytes32 receiver id".to_string(),
            ))
        }
    };

    let exec_layer_byte = match &decoded[2] {
        Token::Bytes(b) => b,
        _ => return Err(CmaParserError::Message(format!("Error decoding bytes execution layer data"))),
    };

    let exec_layer_data = format!("0x{}", hex::encode(exec_layer_byte));

    Ok(CmaParserInputData::EtherTransfer(CmaParserEtherTransfer {
        receiver,
        amount,
        exec_layer_data,
    }))
}

fn handle_erc20_transfer(input: JsonValue) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;

    let bytes = hex::decode(payload_hex.trim_start_matches("0x")).expect("invalid hex");
    let (_first_4_bytes, encoded_args) = bytes.split_at(4);

    let param_types = vec![
        ParamType::Address,
        ParamType::FixedBytes(32),
        ParamType::Uint(256),
        ParamType::Bytes
    ];

    let decoded = decode(&param_types, encoded_args).map_err(|e| CmaParserError::Message(format!("ABI decode failed: {}", e)))?;
    let token = match  &decoded[0] {
        Token::Address(v) => *v,
        _ => return Err(CmaParserError::Message(format!("Error decoding token address"))),
    };

    let receiver = match &decoded[1] {
        Token::FixedBytes(v) if v.len() == 32 => {
            U256::from_big_endian(v)
        }
        _ => {
            return Err(CmaParserError::Message(
                "Error decodeing bytes32 receiver id".to_string(),
            ))
        }
    };

    let amount = match  &decoded[2] {
        Token::Uint(v) => *v,
        _ => return Err(CmaParserError::Message(format!("Error decoding u256 amount"))),
    };


    let exec_layer_byte = match &decoded[3] {
        Token::Bytes(b) => b,
        _ => return Err(CmaParserError::Message(format!("Error decoding bytes execution layer data"))),
    };

    let exec_layer_data = format!("0x{}", hex::encode(exec_layer_byte));

    return Ok(CmaParserInputData::Erc20Transfer(CmaParserErc20Transfer {
        receiver,
        token,
        amount,
        exec_layer_data,
    }))
}

fn handle_erc721_transfer(input: JsonValue) -> Result<CmaParserInputData, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;

    let bytes = hex::decode(payload_hex.trim_start_matches("0x")).expect("invalid hex");
    let (_first_4_bytes, encoded_args) = bytes.split_at(4);

    let param_types = vec![
        ParamType::Address,
        ParamType::FixedBytes(32),
        ParamType::Uint(256),
        ParamType::Bytes
    ];

    let decoded = decode(&param_types, encoded_args).map_err(|e| CmaParserError::Message(format!("ABI decode failed: {}", e)))?;
    let token = match  &decoded[0] {
        Token::Address(v) => *v,
        _ => return Err(CmaParserError::Message(format!("Error decoding token address"))),
    };

    let receiver = match &decoded[1] {
        Token::FixedBytes(v) if v.len() == 32 => {
            U256::from_big_endian(v)
        }
        _ => {
            return Err(CmaParserError::Message(
                "Error decodeing bytes32 receiver id".to_string(),
            ))
        }
    };

    let token_id = match  &decoded[2] {
        Token::Uint(v) => *v,
        _ => return Err(CmaParserError::Message(format!("Error decoding u256 amount"))),
    };


    let exec_layer_byte = match &decoded[3] {
        Token::Bytes(b) => b,
        _ => return Err(CmaParserError::Message(format!("Error decoding bytes execution layer data"))),
    };

    let exec_layer_data = format!("0x{}", hex::encode(exec_layer_byte));

    return Ok(CmaParserInputData::Erc721Transfer(
        CmaParserErc721Transfer {
            receiver,
            token,
            token_id,
            exec_layer_data,
        },
    ))
}

pub fn cma_decode_advance(req_type:  CmaParserInputType, input: JsonValue) -> Result<CmaParserInput, CmaParserError> {
    // Determine the portal type based on msg_sender
    match req_type {
        CmaParserInputType::CmaParserInputTypeErc1155BatchDeposit => {
            return handle_parse_erc1155_batch_deposit(input).map(|data| CmaParserInput {
                req_type,
                input: data,
            });
        }
        CmaParserInputType::CmaParserInputTypeErc1155SingleDeposit=> {
            return handle_parse_erc1155_single_deposit(input).map(|data| CmaParserInput {
                req_type,
                input: data,
            });
        }
        CmaParserInputType::CmaParserInputTypeErc721Deposit => {
            return handle_parse_erc721_deposit(input, TokenType::Erc721).map(|data| {
                CmaParserInput {
                    req_type,
                    input: data,
                }
            });
        }
        CmaParserInputType::CmaParserInputTypeErc20Deposit => {
            return handle_parse_erc20_deposit(input, TokenType::Erc20).map(|data| {
                CmaParserInput {
                    req_type,
                    input: data,
                }
            });
        }
        CmaParserInputType::CmaParserInputTypeEtherDeposit => {
            return handle_parse_ether_deposit(input).map(|data| CmaParserInput {
                req_type,
                input: data,
            });
        }
        // IF CALLER IS NOT ANY OF THE ABOVE PORTALS, WE TRY TO PARSE THE PAYLOAD FOR WITHDRAWALS/TRANSFERS
        CmaParserInputType::CmaParserInputTypeAuto => {
            let payload_hex = input["data"]["payload"]
                .as_str()
                .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
            
            let bytes = hex::decode(payload_hex.trim_start_matches("0x")).expect("invalid hex");

            let (first_4_bytes,_encoded_args) = bytes.split_at(4);
            let selector_str = format!("0x{}", hex::encode(first_4_bytes));
            let req_type = TxHexCodes::from_string(&selector_str).to_input_type();

            // Handle the request based on the determined type
            match req_type {
                CmaParserInputType::CmaParserInputTypeEtherWithdrawal => {
                    return handle_ether_withdrawal(input).map(|data| CmaParserInput {
                        req_type,
                        input: data,
                    });
                }
                CmaParserInputType::CmaParserInputTypeErc20Withdrawal => {
                    return handle_erc20_withdrawal(input).map(|data| CmaParserInput {
                        req_type,
                        input: data,
                    });
                }
                CmaParserInputType::CmaParserInputTypeErc721Withdrawal => {
                    return handle_erc721_withdrawal(input).map(|data| CmaParserInput {
                        req_type,
                        input: data,
                    });
                }
                CmaParserInputType::CmaParserInputTypeEtherTransfer => {
                    return handle_ether_transfer(input).map(|data| CmaParserInput {
                        req_type,
                        input: data,
                    });
                }
                CmaParserInputType::CmaParserInputTypeErc20Transfer => {
                    return handle_erc20_transfer(input).map(|data| CmaParserInput {
                        req_type,
                        input: data,
                    });
                }
                CmaParserInputType::CmaParserInputTypeErc721Transfer => {
                    return handle_erc721_transfer(input).map(|data| CmaParserInput {
                        req_type,
                        input: data,
                    });
                }
                CmaParserInputType::CmaParserInputTypeUnidentified => {
                    return handle_unidentified_method(input).map(|data| CmaParserInput {
                        req_type,
                        input: data,
                    });
                }
                _ => Err(CmaParserError::IncompatibleInput),
            }
        }
        _ => {
            return Err(CmaParserError::Unknown)
        }
    }
}

fn handle_ledger_get_balance(parsed_json: JsonValue) -> Result<CmaParserBalance, CmaParserError> {
    // Extract params array
    let params_val = &parsed_json["params"];
    if !params_val.is_array() {
        return Err(CmaParserError::Message(
            "Invalid params: not an array".into(),
        ));
    }

    // params[0] = account
    let account_str = params_val[0]
        .as_str()
        .ok_or_else(|| CmaParserError::Message("Invalid account param".into()))?;

    let account = account_str
        .parse::<Address>()
        .map_err(|e| CmaParserError::Message(format!("account address error: {}", e)))?;

    // params[1] = token
    let token_str = params_val[1]
        .as_str()
        .ok_or_else(|| CmaParserError::Message("Invalid token param".into()))?;

    let token = token_str
        .parse::<Address>()
        .map_err(|e| CmaParserError::Message(format!("token address error: {}", e)))?;

    // params[2] = optional array containing token id's
    let mut token_id: Vec<U256> = Vec::new();
    if params_val.len() > 2 && params_val[2].is_array() {
        // params_val[2][0] might itself be an array of ids or params_val[2] might be the array
        if params_val[2][0].is_array() {
            for v in params_val[2][0].members() {
                let s = v
                    .as_str()
                    .ok_or_else(|| CmaParserError::Message("Invalid token id".into()))?;
                let id = U256::from_dec_str(s)
                    .map_err(|e| CmaParserError::Message(format!("token id parse error: {}", e)))?;
                token_id.push(id);
            }
        } else {
            // handle array of strings/numbers or comma-separated string(s)
            for v in params_val[2].members() {
                if let Some(s) = v.as_str() {
                    if s.contains(',') {
                        for part in s.split(',').map(str::trim).filter(|p| !p.is_empty()) {
                            let id = U256::from_dec_str(part).map_err(|e| {
                                CmaParserError::Message(format!("token id parse error: {}", e))
                            })?;
                            token_id.push(id);
                        }
                    } else {
                        let id = U256::from_dec_str(s).map_err(|e| {
                            CmaParserError::Message(format!("token id parse error: {}", e))
                        })?;
                        token_id.push(id);
                    }
                } else if v.is_number() {
                    let id = U256::from_dec_str(&v.to_string()).map_err(|e| {
                        CmaParserError::Message(format!("token id parse error: {}", e))
                    })?;
                    token_id.push(id);
                } else {
                    return Err(CmaParserError::Message("Invalid token id format".into()));
                }
            }
        }
    }

    Ok(CmaParserBalance {
        account,
        token,
        token_ids: Some(token_id),
    })
}

fn handle_ledger_get_supply(parsed_json: JsonValue) -> Result<CmaParserInputData, CmaParserError> {
    // Ensure params is an array
    let params_val = &parsed_json["params"];
    if !params_val.is_array() {
        return Err(CmaParserError::Message(
            "Invalid params: not an array".into(),
        ));
    }

    // params[0] = token address string
    let token_str = params_val[0]
        .as_str()
        .ok_or_else(|| CmaParserError::Message("Invalid token param".into()))?;

    let token = token_str
        .parse::<Address>()
        .map_err(|e| CmaParserError::Message(format!("token address error: {}", e)))?;

    // params[1] = optional array containing token id's
    let mut token_id: Vec<U256> = Vec::new();
    if params_val.len() > 1 && params_val[1].is_array() {
        // params_val[1][0] might itself be an array of ids or params_val[1] might be the array
        if params_val[1][0].is_array() {
            for v in params_val[1][0].members() {
                let s = v
                    .as_str()
                    .ok_or_else(|| CmaParserError::Message("Invalid token id".into()))?;
                let id = U256::from_dec_str(s)
                    .map_err(|e| CmaParserError::Message(format!("token id parse error: {}", e)))?;
                token_id.push(id);
            }
        } else {
            // handle array of strings/numbers or comma-separated string(s)
            for v in params_val[1].members() {
                if let Some(s) = v.as_str() {
                    if s.contains(',') {
                        for part in s.split(',').map(str::trim).filter(|p| !p.is_empty()) {
                            let id = U256::from_dec_str(part).map_err(|e| {
                                CmaParserError::Message(format!("token id parse error: {}", e))
                            })?;
                            token_id.push(id);
                        }
                    } else {
                        let id = U256::from_dec_str(s).map_err(|e| {
                            CmaParserError::Message(format!("token id parse error: {}", e))
                        })?;
                        token_id.push(id);
                    }
                } else if v.is_number() {
                    let id = U256::from_dec_str(&v.to_string()).map_err(|e| {
                        CmaParserError::Message(format!("token id parse error: {}", e))
                    })?;
                    token_id.push(id);
                } else {
                    return Err(CmaParserError::Message("Invalid token id format".into()));
                }
            }
        }
    }

    Ok(CmaParserInputData::Supply(CmaParserSupply {
        token,
        token_ids: token_id,
    }))
}

pub fn cma_decode_inspect(input: JsonValue) -> Result<CmaParserInput, CmaParserError> {
    let payload_hex = input["data"]["payload"]
        .as_str()
        .ok_or(CmaParserError::Message(String::from("Invalid payload hex")))?;
    let payload_str = hex_to_string(payload_hex)
        .map_err(|e| CmaParserError::Message(format!("hex to string conversion error: {}", e)))?;
    let payload_json = json::parse(&payload_str)
        .map_err(|e| CmaParserError::Message(format!("Error parsing string to JSON: {}", e)))?;

    let req_type: CmaParserInputType =
        CmaParserInputType::from_string(payload_json["method"].as_str().ok_or(
            CmaParserError::Message(String::from("Invalid inspection type")),
        )?);

    match req_type {
        CmaParserInputType::CmaParserInputTypeBalance => {
            return handle_ledger_get_balance(payload_json).map(|data| CmaParserInput {
                req_type,
                input: CmaParserInputData::Balance(data),
            });
        }
        CmaParserInputType::CmaParserInputTypeSupply => {
            return handle_ledger_get_supply(payload_json).map(|data| CmaParserInput {
                req_type,
                input: data,
            });
        }
        _ => {
            // Unsupported inspection type
            return Err(CmaParserError::IncompatibleInput);
        }
    }
}

fn handle_ether_voucher_encoding(
    voucher_request: &CmaVoucherFieldType,
) -> Result<CmaVoucher, CmaParserError> {
    if let CmaVoucherFieldType::EtherVoucherFields(fields) = &voucher_request {
        let payload = "0x".to_string();

        let mut value_bytes = [0u8; 32];
        fields.amount.to_big_endian(&mut value_bytes);

        let voucher = CmaVoucher {
            destination: to_checksum(&fields.receiver, None),
            payload,
        };

        Ok(voucher)
    } else {
        Err(CmaParserError::Message(String::from(
            "Invalid voucher fields for Ether",
        )))
    }
}

fn handle_erc20_voucher_encoding(
    voucher_request: &CmaVoucherFieldType,
) -> Result<CmaVoucher, CmaParserError> {
    if let CmaVoucherFieldType::Erc20VoucherFields(fields) = &voucher_request {
        let token = fields.token;

        let args: Vec<Token> = vec![Token::Address(fields.receiver), Token::Uint(fields.amount)];

        let function_sig = "transfer(address,uint256)";
        let selector = &id(function_sig)[..4];

        let encoded_args = encode(&args);
        let mut payload_bytes = Vec::new();
        payload_bytes.extend_from_slice(selector);
        payload_bytes.extend_from_slice(&encoded_args);
        let payload = format!("0x{}", hex::encode(payload_bytes));

        let voucher = CmaVoucher {
            destination: format!("{:?}", token),
            payload: format!("{}", payload),
        };
        return Ok(voucher);
    } else {
        Err(CmaParserError::Message(String::from(
            "Invalid voucher fields for ERC20",
        )))
    }
}

fn handle_erc721_voucher_encoding(
    voucher_request: &CmaVoucherFieldType,
) -> Result<CmaVoucher, CmaParserError> {
    if let CmaVoucherFieldType::Erc721VoucherFields(fields) = &voucher_request {
        let token = fields.token;

        let args: Vec<Token> = vec![
            Token::Address(fields.application_address),
            Token::Address(fields.receiver),
            Token::Uint(fields.token_id.into()),
        ];
        let function_sig = "transferFrom(address,address,uint256)";
        let selector = &id(function_sig)[..4];
        let encoded_args = encode(&args);
        let mut payload_bytes = Vec::new();
        payload_bytes.extend_from_slice(selector);
        payload_bytes.extend_from_slice(&encoded_args);
        let payload = format!("0x{}", hex::encode(payload_bytes));

        let voucher = CmaVoucher {
            destination: format!("{:?}", token),
            payload: format!("{}", payload),
        };

        return Ok(voucher);
    } else {
        Err(CmaParserError::Message(String::from(
            "Invalid voucher fields for ERC721",
        )))
    }
}

pub fn cma_encode_voucher(
    req_type: CmaParserVoucherType,
    voucher_request: CmaVoucherFieldType,
) -> Result<CmaVoucher, CmaParserError> {
    match req_type {
        CmaParserVoucherType::CmaParserVoucherTypeEther => {
            return handle_ether_voucher_encoding(&voucher_request);
        }
        CmaParserVoucherType::CmaParserVoucherTypeErc20 => {
            return handle_erc20_voucher_encoding(&voucher_request);
        }
        CmaParserVoucherType::CmaParserVoucherTypeErc721 => {
            return handle_erc721_voucher_encoding(&voucher_request);
        }
        CmaParserVoucherType::CmaParserVoucherTypeErc1155Single => {
            // TODO Implement encoding logic for ERC1155 Single voucher
            return Err(CmaParserError::Message(String::from("Not Implemented yet")));
        }
        CmaParserVoucherType::CmaParserVoucherTypeErc1155Batch => {
            // TODO Implement encoding logic for ERC1155 Batch voucher
            return Err(CmaParserError::Message(String::from("Not Implemented yet")));
        }
        CmaParserVoucherType::CmaParserVoucherTypeNone => {
            return Err(CmaParserError::Message(String::from("Not Implemented yet")))
        }
    }
}
