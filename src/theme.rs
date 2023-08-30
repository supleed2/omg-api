use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Theme {
    /// List available omg.lol profile themes
    List,
    /// Get information about a specific theme
    Info {
        /// ID of the theme to get information for
        id: String,
    },
    /// Get a preview (HTML) of a theme
    Preview {
        /// ID of the theme to get a preview (HTML) of
        id: String,
    },
}

impl Theme {
    pub fn process(&self, _address: &str) {
        match self {
            Theme::List => todo!(),
            Theme::Info { id: _ } => todo!(),
            Theme::Preview { id: _ } => todo!(),
        }
    }
}
