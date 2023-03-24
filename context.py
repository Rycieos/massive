from datetime import datetime, timedelta
from typing import Any, Callable, Dict, List, Optional

from cache import CacheInstance
import data


class Context:
    def __init__(
        self,
        config: Dict,
        global_cache: Optional[Dict],
        client_cache: Optional[Dict],
        shell: str,
        width: int,
        exit_status: int,
        pipe_status: List[int],
        jobs_running: int,
        jobs_sleeping: int,
        current_dir: str,
        envs: Dict[str, str],
    ):
        self.config = config
        self.inst_cache: Dict[str, Any] = dict()
        self.timed_caches = bool(global_cache and client_cache)
        self.global_cache = global_cache or dict()
        self.client_cache = client_cache or dict()

        self.shell = shell
        self.width = width
        self.exit_status = exit_status
        self.pipe_status = pipe_status
        self.jobs_running = jobs_running
        self.jobs_sleeping = jobs_sleeping
        self.current_dir = current_dir
        self.environment_vars = envs

    def with_inst_cache(
        self,
        func: Callable[[Any, Dict], Any],
    ) -> Any:
        key = str(func)
        if key in self.inst_cache:
            return self.inst_cache[key]

        value: str = func(self, self.environment_vars)

        self.inst_cache[key] = value

        return value

    def with_cache(
        self,
        cache: Dict[str, CacheInstance],
        func: Callable[[Any, Dict], Any],
        timeout: timedelta,
    ) -> Any:
        if not self.timed_caches:
            return self.with_inst_cache(func)

        key = str(func)
        if key in cache and not cache[key].is_expired():
            return cache[key].value

        value: str = func(self, self.environment_vars)

        cache[key] = CacheInstance(value, datetime.now() + timeout)

        return value

    def hostname(self) -> str:
        return self.with_cache(self.global_cache, data.hostname, timedelta(minutes=5))

    def username(self) -> str:
        return self.with_cache(self.client_cache, data.username, timedelta(minutes=10))

    def work_dir(self) -> str:
        return self.with_inst_cache(data.work_dir)

    def shell_level(self) -> int:
        return self.with_inst_cache(data.shell_level)
