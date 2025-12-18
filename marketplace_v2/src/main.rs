use cma_rust_parser::{Address, AddressCBindingsExt, U256, U256CBindingsExt};
use ethers_core::abi::{ParamType, Token, decode};
use json::{object, JsonValue};
use std::env;
use cma_rust_parser::helpers::{ToAddress, ToJson};
use cma_rust_parser::parser::{
    CmaParserErc721VoucherFields, CmaParserError, CmaParserInputData, CmaParserInputType, CmaParserVoucherType, CmaVoucherFieldType, cma_decode_advance, cma_decode_inspect, cma_encode_voucher
};
use cma_rust_parser::ledger::{Ledger};
use hex;

#[derive(Default)]
pub struct Storage {
    pub erc721_portal_address: String,
    pub erc20_portal_address: String,
    pub erc721_token: String,
    pub erc20_token: String,
    pub app_address: String,
    pub list_price: u128,
    pub listed_tokens: Vec<u128>,
    pub erc721_id_to_owner_address: std::collections::HashMap<u128, String>,
    pub ledger: Ledger,
}

impl Storage {
    fn new (
        erc721_portal_address: String,
        erc20_portal_address: String,
        erc721_token: String,
        erc20_token: String,
        list_price: u128,
        ledger: Ledger
    ) -> Self {
        Storage{
            erc20_portal_address,
            erc721_portal_address,
            erc20_token,
            erc721_token,
            list_price,
            app_address: "0x0000000000000000000000000000000000000000".to_string(),
            listed_tokens: Vec::new(),
            erc721_id_to_owner_address: std::collections::HashMap::new(),
            ledger
        }
    }

    fn get_listed_tokens(&self) -> Vec<&u128> {
        self.listed_tokens.iter().collect()
    }

    fn get_erc721_token_owner(&self, token_id: u128) -> Option<&String> {
        self.erc721_id_to_owner_address.get(&token_id)
    }

    fn list_token_for_sale(&mut self, token_id: u128) {
        if !self.listed_tokens.contains(&token_id) {
            self.listed_tokens.push(token_id);
        }
    }

    fn change_erc721_token_owner(&mut self, token_id: u128, new_owner: String) {
       if let Some(owner) =  self.erc721_id_to_owner_address.get_mut(&token_id) {
            *owner = new_owner.clone();
       }
    }

    async fn purchase_erc721_token(&mut self, buyer_address: &str, token_id: u128) -> Result<(), String> {
        let owner = self.get_erc721_token_owner(token_id).unwrap();
        let owner_id = self.ledger.retrieve_account_via_address(Address::from_str_hex(owner).unwrap()).map_err(|e| format!("{}", e))?;
        let buyer_id = self.ledger.retrieve_account_via_address(Address::from_str_hex(buyer_address).unwrap()).map_err(|e| format!("{}", e))?;
        let erc20_token_id = self.ledger.retrieve_erc20_asset_via_address(Address::from_str_hex(self.erc20_token.as_str()).unwrap()).map_err(|e| format!("{}", e))?;
        let erc721_token_id = self.ledger.retrieve_erc721_assets_via_address(Address::from_str_hex(self.erc721_token.to_lowercase().as_str()).unwrap(), U256::from(token_id)).map_err(|e| format!("{}", e))?;

        println!("USER WITH ID: {:?}, ATTEMPT TO PURCHASE USING TOKEN WITH ASSET_ID {:?}, FROM SELLER: {:?}", buyer_id, erc20_token_id, owner_id);
        
        match self.ledger.transfer(erc20_token_id, buyer_id, owner_id, U256::from(self.list_price)).map_err(|e| format!("{}", e)) {
            Ok(_) => {
                self.listed_tokens.retain(|token| *token != token_id);
                let zero_address = "0x0000000000000000000000000000000000000000";
                self.change_erc721_token_owner( token_id,  zero_address.to_string());

                match self.ledger.withdraw(erc721_token_id, owner_id, U256::from_u64(1)) {
                    Ok(_) => {return Ok(())},
                    Err(e) => {return Err(format!("{}", e))}
                }
            },
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
    }
}

async fn handle_erc20_deposit(input: &CmaParserInputData, storage: &mut Storage) -> Result<(), String> {
    if let CmaParserInputData::Erc20Deposit(data) = input {
        let token_address = format!("{:?}", data.token).to_lowercase();
        let depositor_address = format!("{:?}", data.sender);
        let amount_deposited = data.amount;
    
        println!("TOKEN ADDRESS: {}, EXPECTED TOKEN: {},  DEPOSITOR ADDRESS: {:?}, AMOUNT DEPOSITED: {}, IS SAME ADDRESS: {}, ADDRESS LENGTH: {}", 
        token_address, storage.erc20_token.to_lowercase(), depositor_address, amount_deposited, {token_address.to_lowercase() == storage.erc20_token.to_lowercase()}, token_address.len());

        let depositor_id = storage.ledger.retrieve_account_via_address(Address::from_str_hex(depositor_address.as_str()).unwrap()).map_err(|e| format!("{}", e))?;
        let asset_id = storage.ledger.retrieve_erc20_asset_via_address(Address::from_str_hex(token_address.as_str()).unwrap()).map_err(|e| format!("{}", e))?;

        match storage.ledger.deposit(asset_id, depositor_id, amount_deposited) {
            Ok(_) => {
                println!("USER WITH ID: {:?}, DEPOSITED TOKEN WITH ASSET_ID {:?}, ADDRESS: {:?}, AND AMOUNT: {}", depositor_id, asset_id, Address::from_str_hex(token_address.as_str()).unwrap(), amount_deposited);
                emit_notice(format!("AssetId Id: {:?}, Deposited by User: {}", asset_id, depositor_address)).await;
                return Ok(())
            },
            Err(e) => {
                println!("Error depositiing Token!!!: {}", e);
                emit_report(format!("Error depositing token:: {}", e)).await;
                return Err(format!("Error depositing token:: {}", e));
            }
        }
    } else {
        emit_report("Invalid input data for ERC20 deposit".into()).await;
        return Err(format!("Invalid input data for ERC20 deposit"));
    }
}

async fn handle_erc721_deposit(input: &CmaParserInputData, storage: &mut Storage) -> Result<(), String> {
    if let CmaParserInputData::Erc721Deposit(data) = input {
        let token_address = format!("{:?}", data.token);
        let depositor_address = format!("{:?}", data.sender);
        let token_id = data.token_id;

        let depositor_id = storage.ledger.retrieve_account_via_address(Address::from_str_hex(depositor_address.as_str()).unwrap()).map_err(|e| format!("{}", e))?;
        let asset_id = storage.ledger.retrieve_erc721_assets_via_address(Address::from_str_hex(&token_address).unwrap(), token_id).map_err(|e| format!("{}", e))?;

        match storage.ledger.deposit(asset_id, depositor_id, U256::from_u64(1)) {
            Ok(_) => {
                storage.list_token_for_sale(token_id.as_u128());

                println!("RECORED TOKEN DEPOSIT AND OWNER FOR TOKEN ID: {} AND OWNER: {}", token_id.as_u128(), depositor_address.to_lowercase());
                println!("USER WITH ID: {:?}, DEPOSITED TOKEN WITH ASSET_ID {:?}, AND ID: {}", depositor_id, asset_id, token_id);

                let zero_address = "0x0000000000000000000000000000000000000000".to_string();
                if storage.get_erc721_token_owner(token_id.as_u128()) == Some(&zero_address) {
                    storage.change_erc721_token_owner(token_id.as_u128(), depositor_address.to_lowercase());
                } else {
                    storage.erc721_id_to_owner_address.insert(token_id.as_u128(), depositor_address.to_lowercase());
                }
                return Ok(())
            },
            Err(e) => {
                println!("Error depositiing Token!!!: {}", e);
                emit_report(format!("Error depositing token:: {}", e)).await;
                return Err(format!("Error depositing token:: {}", e));
            }
        }
    } else {
        emit_report("Invalid input data for ERC721 deposit".into()).await;
        return Err(format!("Invalid input data for ERC721 deposit"));
    }
}

async fn handle_purchase_token(sender: String, input_args: &[u8], storage: &mut Storage) -> Result<String, String> {
    let param_types = vec![
        ParamType::Uint(256),
    ];

    let decoded = decode(&param_types, input_args).map_err(|e| format!("ABI decode failed: {}", e))?;

    let token_id = match &decoded[0] {
        Token::Uint(v) => v,
        _ => return Err(format!("Error decoding uint256 amount")),
    };

    let token_id: u128 = token_id.as_u128();

    println!("ATTEMPTING TO PURCHASE TOKEN WITH ID: {}", token_id);

    if storage.listed_tokens.contains(&token_id) != true {
        emit_report("TOKEN NOT LISTED FOR SALE!!".to_string()).await;
        return Err(format!("TOKEN NOT LISTED FOR SALE!!"));
    }
    match storage.purchase_erc721_token(&sender, token_id).await {
        Ok(_) => {
            let voucher_request = CmaParserErc721VoucherFields{
                token: storage.erc721_token.to_address().unwrap(),
                token_id: token_id.into(),
                receiver: sender.to_address().unwrap(),
                value: U256::from_dec_str("0").unwrap(),
                application_address: storage.app_address.to_address().unwrap()
            };

            if let Ok(voucher) = cma_encode_voucher(CmaParserVoucherType::CmaParserVoucherTypeErc721, CmaVoucherFieldType::Erc721VoucherFields(voucher_request)) {
                let json_string = format!("{}", voucher.to_json());
                println!("VOUCHER STRING IS: {}", json_string);
                emit_voucher(voucher.to_json()).await;
                println!("Token purchased and Withdrawn successfully");
            }
            return Ok(String::new());
        },
        Err(e) => {
            emit_report("Failed to purchase token".into()).await; 
            println!("Failed to purchase token: {}", e);
            return Err(format!("Failed to purchase token: {}", e));
        }
    }
}

async fn handle_application_defined_methods(input: &CmaParserInputData, storage: &mut Storage) {
    if let CmaParserInputData::Unidentified(data) = input {
        let input = data.abi_encoded_bytes.clone();
        // print!("RECEIVED CALLER IS: {}, EXPECTED CALLER IS: {}",data.msg_sender.to_string(), format!("{:?}", data.msg_sender));
        let caller = format!("{:?}", data.msg_sender);

        let (first_4_bytes, encoded_args) = input.split_at(4);
        let function_selector = format!("0x{}", hex::encode(first_4_bytes)).to_lowercase();
        println!("FUNCTION SELECTOR IS: {}", function_selector);

        match function_selector.as_str() {
            // Purchase token
            "0x3048f512" => {
                let _ = handle_purchase_token(caller, encoded_args, storage).await;
            },
            _ => {
                println!("Unsupported application-defined method: {}", function_selector);
                emit_report(format!("Unsupported application-defined method: {}", function_selector)).await;   
            }
        }
    }
}

pub async fn handle_advance(
    _client: &hyper::Client<hyper::client::HttpConnector>,
    _server_addr: &str,
    request: JsonValue,
    storage: &mut Storage
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received advance request data {}", &request);
    let zero_address = "0x0000000000000000000000000000000000000000".to_string();

    let msg_sender =
    request["data"]["metadata"]["msg_sender"]
        .as_str()
        .ok_or("Invalid msg_sender address")?;

    let app_addr = request["data"]["metadata"]["app_contract"]
    .as_str()
    .ok_or("Missing payload")?;
    if storage.app_address == zero_address {
        storage.app_address = app_addr.to_string();
    }
    
    let mut decoded_req = Err(CmaParserError::Unknown);

    match msg_sender {
        s if s.to_lowercase() == storage.erc721_portal_address.to_lowercase() => {
            let req_type = CmaParserInputType::CmaParserInputTypeErc721Deposit;
            decoded_req = cma_decode_advance(req_type, request.clone());
        },
        s if s.to_lowercase() == storage.erc20_portal_address.to_lowercase() => {
            let req_type = CmaParserInputType::CmaParserInputTypeErc20Deposit;
            decoded_req = cma_decode_advance(req_type, request.clone());
        },
        _ => {
            let req_type: CmaParserInputType = CmaParserInputType::CmaParserInputTypeAuto;
            decoded_req = cma_decode_advance(req_type, request.clone());
        }
    }

    match decoded_req {
        Ok(decoded) => {
            match decoded.req_type {
                CmaParserInputType::CmaParserInputTypeErc20Deposit => {
                    let _ = handle_erc20_deposit(&decoded.input, storage).await;
                },
                CmaParserInputType::CmaParserInputTypeErc721Deposit => {
                     let _ = handle_erc721_deposit(&decoded.input, storage).await;
                },
                CmaParserInputType::CmaParserInputTypeUnidentified => {
                    handle_application_defined_methods(&decoded.input, storage).await;
                }
                _ => {}
            }
        },
        Err(e) => {
            println!("Error decoding advance request: {:?}", e);
            emit_report(format!("Error decoding advance request: {:?}", e)).await;
        }
    }
    Ok("accept")
}


pub async fn handle_inspect(
    _client: &hyper::Client<hyper::client::HttpConnector>,
    _server_addr: &str,
    request: JsonValue,
    storage: &mut Storage
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received inspect request data {}", &request);

    match cma_decode_inspect(request) {
        Ok(parsed_json) => {
            match parsed_json.req_type {
                CmaParserInputType::CmaParserInputTypeBalance => {
                   if let CmaParserInputData::Balance(data) = parsed_json.input {
                        let account_id = storage.ledger.retrieve_account_via_address(data.account).map_err(|e| format!("{}", e))?;
                        let mut token;
                        let token_id = data.token_ids;
                        if let Some(tokens) = token_id.clone() {
                            if tokens.is_empty() {
                                token = storage.ledger.retrieve_erc20_asset_via_address(data.token).map_err(|e| format!("{}", e))?;
                                let token_bal = storage.ledger.get_balance(token, account_id).map_err(|e| format!("{}", e))?;
                                emit_report(format!("User: {}, balance: {:?}", data.account.to_string(), token_bal)).await;
                            } else {
                                let mut balance: Vec<U256> = Vec::new();
                                for token_id in tokens {
                                    token = storage.ledger.retrieve_erc721_assets_via_address(data.token, token_id).map_err(|e| format!("{}", e))?;
                                    let token_bal = storage.ledger.get_balance(token, account_id).map_err(|e| format!("{}", e))?;
                                    balance.push(token_bal);
                                }
                                emit_report(format!("User: {}, balance: {:?}", data.account.to_string(), balance)).await;
                            }
                        } 
                   }
                }
                _ => {}
            }
        }
        Err(err) => {
            match err {
                CmaParserError::IncompatibleInput => {
                    let listed_tokens = storage.get_listed_tokens();
                    emit_report(format!("All listed tokens are: {:?}", listed_tokens)).await;
                },
                _ => {
                    println!("Invalid inspect request received");
                    emit_report(String::from("Invalid inspect request received")).await;
                }
            }
        }
    }
    Ok("accept")
}

pub fn hex_to_string(hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    let hexstr = hex.strip_prefix("0x").unwrap_or(hex);
    let bytes = hex::decode(hexstr).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let s = String::from_utf8(bytes).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    Ok(s)
}

async fn emit_notice( payload: String) {
    let hex_string = {
        let s = payload.strip_prefix("0x").unwrap_or(payload.as_str());
        hex::encode(s.as_bytes())
    };

    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL").expect("ROLLUP_HTTP_SERVER_URL not set");
    let client = hyper::Client::new();

    let response = object! {
        "payload" => format!("0x{}", hex_string),
    };
    let request = hyper::Request::builder()
    .method(hyper::Method::POST)
    .header(hyper::header::CONTENT_TYPE, "application/json")
    .uri(format!("{}/notice", server_addr))
    .body(hyper::Body::from(response.dump()))
    .ok();
    let _response = client.request(request.unwrap()).await;
}

async fn emit_report( payload: String) {
    println!("GENERATING REPORT WITH THIS DETAILS::: {}", payload);
    let hex_string = {
        let s = payload.strip_prefix("0x").unwrap_or(payload.as_str());
        hex::encode(s.as_bytes())
    };

    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL").expect("ROLLUP_HTTP_SERVER_URL not set");
    let client = hyper::Client::new();

    let response = object! {
        "payload" => format!("0x{}", hex_string),
    };
    let request = hyper::Request::builder()
    .method(hyper::Method::POST)
    .header(hyper::header::CONTENT_TYPE, "application/json")
    .uri(format!("{}/report", server_addr))
    .body(hyper::Body::from(response.dump()))
    .ok();
    let _response = client.request(request.unwrap()).await;
}

async fn emit_voucher( voucher: JsonValue) -> Option<bool> {
    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL").expect("ROLLUP_HTTP_SERVER_URL not set");
    let client = hyper::Client::new();

    let request = hyper::Request::builder()
    .method(hyper::Method::POST)
    .header(hyper::header::CONTENT_TYPE, "application/json")
    .uri(format!("{}/voucher", server_addr))
    .body(hyper::Body::from(voucher.dump()))
    .ok()?;

    let response = client.request(request).await;

    match response {
        Ok(_) => {
            println!("Voucher generation successful");
            return Some(true);
        }
        Err(e) => {
            println!("Voucher request failed {}", e);
            None
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = hyper::Client::new();
    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL")?;
    let ledger = Ledger::new()?;

    let erc721_portal_address = String::from("0xc700d52F5290e978e9CAe7D1E092935263b60051");
    let erc20_portal_address = String::from("0xc700D6aDd016eECd59d989C028214Eaa0fCC0051");
    let erc20_token = String::from("0xFBdB734EF6a23aD76863CbA6f10d0C5CBBD8342C");
    let erc721_token = String::from("0xBa46623aD94AB45850c4ecbA9555D26328917c3B");

    let list_price: u128 = 100_000_000_000_000_000_000;
    let mut storage = Storage::new(erc721_portal_address, erc20_portal_address, erc721_token, erc20_token, list_price, ledger);


    let mut status = "accept";
    loop {
        println!("Sending finish");
        let response = object! {"status" => status};
        let request = hyper::Request::builder()
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .uri(format!("{}/finish", &server_addr))
            .body(hyper::Body::from(response.dump()))?;
        let response = client.request(request).await?;
        println!("Received finish status {}", response.status());

        if response.status() == hyper::StatusCode::ACCEPTED {
            println!("No pending rollup request, trying again");
        } else {
            let body = hyper::body::to_bytes(response).await?;
            let utf = std::str::from_utf8(&body)?;
            let req = json::parse(utf)?;

            let request_type = req["request_type"]
                .as_str()
                .ok_or("request_type is not a string")?;
            status = match request_type {
                "advance_state" => handle_advance(&client, &server_addr[..], req, &mut storage).await?,
                "inspect_state" => handle_inspect(&client, &server_addr[..], req, &mut storage).await?,
                &_ => {
                    eprintln!("Unknown request type");
                    "reject"
                }
            };
        }
    }
}
