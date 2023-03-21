from socket import getfqdn
from typing import Dict


def hostname(config: Dict, envs: Dict) -> str:
    hostname = getfqdn()

    if not config["hostname"]["fqdn"]:
        hostname = hostname.split(".")[0]

    # TODO: handle cases where display is conditional/disabled.
    return hostname


def username(config: Dict, envs: Dict) -> str:
    username = envs.get("LOGNAME")

    if username is None:
        username = envs.get("USER")

    if username is None:
        return envs.get("USERNAME", "")

    return username


# TODO: how do we indicate if the data is worth showing?
def shell_level(config: Dict, envs: Dict) -> int:
    # TODO: handle cast/parse errors.
    return int(envs.get("SHLVL", "1"))
