req_fifo=/tmp/massive_in.fifo
resp_fifo=/tmp/massive_out.fifo

function __massive_setup() {
  # Ensure that $COLUMNS gets set
  shopt -s checkwinsize

  # register client.
  printf '\x01%s' >"$req_fifo" "$BASHPID"

  if [[ ! -p $resp_fifo ]]; then
    [[ -r $resp_fifo ]] && rm $resp_fifo
    mkfifo "$resp_fifo"
  fi

  PROMPT_COMMAND=__massive_set_prompt
}

function __massive_bye() {
  printf '\x02%s' >"$req_fifo" "$BASHPID"
}

function __massive_set_prompt() {
  # This line MUST be first.
  local exit_status="$?" pipe_status="${PIPESTATUS[*]}"

  # TODO: find a way to not run these expensive (1-2ms) subshells unless the
  # feature is enabled.

  # The $(...) syntax strips trailing newlines, so add a character to the end
  # then remove it to prevent that. Otherwise 0 and 1 jobs look the same.
  local jobs_running="$(jobs -r; printf x)"
  jobs_running="${jobs_running//[!$'\n']}"
  jobs_running="${#jobs_running}"

  local jobs_sleeping="$(jobs -s; printf x)"
  jobs_sleeping="${jobs_sleeping//[!$'\n']}"
  jobs_sleeping="${#jobs_sleeping}"

  local env_vars="$(builtin export -p)"

  local sep=$'\x1f'

  # Open for read as well as write so the write doesn't block.
  # https://unix.stackexchange.com/a/522940/421569
  builtin printf '\x03%s' 1<>"$req_fifo" >"$req_fifo" "${BASHPID}${sep}${resp_fifo}${sep}bash${sep}${COLUMNS-}${sep}${exit_status}${sep}${pipe_status}${sep}${jobs_running}${sep}${jobs_sleeping}${sep}${PWD-}${sep}${env_vars}"

  # TODO: find a way to timeout this command.
  read -r PS1 <"$resp_fifo"
}

#__massive_setup
