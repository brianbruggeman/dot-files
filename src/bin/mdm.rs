//!
//!
//!
mod cli;

use cli::handlers;

// ////////////////////////////////////////////////////////////////////////////
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Mdm {
    #[clap(subcommand)]
    subcommand: SubCommands,
}


#[derive(Parser, Debug)]
pub enum SubCommands {
    /// Displays config information
    Config,

    /// Link local dotfiles to HOME
    Link {
        /// Force the linkage
        #[clap(short, long)]
        force: bool,
        /// Path to the folder containing the dotfiles
        path: Option<PathBuf>,
    },

    /// Sync repository to remote host (github)
    Sync {
        /// Path to the repository
        path: Option<PathBuf>,
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = mdm::load_config().await?;
    let mdm = Mdm::parse();

    match mdm.subcommand {
        SubCommands::Config => {
            println!("{:#?}", config);
        },

        SubCommands::Link { path, force } => {
            let path = match path {
                Some(path) => {
                    let path = mdm::true_path(path);
                    match path.exists() {
                        true => path,
                        false => return Err(anyhow::anyhow!("Dotfiles path does not exist: {}", path.display())),
                    }
                }
                None => config.dots_path().to_owned(),
            };
            let config = config.as_builder().dots_path(&path).as_config();
            mdm::store_config(&config).await?;
            handlers::link_dotfiles(config.dots_path(), force, config.ignored_paths()).await?;
        },
        SubCommands::Sync { path } => {
            let path = match path {
                Some(path) => {
                    let path = mdm::true_path(path);
                    match path.exists() {
                        true => path,
                        false => return Err(anyhow::anyhow!("Dotfiles path does not exist: {}", path.display())),
                    }
                }
                None => config.repo_path().to_owned(),
            };
            let config = config.as_builder().repo_path(&path).as_config();
            mdm::store_config(&config).await?;
            handlers::sync_dotfiles(path).await?;
        },
    }

    Ok(())
}