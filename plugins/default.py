def generate_prompt(context, config) -> str:
    prompt = f"[{context.username()}"

    if hostname := context.hostname():
        prompt += f"@{hostname}"

    prompt += "]"

    if (shell_level := context.shell_level()) > 1:
        # TODO: handle current theme config pathing more elegantly.
        mark = config["themes"]["default"]["shell_level"]["mark"]
        prompt += f"{mark}{shell_level}"

    prompt += " > "

    return prompt
