use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Subcommand;

use crate::DEBUG_PROFILE;

mod check;
mod new;
mod update;

#[derive(Subcommand)]
pub enum DevCommands {
    New {
        path: PathBuf,
        #[arg(long)]
        no_git: bool,
    },
    Check,
    Update,
}

impl DevCommands {
    pub fn run(self) -> Result<()> {
        match self {
            DevCommands::New { path, no_git } => {
                if DEBUG_PROFILE {
                    bail!("Disabled in the debug build");
                }

                new::new(&path, no_git).context(INIT_ERR)
            }
            DevCommands::Check => check::check(),
            DevCommands::Update => update::update(),
        }
    }
}

const INIT_ERR: &str = "Initialization failed.
After resolving the issue, delete the `rustlings` directory (if it was created) and try again";
