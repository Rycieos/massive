from data import Data


def generate_prompt(data: Data) -> str:
    return data.hostname()
