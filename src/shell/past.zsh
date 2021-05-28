# Source this in your ~/.zshrc
autoload -U add-zsh-hook

_past_preexec(){
    past $@
}

_past_precmd(){
    # Perhaps you want to implement your own precmd here
    # To read the exit code of the command.
    # Here's how https://github.com/ellie/atuin/blob/main/src/shell/atuin.zsh
}

add-zsh-hook preexec _past_preexec
# add-zsh-hook precmd _past_precmd