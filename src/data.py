from dataclasses import dataclass
from enum import Enum
from typing import Optional


class Vcs(Enum):
    git = 1
    hg  = 2
    svn = 3

class Data:

    @dataclass
    class Hostname:
        display: str
        full: str
        short: str

    def hostname() -> Optional[Hostname]:
        pass


    def username() -> str:
        pass


    def vcs_active(vcs: Optional[Vcs] = None) -> bool:
        pass


    def vcs_branch(vcs: Optional[Vcs] = None) -> Optional[str]:
        pass
