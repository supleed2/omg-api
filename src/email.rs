use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Email {
    /// Get forwarding address(es)
    GetForwards,
    /// Set forwarding address(es)
    SetForwards {
        /// Addresses to forward emails to
        json_data: String,
    },
}

impl Email {
    pub fn process(&self, _address: &str) {
        match self {
            Email::GetForwards => todo!(),
            Email::SetForwards { json_data: _ } => todo!(),
        }
    }
}
