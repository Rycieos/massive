
function set_prompt() {
  local req_fifo=/tmp/massive_in.fifo
  local resp_fifo=/tmp/massive_out.fifo

  local payload payload_len payload_len_hex

  # register client.
  #echo "$BASHPID" >&"$fd_out"

  if [[ ! -p $resp_fifo ]]; then
    mkfifo "$resp_fifo"
  fi

  payload="$BASHPID"$'\x1f'"$resp_fifo"$'\x1fbash\x1f'"$PWD"

  printf -v _ '%s%n' "$payload" payload_len
  printf -v payload_len_hex '%x' "$payload_len"

  printf "\\x${payload_len_hex}%s" "$payload" >"$req_fifo"

  printf '%s\n' "$(<"$resp_fifo")"
}
