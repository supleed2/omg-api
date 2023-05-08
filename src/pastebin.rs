use clap::Subcommand;

#[derive(Subcommand)]
pub enum Pastebin {
    /// Get a specific paste for an omg.lol address
    Get {
        /// Name of the paste to get
        name: String,
    },
    /// Get all pastes for an omg.lol address
    GetAll,
    /// Get all public pastes for an omg.lol address
    GetAllPublic,
    /// Create/update a paste for an omg.lol address
    Set {
        /// Name of the paste to create (and the address used to retrieve it)
        name: String,
        /// Content of the paste
        content: String,
    },
    /// Delete a paste for an omg.lol address
    Delete {
        /// Name of the paste to delete
        name: String,
    },
}

impl Pastebin {
    pub fn process(&self, _address: &Option<String>) {
        match self {
            Pastebin::Get { name: _ } => todo!(),
            Pastebin::GetAll => todo!(),
            Pastebin::GetAllPublic => todo!(),
            Pastebin::Set { name: _, content: _ } => todo!(),
            Pastebin::Delete { name: _ } => todo!(),
        }
    }
}
