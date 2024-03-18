# set -x
# History
setopt INC_APPEND_HISTORY        # add to history file immediately
setopt SHARE_HISTORY             # share history between all sessions.
setopt APPEND_HISTORY            # append to history file, don't overwrite
setopt HIST_IGNORE_ALL_DUPS      # delete old recorded entry if new entry is a duplicate
setopt HIST_REDUCE_BLANKS        # remove superfluous blanks before recording entry
HISTSIZE=10000                   # set history file size to 10000
SAVEHIST=10000                   # save 10000 lines of history

# From: http://unix.stackexchange.com/a/273863
setopt BANG_HIST                 # Treat the '!' character specially during expansion
setopt EXTENDED_HISTORY          # Write the history file in the ":start:elapsed;command" format
setopt HIST_EXPIRE_DUPS_FIRST    # Expire duplicate entries first when trimming history
setopt HIST_FIND_NO_DUPS         # Do not display a line previously found
setopt HIST_IGNORE_SPACE         # Don't record an entry starting with a space
setopt HIST_SAVE_NO_DUPS         # Don't write duplicate entries in the history file
setopt HIST_VERIFY               # Don't execute immediately upon history expansion
setopt HIST_BEEP                 # Beep when accessing nonexistent history

# History
autoload -U up-line-or-beginning-search
autoload -U down-line-or-beginning-search
zle -N up-line-or-beginning-search
zle -N down-line-or-beginning-search
bindkey "^[[A" up-line-or-beginning-search # Up
bindkey "^[[B" down-line-or-beginning-search # Down

# For ZSH-Completions
autoload -Uz compinit && compinit

# Initializes starship
eval "$(starship init ${SHELL})"

# Initializes fzf
[ -f ~/.fzf.zsh ] && source ~/.fzf.zsh

# setup zsh history
substring_search=/usr/local/share/zsh-history-substring-search/zsh-history-substring-search.zsh \
	&& [ -f $substring_search ] && source $substring_search || :

[[ /usr/local/bin/kubectl ]] && source <(kubectl completion zsh)

### MANAGED BY RANCHER DESKTOP START (DO NOT EDIT)
export PATH="/Users/brianbruggeman/.rd/bin:$PATH"
### MANAGED BY RANCHER DESKTOP END (DO NOT EDIT)

[ -f ~/.we/alias ] && source ~/.we/alias
# Wizehire Bash Profile

# These flags will tell docker and docker-compose to always use the new build tools
export COMPOSE_DOCKER_CLI_BUILD=1
export DOCKER_BUILDKIT=1
# We start many services at once, so we should naturally wait longer
export COMPOSE_HTTP_TIMEOUT=180

# this is similar to "clear" but actually flush out old logs
alias empty="clear && printf '\e[3J'"
# allow you to not have to type the config while developing
alias wecompose="docker-compose -p main -f docker-compose.yml -f docker-compose.code.yml -f docker-compose.arm.yml"


# Generated for envman. Do not edit.
[ -s "$HOME/.config/envman/load.sh" ] && source "$HOME/.config/envman/load.sh"

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
