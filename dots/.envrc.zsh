#!/usr/sh

check_for_zsh() {
    if [ -n "$ZSH_VERSION" ]; then
        return 0
    else
        return 1
    fi
}

updates_for_zsh() {
    # -----------------------------------------------------------------------------
    # Zsh History
    # -----------------------------------------------------------------------------
    export HISTFILE=${ZDOTDIR:-$HOME}/.zsh_history
    export HISTSIZE=10000000
    export SAVEHIST=10000000
}

main() {
    check_for_zsh && updates_for_zsh
}

main
