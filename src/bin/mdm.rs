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

        /// Profile to use
        #[clap(short, long, default_value="default")]
        profile: String,

        /// Path to the folder containing the dotfiles
        path: Option<PathBuf>,
    },

    /// Sync repository to remote host (github)
    Sync {
        /// Profile to use
        #[clap(short, long, default_value="default")]
        profile: String,

        /// Path to the repository
        path: Option<PathBuf>,
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config_path = mdm::get_config_file().await?;
    let mut config = mdm::load_config(config_path).await?;
    let mdm = Mdm::parse();

    match mdm.subcommand {
        SubCommands::Config => {
            println!("{:#?}", config);
        },

        SubCommands::Link { path, force, profile } => {
            let profile = config.get_mut(&profile).unwrap_or_else(|| {
                eprintln!("Profile not found: {profile}");
                std::process::exit(1);
            });
            let path = match path {
                Some(path) => {
                    let path = mdm::true_path(path);
                    match path.exists() {
                        true => path,
                        false => return Err(anyhow::anyhow!("Dotfiles path does not exist: {}", path.display())),
                    }
                }
                None => profile.dots_path().to_owned(),
            };
            profile.with_dots_path(&path);
            handlers::link_dotfiles(profile.dots_path(), force, profile.ignored_paths()).await?;
            mdm::store_config(&config).await?;
        },
        SubCommands::Sync { path, profile } => {
            let profile = config.get_mut(&profile).unwrap_or_else(|| {
                eprintln!("Profile not found: {}", profile);
                std::process::exit(1);
            });
            let path = match path {
                Some(path) => {
                    let path = mdm::true_path(path);
                    match path.exists() {
                        true => path,
                        false => return Err(anyhow::anyhow!("Dotfiles path does not exist: {}", path.display())),
                    }
                }
                None => profile.repo_path().to_owned(),
            };
            profile.with_repo_path(&path);
            handlers::sync_dotfiles(path).await?;
            mdm::store_config(&config).await?;
        },
    }

    Ok(())
}