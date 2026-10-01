# runIt zsh widget: Alt-R picks a command from the notes and inserts it at the
# cursor. Load it from ~/.zshrc: source /path/to/runIt/shell/runit.zsh
_runit_widget() {
	local cmd saved
	# zle turns the terminal echo off (it draws the keys itself): turn it back
	# on while runit asks for the <param> values, then give zle its mode back.
	saved=$(stty -g </dev/tty)
	stty echo </dev/tty
	cmd=$(RUNIT_ROOT=~/documents/memento/fiches runit)
	stty $saved </dev/tty
	if [[ -n $cmd ]]; then
		LBUFFER+=$cmd
	fi
	zle reset-prompt
}

zle -N _runit_widget
bindkey '^[r' _runit_widget
