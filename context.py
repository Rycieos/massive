from datetime import datetime, timedelta
from typing import Any, Callable, Dict

from cache import CacheInstance
import data


class Context:
    def __init__(
        self, config: Dict, envs: Dict, global_cache: Dict, client_cache: Dict
    ):
        self.config = config
        self.environment_vars = envs
        self.global_cache = global_cache
        self.client_cache = client_cache
        self.inst_cache: Dict[str, Any] = dict()

    def with_inst_cache(
        self,
        func: Callable[[Dict, Dict], Any],
    ) -> str:
        key = str(func)
        if key in self.inst_cache:
            return self.inst_cache[key]

        value: str = func(self.config, self.environment_vars)

        self.inst_cache[key] = value

        return value

    def with_cache(
        self,
        cache: Dict[str, CacheInstance],
        func: Callable[[Dict, Dict], Any],
        timeout: timedelta,
    ) -> str:
        key = str(func)
        if key in cache and not cache[key].is_expired():
            return cache[key].value

        value: str = func(self.config, self.environment_vars)

        cache[key] = CacheInstance(value, datetime.now() + timeout)

        return value

    def hostname(self) -> str:
        return self.with_cache(self.global_cache, data.hostname, timedelta(minutes=5))

    def username(self) -> str:
        return self.with_cache(self.client_cache, data.username, timedelta(minutes=10))

    def shell_level(self) -> int:
        return self.with_inst_cache(data.shell_level)
