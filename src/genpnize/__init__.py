import argparse

from more_itertools import chunked

__version__ = "0.0.1"


def main():
    args = parse_args()

    input_text = args.input.read()
    for chunk in chunked(input_text.replace("\n", ""), 13):
        print("".join(chunk))


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=argparse.FileType("r"))
    return parser.parse_args()
