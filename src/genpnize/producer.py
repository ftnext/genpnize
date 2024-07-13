from collections.abc import Generator
from typing import Protocol

from more_itertools import chunked


class Producer(Protocol):
    def write_lines(self, text: str) -> Generator[str, None, None]: ...


class GenP:
    def write_lines(self, text: str) -> Generator[str, None, None]:
        for chunk in chunked(text.replace("\n", ""), 13):
            yield "".join(chunk)
