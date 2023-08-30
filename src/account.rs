use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Account {
    /// Get information about your account
    #[clap(visible_alias = "gi")]
    GetInfo {
        /// Email of your omg.lol account
        email: String,
    },
    /// Get all addresses associated with your account
    #[clap(visible_alias = "ga")]
    GetAddresses {
        /// Email of your omg.lol account
        email: String,
    },
    /// Get the name associated with your account
    #[clap(visible_alias = "gn")]
    GetName {
        /// Email of your omg.lol account
        email: String,
    },
    /// Update the name associated with your account
    #[clap(visible_alias = "sn")]
    SetName {
        /// Email of your omg.lol account
        email: String,
        /// Name to set for your account
        name: String,
    },
    /// Get all sessions associated with your account
    #[clap(visible_alias = "gs")]
    GetSessions {
        /// Email of your omg.lol account
        email: String,
    },
    /// Delete a session from your account
    #[clap(visible_alias = "rs")]
    RemoveSession {
        /// Email of your omg.lol account
        email: String,
        /// ID of the session to remove
        session_id: String,
    },
    /// Get settings associated with your account
    #[clap(visible_alias = "gset")]
    GetSettings {
        /// Email of your omg.lol account
        email: String,
    },
    /// Update settings associated with your account
    #[clap(visible_alias = "sset")]
    SetSettings {
        /// Email of your omg.lol account
        email: String,
        /// Temporary JSON data input
        json_data: String,
    },
}

impl Account {
    pub fn process(&self) {
        match self {
            Account::GetInfo { email: _ } => todo!(),
            Account::GetAddresses { email: _ } => todo!(),
            Account::GetName { email: _ } => todo!(),
            Account::SetName { email: _, name: _ } => todo!(),
            Account::GetSessions { email: _ } => todo!(),
            Account::RemoveSession { email: _, session_id: _ } => todo!(),
            Account::GetSettings { email: _ } => todo!(),
            Account::SetSettings { email: _, json_data: _ } => todo!(),
        }
    }
}
