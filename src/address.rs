use clap::Subcommand;
use serde::Deserialize;

use crate::{get, get_auth};

#[derive(Debug, Subcommand)]
pub enum Address {
    /// Get information about the availability of an address
    IsAvailable {
        /// Address to get availability of
        address: String,
    },
    /// Get the expiration date for an address
    GetExpiry {
        /// Address to get availability of
        address: String,
    },
    /// Get limited (public) information about an address (no auth required)
    GetPublicInfo {
        /// Address to get availability of
        address: String,
    },
    /// Get comprehensive information about an address
    GetInfo {
        /// Address to get availability of
        address: String,
    },
}

impl Address {
    pub fn process(&self, api_key: &str) -> Result<AddressResponse, reqwest::Error> {
        match self {
            Address::IsAvailable => todo!(),
            Address::GetExpiry => todo!(),
            Address::GetPublicInfo => todo!(),
            Address::GetInfo => todo!(),
        }
    }
}
