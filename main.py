#!/usr/bin/env python3

import click
import logging
import os
import time

from context import Context
from config import load_config
from theme import load_theme

logger = logging.getLogger(__name__)

# TODO: make this a static location unique to user, like ~/.config/massive/
in_fifo = "/tmp/massive_in.fifo"
out_fifo = "/tmp/massive_out.fifo"


@click.group()
def main():
    pass


@main.command()
def daemon():
    config = load_config()
    entrypoint = load_theme(config["theme"])

    global_cache = dict()
    clients_cache = dict()

    if not os.path.exists(in_fifo):
        os.mkfifo(in_fifo)

    with open(in_fifo, "rb") as in_file:
        while True:
            header = in_file.read(3)
            if not header:
                time.sleep(.001)
                continue

            # First two bytes is message length.
            # Third byte is message type.
            length = int.from_bytes(header[:2], "big")
            data = in_file.read(length)
            payload = data.decode("utf-8")

            if header[2] == 1:  # hello
                clients_cache[payload] = dict()
                print("hello client", payload)

            elif header[2] == 2:  # bye
                clients_cache.pop(payload, None)
                print("bye client", payload)

            elif header[2] == 3:  # prompt request
                sections = payload.split("\x1f")

                client_id = sections[0]
                resp_fifo = sections[1]
                shell = sections[2]
                work_dir = sections[3]

                context = Context(
                    config, os.environ, global_cache, clients_cache[client_id]
                )

                prompt = entrypoint(context, config)

                with open(resp_fifo, "w") as out_file:
                    out_file.write(prompt)


@main.command()
def prompt():
    config = load_config()
    entrypoint = load_theme(config["theme"])

    global_cache = dict()
    client_cache = dict()

    context = Context(config, os.environ, global_cache, client_cache)

    click.echo(entrypoint(context, config))


if __name__ == "__main__":
    main()
