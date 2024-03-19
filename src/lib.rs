//!
//!
//!
mod config;
mod true_path;

// ////////////////////////////////////////////////////////////////////////////
use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use glob::Pattern;
use tokio::fs;
use tokio::process::Command;
use tokio::io::{BufReader, AsyncBufReadExt};
use std::process::Stdio;
use home::home_dir;

use config::MdmConfig;
pub use true_path::true_path;


/// Retrieve the configuration for the dfs tool
pub async fn get_xdg_config_dir() -> anyhow::Result<PathBuf> {
    match env::var("XDG_CONFIG_HOME") {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(e) => {
            match home_dir() {
                Some(path) => Ok(path.join(".config/mdm")),
                None => Err(anyhow::anyhow!("Please set the XDG_CONFIG_HOME environment variable and rerun: {e}")),
            }
        }
    }
}

/// Retrieve the mdm configuration file
pub async fn get_config_file() -> anyhow::Result<PathBuf> {
    let config_dir = get_xdg_config_dir().await?;
    let config_file = format!("{}/config.toml", config_dir.display());
    Ok(true_path(&config_file))
}

pub async fn load_config() -> anyhow::Result<config::MdmConfig> {
    let config_file = get_config_file().await?;
    match fs::try_exists(&config_file).await {
        Ok(true) => {
            let contents = match fs::read_to_string(&config_file).await {
                Ok(contents) => contents,
                Err(e) => return Err(anyhow::anyhow!("Failed to read config file: {e}")),
            };
            parse_config(contents).await
        },
        Ok(false) => Ok(MdmConfig::default()),
        Err(e) => Err(anyhow::anyhow!("Failed to check for config file: {e}")),
    }
}

pub async fn parse_config(contents: impl AsRef<str>) -> anyhow::Result<config::MdmConfig> {
    let config: config::MdmConfig = toml::from_str(contents.as_ref())?;
    Ok(config)
}

pub async fn store_config(config: &config::MdmConfig) -> anyhow::Result<()> {
    let config_file = get_config_file().await?;
    if !fs::try_exists(&config_file).await? {
        let parent = config_file.parent().ok_or_else(|| anyhow!("Unable to determine parent directory"))?;
        fs::create_dir_all(parent).await?;
    }
    let contents = toml::to_string(config)?;
    fs::write(config_file, contents).await?;
    Ok(())
}

/// Capture all of the dot files within the specified path
pub async fn get_dotfiles_in_path(path: impl AsRef<Path>, ignored_paths: &[impl AsRef<Path>]) -> anyhow::Result<Vec<String>> {
    let mut dir = fs::read_dir(path).await?;
    let mut names = Vec::with_capacity(100);
    while let Some(entry) = dir.next_entry().await? {
        let filename = entry.file_name();
        if let Some(dotfile) = filename.to_str() {
            if !dotfile.starts_with(".") {
                continue;
            }
            if ignored_paths.iter().any(|ignored| Pattern::new(ignored.as_ref().to_str().unwrap()).unwrap().matches(dotfile)) {
                continue;
            }
            names.push(dotfile.to_string());
        }
    }
    Ok(names)
}

pub async fn exists(path: impl AsRef<Path>) -> bool {
    let path = true_path(path);
    match fs::metadata(&path).await.is_ok() {
        true => {
            println!("Exists: {}", path.display());
            true
        }
        false => {
            println!("Does not exist: {}", path.display());
            false
        }
    }
}

pub async fn is_symlink(path: impl AsRef<Path>) -> bool {
    let path = true_path(path);
    match fs::symlink_metadata(path).await {
        Ok(metadata) => metadata.file_type().is_symlink(),
        Err(_) => false,
    }
}

pub async fn is_live_symlink(path: impl AsRef<Path>) -> bool {
    let path = true_path(path);
    match exists(&path).await && is_symlink(&path).await {
        true => match fs::read_link(&path).await {
                Ok(target_path) => exists(&target_path).await,
                Err(_) => false,
            }
        false => false
    }
}


pub async fn symlink_dotfiles(dotfiles_path: impl AsRef<Path>, force: bool, ignored_paths: &[impl AsRef<Path>]) -> anyhow::Result<()> {
    let dotfiles_path = true_path(dotfiles_path);
    let dotfiles = get_dotfiles_in_path(&dotfiles_path, ignored_paths).await?;
    let home_dir = home_dir().ok_or_else(|| anyhow!("Unable to determine home directory"))?;
    let home_dotfiles = get_dotfiles_in_path(&home_dir, ignored_paths).await?;
    for dotfile in dotfiles {
        let dotfile_path = dotfiles_path.join(&dotfile);
        let home_dotfile_path = true_path(home_dir.join(&dotfile));
        match (home_dotfiles.contains(&dotfile), force) {
            (true, false) => {}
            (false, _) => match fs::symlink(&dotfile_path, &home_dotfile_path).await {
                Ok(_) => println!("Created symlink for {} -> {}", dotfile_path.display(), home_dotfile_path.display()),
                Err(e) => println!("Failed to create symlink for {} -> {}: {e}", dotfile_path.display(), home_dotfile_path.display()),
            },
            (_, true) => {
                if let Err(why) = fs::remove_file(&home_dotfile_path).await {
                    println!("Failed to remove existing file at {}: {why}", home_dotfile_path.display());
                    continue;
                }
                match fs::symlink(&dotfile_path, &home_dotfile_path).await {
                    Ok(_) => println!("Created symlink for {} -> {}", dotfile_path.display(), home_dotfile_path.display()),
                    Err(e) => println!("Failed to create symlink for {} -> {}: {e}", dotfile_path.display(), home_dotfile_path.display()),
                }
            }
        }
    }
    remove_broken_symlinks(ignored_paths).await?;
    Ok(())
}


pub async fn remove_broken_symlinks(ignored_paths: &[impl AsRef<Path>]) -> anyhow::Result<()> {
    let home_dir = home_dir().ok_or_else(|| anyhow!("Unable to determine home directory"))?;
    let home_dotfiles = get_dotfiles_in_path(&home_dir, ignored_paths).await?;
    for dotfile in home_dotfiles {
        let path = home_dir.join(&dotfile);
        if is_symlink(&path).await && !is_live_symlink(&path).await {
            match fs::remove_file(&path).await {
                Err(why) => println!("Failed to remove broken symlink at {}: {why}", path.display()),
                Ok(_) => println!("Removed broken symlink at {}", path.display()),
            }
        }
    }
    Ok(())
}


pub async fn sync_github(repo_path: impl AsRef<Path>) -> Result<()> {
    let repo_path = repo_path.as_ref().to_path_buf();
    println!("Starting GitHub sync for repository at {:?}", repo_path);
    let stashed = stash_local_changes(&repo_path).await?;
    pull_changes(&repo_path).await?;
    if stashed {
        if let Err(e) = apply_stashed_changes(&repo_path).await {
            return Err(anyhow!("Failed to apply stashed changes, manual intervention required: {}", e));
        }
        commit_changes(&repo_path).await?;
        push_changes(&repo_path).await?;
    }
    println!("GitHub sync completed successfully.");
    Ok(())
}

async fn run_command(command_line: &str, repo_path: &PathBuf) -> Result<()> {
    let parts: Vec<&str> = command_line.split_whitespace().collect();
    let (command, args) = parts.split_first().ok_or_else(|| anyhow!("Invalid command line"))?;

    println!("Executing: {}", command_line);
    let mut command = Command::new(command);
    command.args(args).current_dir(repo_path).stdout(Stdio::piped());

    let mut child = command.spawn()?;
    let stdout = child.stdout.take().expect("Failed to capture stdout");

    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }

    let status = child.wait().await?;
    if !status.success() {
        return Err(anyhow!("Command `{}` failed", command_line));
    }

    Ok(())
}

async fn stash_local_changes(repo_path: &PathBuf) -> Result<bool> {
    run_command("git stash push -m 'Automatic stash by sync script'", repo_path).await?;

    // Simplified for demonstration; you might want to check if stashing was actually needed.
    Ok(true)
}

async fn pull_changes(repo_path: &PathBuf) -> Result<()> {
    run_command("git pull", repo_path).await
}

async fn apply_stashed_changes(repo_path: &PathBuf) -> Result<()> {
    run_command("git stash pop", repo_path).await
}

async fn commit_changes(repo_path: &PathBuf) -> Result<()> {
    run_command("git commit -am 'Automatic commit by mdm script'", repo_path).await
}

async fn push_changes(repo_path: &PathBuf) -> Result<()> {
    run_command("git push", repo_path).await
}
