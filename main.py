#!/usr/bin/env python3

from importlib import import_module
import logging
import os
import toml
from typing import Callable, Dict

from context import Context

logger = logging.getLogger(__name__)


def load_theme(theme: str) -> Callable:
    try:
        plugin = import_module("." + theme, package="plugins")
    except (ModuleNotFoundError, ImportError):
        logger.exception("Could not load plugin '%s'", theme)
        raise

    return plugin.generate_prompt


def load_config() -> Dict:
    # TODO: martial this into a structure and apply defaults.
    return toml.load("config.toml")


def main():
    config = load_config()
    entrypoint = load_theme(config["theme"])

    global_cache = dict()
    client_cache = dict()

    context = Context(config, os.environ, global_cache, client_cache)

    print(entrypoint(context, config))


if __name__ == "__main__":
    main()
