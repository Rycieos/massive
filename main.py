#!/usr/bin/env python3

import click
import logging
import os

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
            length = in_file.read(1)
            length = int.from_bytes(length, "big")
            data = in_file.read(length)

            if data:
                print(data)
                print(data.decode("utf-8"))
                sections = data.decode("utf-8").split("\x1f")
                print(sections)

                client_id = sections[0]
                resp_fifo = sections[1]
                shell = sections[2]
                work_dir = sections[3]

                client_cache = dict()
                clients_cache[client_id] = client_cache

                context = Context(config, os.environ, global_cache, client_cache)

                prompt = entrypoint(context, config)
                print(prompt)

                with open(resp_fifo, "wb") as out_file:
                    out_file.write(bytes(prompt, "utf-8"))


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
