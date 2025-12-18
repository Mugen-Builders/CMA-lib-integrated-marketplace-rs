use crate::bindings;

/// Ledger operation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    Unknown,
    Exception,
    InsufficientFunds,
    AccountNotFound,
    AssetNotFound,
    SupplyOverflow,
    BalanceOverflow,
    InvalidAccount,
    InsertionError,
    Other(i32),
}

impl LedgerError {
    pub fn from_code(code: i32) -> Self {
        match code {
            x if x == bindings::CMA_LEDGER_SUCCESS as i32 => unreachable!(),
            x if x == bindings::CMA_LEDGER_ERROR_UNKNOWN as i32 => LedgerError::Unknown,
            x if x == bindings::CMA_LEDGER_ERROR_EXCEPTION as i32 => LedgerError::Exception,
            x if x == bindings::CMA_LEDGER_ERROR_INSUFFICIENT_FUNDS as i32 => LedgerError::InsufficientFunds,
            x if x == bindings::CMA_LEDGER_ERROR_ACCOUNT_NOT_FOUND as i32 => LedgerError::AccountNotFound,
            x if x == bindings::CMA_LEDGER_ERROR_ASSET_NOT_FOUND as i32 => LedgerError::AssetNotFound,
            x if x == bindings::CMA_LEDGER_ERROR_SUPPLY_OVERFLOW as i32 => LedgerError::SupplyOverflow,
            x if x == bindings::CMA_LEDGER_ERROR_BALANCE_OVERFLOW as i32 => LedgerError::BalanceOverflow,
            x if x == bindings::CMA_LEDGER_ERROR_INVALID_ACCOUNT as i32 => LedgerError::InvalidAccount,
            x if x == bindings::CMA_LEDGER_ERROR_INSERTION_ERROR as i32 => LedgerError::InsertionError,
            _ => LedgerError::Other(code),
        }
    }

    pub fn message(&self) -> String {
        unsafe {
            let msg = bindings::cma_ledger_get_last_error_message();
            if msg.is_null() {
                format!("{:?}", self)
            } else {
                std::ffi::CStr::from_ptr(msg)
                    .to_string_lossy()
                    .to_string()
            }
        }
    }
}

impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for LedgerError {}

/// Parser operation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    Unknown,
    Exception,
    IncompatibleInput,
    MalformedInput,
    InvalidAmount,
    Other(i32),
}

impl ParserError {
    pub fn from_code(code: i32) -> Self {
        match code {
            x if x == bindings::CMA_PARSER_SUCCESS as i32 => unreachable!(),
            x if x == bindings::CMA_PARSER_ERROR_UNKNOWN as i32 => ParserError::Unknown,
            x if x == bindings::CMA_PARSER_ERROR_EXCEPTION as i32 => ParserError::Exception,
            x if x == bindings::CMA_PARSER_ERROR_INCOMPATIBLE_INPUT as i32 => ParserError::IncompatibleInput,
            x if x == bindings::CMA_PARSER_ERROR_MALFORMED_INPUT as i32 => ParserError::MalformedInput,
            x if x == bindings::CMA_PARSER_ERROR_INVALID_AMOUNT as i32 => ParserError::InvalidAmount,
            _ => ParserError::Other(code),
        }
    }

    pub fn message(&self) -> String {
        unsafe {
            let msg = bindings::cma_parser_get_last_error_message();
            if msg.is_null() {
                format!("{:?}", self)
            } else {
                std::ffi::CStr::from_ptr(msg)
                    .to_string_lossy()
                    .to_string()
            }
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ParserError {}