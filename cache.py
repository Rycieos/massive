from datetime import datetime


class CacheInstance:
    def __init__(self, value: str, ttl: datetime):
        self.value = value
        self.ttl = ttl

    def is_expired(self) -> bool:
        return self.ttl < datetime.now()
