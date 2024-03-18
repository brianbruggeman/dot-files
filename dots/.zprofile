if [ -s ~/.envrc ]; then source ~/.envrc; fi

typeset -U path

# Just to cover all the bases...
#   ~/.local/bin
#   ~/.local/sbin
path=(
    $HOME/.cargo/bin
    $HOME/.local/bin
    /opt/homebrew/bin
    $(/opt/homebrew/bin/brew --prefix)/opt/python/libexec/bin
    /usr/local/bin
    /usr/local/sbin
    /usr/local/share
    /usr/bin
    /usr/sbin
    /bin
    /sbin
    $path[@]
)

