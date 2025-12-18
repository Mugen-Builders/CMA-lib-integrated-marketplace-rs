use hex;
use once_cell::sync::Lazy;
use ethers_core::types::{Address};
use json::{JsonValue, object};
use crate::parser::CmaVoucher;

pub fn hex_to_string(hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    let hexstr = hex.strip_prefix("0x").unwrap_or(hex);
    let bytes = hex::decode(hexstr).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let s = String::from_utf8(bytes).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    Ok(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Portals {
    ERC1155BatchPortal,
    ERC1155SinglePortal,
    ERC20Portal,
    ERC721Portal,
    EtherPortal,
    None,
}

pub struct CartesiAddresses {
    pub erc1155_batch_portal: String,
    pub erc1155_single_portal: String,
    pub erc20_portal: String,
    pub erc721_portal: String,
    pub ether_portal: String,
}

pub static CARTESI_ADDRESSES: Lazy<CartesiAddresses> = Lazy::new(|| CartesiAddresses {
    erc1155_batch_portal: "0xc700A2e5531E720a2434433b6ccf4c0eA2400051".to_string(),
    erc1155_single_portal: "0xc700A261279aFC6F755A3a67D86ae43E2eBD0051".to_string(),
    erc20_portal: "0xc700D6aDd016eECd59d989C028214Eaa0fCC0051".to_string(),
    erc721_portal: "0xc700d52F5290e978e9CAe7D1E092935263b60051".to_string(),
    ether_portal: "0xc70076a466789B595b50959cdc261227F0D70051".to_string(),
});

pub trait PortalMatcher {
    fn match_portal(&self, addr: &str) -> Portals;
    fn get_portal_address(&self, portal: Portals) -> Option<&str>;
}

impl PortalMatcher for CartesiAddresses {
    fn match_portal(&self, addr: &str) -> Portals {
        let caller_address = addr.trim().to_lowercase();

        if caller_address == self.erc1155_batch_portal.to_lowercase() {
            Portals::ERC1155BatchPortal
        } else if caller_address == self.erc1155_single_portal.to_lowercase() {
            Portals::ERC1155SinglePortal
        } else if caller_address == self.erc20_portal.to_lowercase() {
            Portals::ERC20Portal
        } else if caller_address == self.erc721_portal.to_lowercase() {
            Portals::ERC721Portal
        } else if caller_address == self.ether_portal.to_lowercase() {
            Portals::EtherPortal
        } else {
            Portals::None
        }
    }

    fn get_portal_address(&self, portal: Portals) -> Option<&str> {
        match portal {
            Portals::ERC1155BatchPortal => Some(&self.erc1155_batch_portal),
            Portals::ERC1155SinglePortal => Some(&self.erc1155_single_portal),
            Portals::ERC20Portal => Some(&self.erc20_portal),
            Portals::ERC721Portal => Some(&self.erc721_portal),
            Portals::EtherPortal => Some(&self.ether_portal),
            Portals::None => None,
        }
    }
}

pub trait ToAddress {
    fn to_address(&self) -> Result<Address, String>;
}

impl ToAddress for str {
    fn to_address(&self) -> Result<Address, String> {
        self.parse::<Address>()
            .map_err(|e| format!("Invalid Ethereum address: {}", e))
    }
}

impl ToAddress for String {
    fn to_address(&self) -> Result<Address, String> {
        self.as_str().to_address()
    }
}

#[allow(dead_code)]
pub trait ToJson {
    fn to_json(&self) -> JsonValue;
}

impl ToJson for CmaVoucher {
    fn to_json(&self) -> JsonValue {
        let voucher = object! {
            "destination" => format!("{}", self.destination),
            "payload" => format!("{}", self.payload),
            "value" => format!("{}", self.value),
        };
        return voucher;
    }
}