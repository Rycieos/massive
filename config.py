import toml
from typing import Dict


def load_config() -> Dict:
    # TODO: martial this into a structure and apply defaults.
    return toml.load("config.toml")
