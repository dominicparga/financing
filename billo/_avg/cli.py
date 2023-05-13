"""
(c) 2023 - 2023 Parga Cacheiro, Dominic
"""

import argparse

import billo._avg.core as avg


def _run(parsed_args: list[str]):
    avg.foo(parsed_args)


def setup_parser(parser: argparse.ArgumentParser):
    help_msg = "Path to billo.json"
    parser.add_argument("--config", metavar="PATH", type=str, help=help_msg)
    parser.set_defaults(run=_run)
