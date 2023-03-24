req_fifo=/tmp/massive_in.fifo
resp_fifo=/tmp/massive_out.fifo

function setup() {
  # register client.
  send_payload $'\x01' "$BASHPID"

  if [[ ! -p $resp_fifo ]]; then
    mkfifo "$resp_fifo"
  fi
}

function bye() {
  send_payload $'\x02' "$BASHPID"
}

function set_prompt() {
  local req_fifo=/tmp/massive_in.fifo
  local resp_fifo=/tmp/massive_out.fifo

  send_payload $'\x03' "$BASHPID"$'\x1f'"$resp_fifo"$'\x1fbash\x1f'"$PWD"

  printf '%s\n' "$(<"$resp_fifo")"
}

function send_payload() { # type, payload
  local payload_len payload_len_hex

  printf -v _ '%s%n' "$2" payload_len
  printf -v payload_len_hex '%x' "$payload_len"

  printf "\\x00\\x${payload_len_hex}%s%s" "$1" "$2" >"$req_fifo"
}
