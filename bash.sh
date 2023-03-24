req_fifo=/tmp/massive_in.fifo
resp_fifo=/tmp/massive_out.fifo

function __massive_setup() {
  # Ensure that $COLUMNS gets set
  shopt -s checkwinsize

  # register client.
  __massive_send_payload $'\x01' "$BASHPID"

  if [[ ! -p $resp_fifo ]]; then
    [[ -r $resp_fifo ]] && rm $resp_fifo
    mkfifo "$resp_fifo"
  fi

  PROMPT_COMMAND=__massive_set_prompt
}

function __massive_bye() {
  __massive_send_payload $'\x02' "$BASHPID"
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

  __massive_send_payload $'\x03' "${BASHPID}${sep}${resp_fifo}${sep}bash${sep}${COLUMNS-}${sep}${exit_status}${sep}${pipe_status}${sep}${jobs_running}${sep}${jobs_sleeping}${sep}${PWD-}${sep}${env_vars}"

  #printf '%s\n' "$(<"$resp_fifo")"
  PS1="$(<"$resp_fifo")"
}

function __massive_send_payload() { # type, payload
  local payload_len payload_len_hex_big payload_len_hex_small

  printf -v _ '%s%n' "$2" payload_len
  # split into two bytes.
  local big=$((payload_len >> 8))
  local small=$((payload_len % 256))
  printf -v payload_len_hex_big '%x' "$big"
  printf -v payload_len_hex_small '%x' "$small"

  printf "\\x${payload_len_hex_big}\\x${payload_len_hex_small}%s%s" "$1" "$2" >"$req_fifo"
}

#__massive_setup
