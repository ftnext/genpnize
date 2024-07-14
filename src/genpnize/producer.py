from collections.abc import Generator
from typing import Protocol

from genpnize.iter_recipes import grouper


class Producer(Protocol):
    def write_lines(self, text: str) -> Generator[str, None, None]: ...


class GenP:
    def write_lines(self, text: str) -> Generator[str, None, None]:
        for chunk in grouper(text.replace("\n", ""), 13, fillvalue=""):
            yield "".join(chunk)
