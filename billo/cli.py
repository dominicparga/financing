"""
(c) 2023 - 2023 Parga Cacheiro, Dominic
"""

import argparse

from billo._avg import cli as avg_cli


def _setup_parser():
    parser = argparse.ArgumentParser(description="Billo at your service.", prog="billo")

    subparsers = parser.add_subparsers(title="services")

    run_parser = subparsers.add_parser("run", help="Config based runner")
    avg_cli.setup_parser(parser=run_parser)

    return parser


def main(args: list[str]):
    parser = _setup_parser()
    parsed_args = parser.parse_args(args=args)

    if "run" in parsed_args:
        parsed_args.run(parsed_args=parsed_args)
    else:
        main(args=["-h"])
