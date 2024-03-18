# dot-files
Publically scrubbed dot-files


## Purpose

This includes my publicly facing dot-files.  I have included a tool that lets me sync these files to my home directory through a cron.

The tool is pretty simple.  It's a rust binary that runs on-demand.  It has two basic pieces:  link and sync.  The link command will generate links for the files found within my dot-files directory.  The sync command will sync my repository with github.  So when I make updates, I run `mdm sync`.  When I create new dot files, I run `mdm link` and then run `mdm sync`.

I also have an attempt to exclude files I don't really want to sync, using the .gitignore as a first guard.  Additionally, this tool is XDG compliant and will look for the XDG_CONFIG_HOME environment variable.  If it's not set, it will default to $HOME/.config.  The file should be found under `XDG_CONFIG_HOME/mdm/config.toml`.  It includes a list of files to exclude from the sync.  Additionally, you can set (and forget) where the dotfiles live independent of the repo.

Maybe in the future I'll use Chat GPT's API to generate the commit messages, but for now, it's a simple and mostly useless "Automatic commit by mdm script" message.


## Installation

The installation is basic for rust.  You can just run `cargo install --path .` from the root of the repository.  This will install the binary to your cargo bin directory.  If you don't have that in your path, you can add it to your path or copy the binary to a directory in your path.