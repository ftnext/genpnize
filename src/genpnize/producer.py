from collections.abc import Generator
from typing import Protocol

from genpnize.iter_recipes import grouper


def chop_text(text: str, n: int) -> Generator[str, None, None]:
    for chunk in grouper(text, n, fillvalue=""):
        yield "".join(chunk)


class Producer(Protocol):
    def write_lines(self, text: str) -> Generator[str, None, None]: ...


class GenP:
    def write_lines(self, text: str) -> Generator[str, None, None]:
        yield from chop_text(text.replace("\n", ""), 13)
