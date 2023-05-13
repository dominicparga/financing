"""
(c) 2023 - 2023 Parga Cacheiro, Dominic
"""

import os


def foo(parsed_args: list[str]):
    with open(os.path.dirname(parsed_args[0]), mode="w", encoding="UTF-8") as f:
        f.write("asdf")
