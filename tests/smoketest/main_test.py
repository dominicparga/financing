"""
(c) 2023 - 2023 Parga Cacheiro, Dominic
"""

import os

import billo.cli

_BILLO_JSON = os.path.join(os.path.dirname(__file__), "data", "in", "billo.json")


def test():
    billo.cli.main(args=["run", "--config", _BILLO_JSON])
