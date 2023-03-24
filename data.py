from socket import getfqdn
from typing import Dict


def hostname(context, envs: Dict) -> str:
    hostname = getfqdn()

    if not context.config["hostname"]["fqdn"]:
        hostname = hostname.split(".")[0]

    # TODO: handle cases where display is conditional/disabled.
    return hostname


def username(context, envs: Dict) -> str:
    username = envs.get("LOGNAME")

    if username is None:
        username = envs.get("USER")

    if username is None:
        return envs.get("USERNAME", "")

    return username


def work_dir(context, envs: Dict) -> str:
    if not (work_dir := context.current_dir):
        work_dir = envs.get("PWD", "")

    home = envs.get("HOME")
    if home:
        work_dir = work_dir.replace(home, "~", 1)

    return work_dir


# TODO: how do we indicate if the data is worth showing?
def shell_level(context, envs: Dict) -> int:
    # TODO: handle cast/parse errors.
    return int(envs.get("SHLVL", "1"))
