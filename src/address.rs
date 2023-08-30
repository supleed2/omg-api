use clap::Subcommand;
use serde::Deserialize;

use crate::{get, get_auth};

#[derive(Debug, Subcommand)]
pub enum Address {
    /// Get information about the availability of an address
    #[clap(visible_aliases = &["a", "av"])]
    IsAvailable {
        /// Address to get availability of
        address: String,
    },
    /// Get the expiration date for an address
    #[clap(visible_aliases = &["e", "exp"])]
    GetExpiry {
        /// Address to get availability of
        address: String,
    },
    /// Get limited (public) information about an address (no auth required)
    #[clap(visible_aliases = &["pi", "pinfo"])]
    GetPublicInfo {
        /// Address to get availability of
        address: String,
    },
    /// Get comprehensive information about an address
    #[clap(visible_aliases = &["i", "info"])]
    GetInfo {
        /// Address to get availability of
        address: String,
    },
}

impl Address {
    pub fn process(&self, api_key: &str) -> Result<AddressResponse, reqwest::Error> {
        match self {
            Address::IsAvailable { address } => Ok(AddressResponse::IsAvailable(get(&format!(
                "address/{address}/availability"
            ))?)),
            Address::GetExpiry { address } => Ok(AddressResponse::GetExpiry(get(&format!(
                "address/{address}/expiration"
            ))?)),
            Address::GetPublicInfo { address } => Ok(AddressResponse::GetPublicInfo(get(
                &format!("address/{address}/info"),
            )?)),
            Address::GetInfo { address } => Ok(AddressResponse::GetInfo(get_auth(
                api_key,
                &format!("address/{address}/info"),
            )?)),
        }
    }
}

structstruck::strike! {
    #[strikethrough[allow(dead_code)]]
    #[strikethrough[derive(Debug, Deserialize)]]
    pub enum AddressResponse {
        IsAvailable ( struct {
            pub response: pub struct IsAvailableResponse {
                pub address: String,
                pub available: bool,
            },
        }),
        GetExpiry ( struct {
            pub response: pub struct GetExpiryResponse {
                pub message: String,
                pub expired: bool,
            },
        }),
        GetPublicInfo ( struct {
            pub response: pub struct GetPublicInfoResponse {
                pub address: String,
                pub message: String,
                pub expiration: struct GetPublicInfoExpiration {
                    pub expired: bool,
                },
                pub verification: struct GetPublicInfoVerification {
                    pub verified: bool,
                },
            },
        }),
        GetInfo ( struct {
            pub response: pub struct GetInfoResponse {
                pub address: String,
                pub message: String,
                pub expiration: struct GetInfoExpiration {
                    pub expired: bool,
                    pub will_expire: bool,
                    pub relative_time: String,
                },
                pub verification: struct GetInfoVerification {
                    pub verified: bool,
                },
                pub owner: String,
            }
        }),
    }
}
