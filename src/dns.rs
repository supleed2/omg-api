use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Dns {
    /// Get a list of all your DNS records
    GetRecords,
    /// Add a new DNS record
    AddRecord {
        /// Temporary JSON data input
        json_data: String,
    },
    /// Update an existing DNS record
    UpdateRecord {
        /// Temporary JSON data input
        json_data: String,
    },
    /// Delete a DNS record
    DeleteRecord {
        /// ID of the DNS record to delete
        id: String,
    },
}

impl Dns {
    pub fn process(&self, _address: &str) {
        match self {
            Dns::GetRecords => todo!(),
            Dns::AddRecord { json_data: _ } => todo!(),
            Dns::UpdateRecord { json_data: _ } => todo!(),
            Dns::DeleteRecord { id: _ } => todo!(),
        }
    }
}
