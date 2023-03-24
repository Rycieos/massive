from importlib import import_module
import logging
from typing import Callable

logger = logging.getLogger(__name__)


def load_theme(theme: str) -> Callable:
    try:
        plugin = import_module("." + theme, package="plugins")
    except (ModuleNotFoundError, ImportError):
        logger.exception("Could not load plugin '%s'", theme)
        raise

    return plugin.generate_prompt
