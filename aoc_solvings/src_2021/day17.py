"""Advent of Code 2021 Day17 problem.

Usage:
    day17.py [--example [--quiet] | --local] [--verbose] [--parts=<char>]

Options:
    --example       Use example input rather than running personal input.
    --local         Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose       Use python logging to get verbose output of what is going on
                    in a log file.
    --quiet         Disable logging for example mode.
    --parts PART    Do only specified part, options are 'a', 'b', or 'ab'. [default: ab]
"""

from __future__ import annotations

# Standard Library
import os
import sys
from textwrap import dedent

# External Party
from docopt import docopt

try:
    # My Modules
    from common.log import edit_logger_for_verbosity
    from common.log import get_logger
    from common.template import Day
    from common.template import get_data_args
    from common.template import submit_answers
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.log import edit_logger_for_verbosity
    from common.log import get_logger
    from common.template import Day
    from common.template import get_data_args
    from common.template import submit_answers


LOG = get_logger("day17")

EXAMPLE = dedent(
    """\
    9C0141080250320F1802104A08"""
)


class Day17(Day):
    """Day 17 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> str:
        """Return bin string with no extra spaces."""
        return f"{int(puzzle_input.strip(), 16):0>{len(puzzle_input) * 4}b}"

    def part1(self, data: str) -> int:
        """Return sum of the packet versions for all packets in data."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        return 0

    def part2(self, data: str) -> int:
        """Return count of max element minus count of min element after insertions."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        return 0


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 17, 2021
    day = Day17()

    edit_logger_for_verbosity(
        LOG, args["--verbose"] or args["--example"], args["--quiet"]
    )
    data = get_data_args(args, day=DAY, year=YEAR)
    answers = day.solve(data, parts=args["--parts"])
    print(answers)
    if args["--example"]:
        sys.exit(0)
    submit_answers(answers, args["--parts"], day=DAY, year=YEAR)
