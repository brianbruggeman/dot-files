#!/usr/sh

# -----------------------------------------------------------------------------
# Tmux and Virtual Environments
# -----------------------------------------------------------------------------
if [[ -n "$TMUX" ]] && [[ -n "$VIRTUAL_ENV" ]] ; then
    unset VIRTUALENV
fi;
