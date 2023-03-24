#!/usr/bin/env python3

import click
import logging
import os
import subprocess
import sys

from config import load_config
from context import Context
from env_vars import split_env_vars
from theme import load_theme

logger = logging.getLogger(__name__)

# TODO: make this a static location unique to user, like ~/.config/massive/
in_fifo = "/tmp/massive_in.fifo"
out_fifo = "/tmp/massive_out.fifo"


@click.group()
def main():
    pass


@main.command()
@click.argument("shell", type=click.Choice(["bash"]))
def init(shell):
    # TODO: detect when already running and exit.
    subprocess.Popen(
        [sys.argv[0], "daemon"],
        start_new_session=True,
        # This doesn't work because it can't find the path to itself.
        # cwd="/",
        stdout=subprocess.DEVNULL,
        stderr=subprocess.STDOUT,
    )

    if shell == "bash":
        init_file = "bash.sh"

    with open(init_file) as file_handle:
        print(file_handle.read())


@main.command()
def daemon():
    config = load_config()
    entrypoint = load_theme(config["theme"])

    global_cache = dict()
    clients_cache = dict()

    if not os.path.exists(in_fifo):
        os.mkfifo(in_fifo)

    while True:
        # If we try to read from the same file object, Python see the EOF and
        # will always return empty string. Reseting the file object will make
        # this read block.
        with open(in_fifo, "rb") as in_file:
            header = in_file.read(3)

            # First two bytes is message length.
            # Third byte is message type.
            length = int.from_bytes(header[:2], "big")
            data = in_file.read(length)
            payload = data.decode("utf-8")

            if header[2] == 1:  # hello
                logger.debug("hello client", payload)
                clients_cache[payload] = dict()

            elif header[2] == 2:  # bye
                logger.debug("bye client", payload)
                clients_cache.pop(payload, None)
                if len(clients_cache) == 0:
                    sys.exit()

            elif header[2] == 3:  # prompt request
                sections = payload.split("\x1f", maxsplit=9)

                client_id = sections[0]
                resp_fifo = sections[1]
                shell = sections[2]
                terminal_width = sections[3]
                exit_status = sections[4]
                pipe_status = sections[5].split(" ")
                jobs_running = sections[6]
                jobs_sleeping = sections[7]
                current_dir = sections[8]
                env_vars = sections[9]

                try:
                    terminal_width = int(terminal_width)
                except ValueError:
                    terminal_width = 0

                pipe_status = [int(i) for i in pipe_status]

                parsed_env_vars = split_env_vars(shell, env_vars)

                context = Context(
                    config,
                    global_cache,
                    clients_cache[client_id],
                    shell,
                    terminal_width,
                    exit_status,
                    pipe_status,
                    jobs_running,
                    jobs_sleeping,
                    current_dir,
                    parsed_env_vars,
                )

                prompt = entrypoint(context, config)

                with open(resp_fifo, "w") as out_file:
                    out_file.write(prompt)


@main.command()
def prompt():
    config = load_config()
    entrypoint = load_theme(config["theme"])

    context = Context(
        config,
        None,
        None,
        os.environ.get("SHELL", ""),
        0,
        0,
        [],
        0,
        0,
        "",
        os.environ,
    )

    print(entrypoint(context, config))


if __name__ == "__main__":
    main()
