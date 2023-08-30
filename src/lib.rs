use clap::Subcommand;

pub mod account;
pub use account::Account;
pub mod address;
pub use address::{Address, AddressResponse, GetExpiry, GetInfo, GetPublicInfo, IsAvailable};
pub mod dns;
pub use dns::Dns;
pub mod email;
pub use email::Email;
pub mod now;
pub use now::Now;
pub mod pastebin;
pub use pastebin::Pastebin;
pub mod purl;
pub use purl::Purl;
pub mod status;
pub use status::Status;
pub mod theme;
pub use theme::Theme;
pub mod web;
pub use web::Web;
pub mod weblog;
pub use weblog::Weblog;

fn get<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, reqwest::Error> {
    reqwest::blocking::Client::new()
        .get(format!("https://api.omg.lol/{}", url))
        .send()?
        .error_for_status()?
        .json::<T>()
}

fn get_auth<T: serde::de::DeserializeOwned>(api_key: &str, url: &str) -> Result<T, reqwest::Error> {
    reqwest::blocking::Client::new()
        .get(format!("https://api.omg.lol/{}", url))
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {api_key}"))
        .send()?
        .error_for_status()?
        .json::<T>()
}

// TODO: gate clap derives behind crate feature, not needed for TUI/GUI
// TODO: allow content fields for some commands to provide filepaths, using the content of the file instead
#[derive(Subcommand)]
pub enum Commands {
    /// Get information and make changes to your account
    #[clap(visible_aliases = &["ac"])]
    Account {
        #[clap(subcommand)]
        command: Account,
    },
    /// Get information and make changes to your addresses
    #[clap(visible_aliases = &["a"])]
    Address {
        #[clap(subcommand)]
        command: Address,
    },
    /// Save omg.lol API key to config.toml, will prompt (vs env. var: OMGLOL_API_KEY)
    Auth {
        /// Linked omg.lol name for API key
        name: String,
    },
    /// Get the address directory, consisting of addresses that have opted-in to be listed
    #[clap(visible_aliases = &["dir"])]
    Directory,
    /// Adjust the switchboard / DNS records for your omg.lol address
    #[clap(visible_aliases = &["d"])]
    Dns {
        #[clap(subcommand)]
        command: Dns,
    },
    /// Manage the email configuration for an omg.lol address
    #[clap(visible_aliases = &["e"])]
    Email {
        #[clap(subcommand)]
        command: Email,
    },
    /// Manage your /now page
    #[clap(visible_aliases = &["n"])]
    Now {
        #[clap(subcommand)]
        command: Now,
    },
    /// Manage the pastebin for an omg.lol address
    #[clap(visible_aliases = &["p"])]
    Pastebin {
        #[clap(subcommand)]
        command: Pastebin,
    },
    /// Manage preferences for omg.lol accounts, addresses and objects
    #[clap(visible_aliases = &["pr"])]
    Preferences {
        /// Account to change settings for
        owner: String,
        /// ID of setting to update
        item: String,
        /// Value to set "item" to
        value: String,
    },
    /// Manage PURLs (Persistent URLs) for your omg.lol address
    #[clap(visible_aliases = &["u"])]
    Purl {
        #[clap(subcommand)]
        command: Purl,
    },
    /// Get service information about omg.lol
    Service,
    /// Manage the statuslog for an omg.lol address
    #[clap(visible_aliases = &["s"])]
    Status {
        #[clap(subcommand)]
        command: Status,
    },
    /// Set a default omg.lol address (and API key) from saved addresses
    Switch {
        /// new default omg.lol address, leave blank to list available addresses
        address: Option<String>,
    },
    /// Manage omg.lol profile themes
    #[clap(visible_aliases = &["t"])]
    Theme {
        #[clap(subcommand)]
        command: Theme,
    },
    /// Manage profile page and web stuff for an omg.lol address
    #[clap(visible_aliases = &["w"])]
    Web {
        #[clap(subcommand)]
        command: Web,
    },
    /// Manage the weblog for an omg.lol address
    #[clap(visible_aliases = &["b"])]
    Weblog {
        #[clap(subcommand)]
        command: Weblog,
    },
}

impl Commands {
    // TBD: Is there a more idiomatic / succinct approach to this?
    pub fn process(
        &self,
        _address: &str,
        api_key: &str,
    ) -> Result<CommandResponse, reqwest::Error> {
        match self {
            Commands::Account { command } => {
                command.process();
                Ok(CommandResponse::Todo(()))
            }
            Commands::Address { command } => {
                Ok(CommandResponse::Address(command.process(api_key)?))
            }
            Commands::Auth { name } => unreachable!("{name}"),
            Commands::Directory => todo!(),
            Commands::Dns { command } => todo!("{command:?}"),
            Commands::Email { command } => todo!("{command:?}"),
            Commands::Now { command } => todo!("{command:?}"),
            Commands::Pastebin { command } => todo!("{command:?}"),
            Commands::Preferences { owner, item, value } => todo!("{owner}, {item}, {value}"),
            Commands::Purl { command } => todo!("{command:?}"),
            Commands::Service => todo!(),
            Commands::Status { command } => todo!("{command:?}"),
            Commands::Switch { address: _ } => todo!(),
            Commands::Theme { command } => todo!("{command:?}"),
            Commands::Web { command } => todo!("{command:?}"),
            Commands::Weblog { command } => todo!("{command:?}"),
        }
    }
}

#[derive(Debug)]
pub enum CommandResponse {
    Todo(()),
    Address(AddressResponse),
}
