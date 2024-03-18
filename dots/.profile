if [ -s ~/.envrc ]; then source ~/.envrc; fi

path_extend() {
    if [ -d "$1" ]; then
        val=${1:-" "};
        script="import os; env = os.environ; path = env['PATH'].split(':'); path.append('$val' if '$val' not in path else ''); path=':'.join(p for p in path if p.strip()); print(path)";
        new_path=$(python -c "$script");
        # echo "----------------------------------------"
        # echo " Adding: $1"
        # echo "----------------------------------------"
        # echo "before: $PATH"
        PATH=$new_path;
        # echo " after: $PATH"
        # echo "----------------------------------------"
        export PATH;
    fi
}


path_insert() {
    if [ -d "$1" ]; then
        val=${1:-" "};
        script="import os; env = os.environ; path = env['PATH'].split(':'); path.insert(0, '$val' if '$val' not in path else ''); path=':'.join(p for p in path if p.strip()); print(path)";
        new_path=$(python -c "$script");
        # echo "----------------------------------------"
        # echo " Inserting: $1"
        # echo "----------------------------------------"
        # echo "before: $PATH"
        PATH=$new_path;
        # echo " after: $PATH"
        # echo "----------------------------------------"
        export PATH;
    fi
}

path_insert "/sbin"
path_insert "/usr/sbin"
path_insert "/bin"
path_insert "/usr/bin"
path_insert "/usr/local/bin"
path_insert "/opt/homebrew/bin"
path_insert "$HOME/.local/bin"

if [ -s ~/.bin/tmuxinator.zsh ]; then source ~/.bin/tmuxinator.zsh; fi

export LC_ALL=en_US.UTF-8
export LANG=en_US.UTF-8

. "$HOME/.cargo/env"

# Generated for envman. Do not edit.
[ -s "$HOME/.config/envman/load.sh" ] && source "$HOME/.config/envman/load.sh"
