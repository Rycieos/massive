from typing import Dict


def split_env_vars(shell: str, payload: str) -> Dict[str, str]:
    if shell == "bash":
        return split_bash_vars(payload)

    return dict()


def split_bash_vars(payload: str) -> Dict[str, str]:
    env_vars: Dict[str, str] = dict()

    for var in payload.split("declare -x "):
        if var:
            key, value = var.split('="', maxsplit=1)
            value = value.removesuffix("\n").removesuffix('"')
            value = value.replace('\\"', '"')
            env_vars[key] = value

    return env_vars
