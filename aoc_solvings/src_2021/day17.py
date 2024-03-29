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


def does_hit_target(
    x_vel: int, y_vel: int, target: tuple[tuple[int, int], tuple[int, int]]
) -> bool:
    """Return if the packet hits the target."""
    x_bounds, y_bounds = target
    x_start, x_stop = x_bounds
    y_start, y_stop = y_bounds
    x_pos = y_pos = 0
    # manually loop through and check if this velocity hits the target
    while True:
        # break conditions
        if x_pos > x_stop:
            return False
        if x_vel == 0 and not x_start <= x_pos <= x_stop:
            return False
        if x_vel == 0 and y_pos < y_start:
            return False

        # we hit
        if x_start <= x_pos <= x_stop and y_start <= y_pos <= y_stop:
            return True

        # update position
        x_pos += x_vel
        y_pos += y_vel

        # update velocities
        if x_vel > 0:
            x_vel -= 1
        y_vel -= 1


class Day17(Day):
    """Day 17 of Advent of Code 2021."""

    example = dedent(
        """\
        target area: x=20..30, y=-10..-5"""
    )
    day = 17
    year = 2021

    def parse(self, puzzle_input: str) -> tuple[tuple[int, int], tuple[int, int]]:
        """Return bin string with no extra spaces."""
        _, data = puzzle_input.split(":")
        x, y = data.strip().split(",")
        x_start, x_stop = map(int, x.split("=")[1].split(".."))
        y_start, y_stop = map(int, y.split("=")[1].split(".."))
        # boundaries are inclusive
        return ((x_start, x_stop), (y_start, y_stop))

    def part1(self, data: tuple[tuple[int, int], tuple[int, int]]) -> int:
        """Return sum of the packet versions for all packets in data."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        _, y_bounds = data
        # We go up, then come down. The maximum speed we can have at y=0 on way down
        # is the absolute value of minimum boundary y - 1 (in negative direction)
        # this causes the step from y=0 to the bottom edge of area
        # which maximized our height
        y_min, _ = y_bounds
        init_y_vel = abs(y_min) - 1
        return init_y_vel * (init_y_vel + 1) // 2

    def part2(self, data: tuple[tuple[int, int], tuple[int, int]]) -> int:
        """Return count of max element minus count of min element after insertions."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        x_bounds, y_bounds = data
        max_y = max(map(abs, y_bounds))
        return sum(
            does_hit_target(x, y, data)
            for x in range(x_bounds[1] + 1)
            for y in range(-max_y, max_y + 1)
        )


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    problem = Day17()

    edit_logger_for_verbosity(
        LOG, args["--verbose"] or args["--example"], args["--quiet"]
    )
    data = get_data_args(args, problem)
    answers = problem.solve(data, parts=args["--parts"])
    print(answers)
    if args["--example"]:
        sys.exit(0)
    submit_answers(answers, args["--parts"], day=problem.day, year=problem.year)
