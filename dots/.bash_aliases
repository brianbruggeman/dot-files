# Dynamic loading of .bash_aliases.* files
for file in ~/.bash_aliases.*; do
	# Source the file if it's not empty
    if [ -s "$file" ]; then
        source "$file"
    fi
done

alias sen="docker run -v /var/run/docker.sock:/run/docker.sock -ti -e TERM tomastomecek/sen"
alias subl='reattach-to-user-namespace subl'
alias hist='history 1'
alias k='kubectl'
alias mk='make'
alias lx='exa --git -l --color-scale --time-style=long-iso "$@"'
alias wee='we e'
