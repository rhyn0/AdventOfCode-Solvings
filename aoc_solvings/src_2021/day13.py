"""Advent of Code 2021 Day13 problem.

Usage:
    day13.py [--example | --local] [--verbose]

Options:
    --example   Use example input rather than running personal input.
    --local     Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose   Use python logging to get verbose output of what is going on
                in a log file.
"""
from __future__ import annotations

# Standard Library
from dataclasses import dataclass
import logging
from pathlib import Path
import sys
from textwrap import dedent
from typing import Any

# External Party
from aocd import get_data
from aocd import submit
from docopt import docopt

# My Modules
from aoc_solvings.common.template import Day

LOG_NAME = "day13"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5"""
)


@dataclass(unsafe_hash=True)
class GridLoc:
    """Hold data for location on a gridlike structure."""

    x_pos: int
    y_pos: int

    def __add__(self, other: Any) -> GridLoc:
        """Handle adding two locations together."""
        if not isinstance(other, GridLoc):
            return NotImplemented
        return GridLoc(self.x_pos + other.x_pos, self.y_pos + other.y_pos)

    def __sub__(self, other: Any) -> GridLoc:
        """Handle subtracting other location from self."""
        if not isinstance(other, GridLoc):
            return NotImplemented
        return GridLoc(self.x_pos - other.x_pos, self.y_pos - other.y_pos)

    def __mul__(self, scalar: int) -> GridLoc:
        """Multiply locations by a scalar."""
        return GridLoc(self.x_pos * scalar, self.y_pos * scalar)

    def __rmul__(self, scalar: int) -> GridLoc:
        """Act like other multiplication."""
        return self * scalar

    def x_greater(self, other: GridLoc) -> bool:
        """Return if self.x is greater than other.x.

        Strictly greater than

        Args:
            other (GridLoc): other position to compare against

        Returns:
            bool: True if self has greater x value than other, False otherwise.
        """
        return self.x_pos > other.x_pos

    def y_greater(self, other: GridLoc) -> bool:
        """Return if self.y is greater than other.y.

        Strictly greater than

        Args:
            other (GridLoc): other position to compare against

        Returns:
            bool: True if self has greater y value than other, False otherwise.
        """
        return self.y_pos > other.y_pos


class Fold(GridLoc):
    """Derivative of a point is a straight line along an axis."""

    def __init__(self, line: str) -> None:
        """Initialize a fold with a -1 to simulate a line in the other axis."""
        super().__init__(-1, -1)
        if "x" in line:
            self.x_pos = int(line.split("=")[-1])
        else:
            self.y_pos = int(line.split("=")[-1])

    def compare_loc(self, loc: GridLoc) -> bool:
        """Return if supplied location is affected by fold.

        Compares based on whether fold is a vertical or horizontal.

        Args:
            loc (GridLoc): grid location

        Returns:
            bool: True if location is affected by fold, False otherwise.
        """
        LOG.debug(
            "Fold %r is going to compare %s attribute against %r",
            self,
            "x" if self.y_pos == -1 else "y",
            loc,
        )
        return loc.x_greater(self) if self.y_pos == -1 else loc.y_greater(self)

    def fold_loc(self, loc: GridLoc) -> GridLoc:
        """Return the changed location after executing fold across self's line.

        Args:
            loc (GridLoc): location to move

        Returns:
            GridLoc: New location
        """
        ret_loc = loc - 2 * (
            GridLoc(0, loc.y_pos - self.y_pos)
            if self.x_pos == -1
            else GridLoc(loc.x_pos - self.x_pos, 0)
        )
        LOG.debug("Fold %r is folding the location %r to %r", self, loc, ret_loc)
        return ret_loc


class Day13(Day):
    """Day 13 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> tuple[set[GridLoc], list[Fold]]:
        """Return tuple of dots and fold locations."""
        dots, folds = (section.splitlines() for section in puzzle_input.split("\n\n"))
        locs = {GridLoc(*(int(part) for part in dot.split(","))) for dot in dots}
        fold_list = [Fold(fold) for fold in folds]
        return locs, fold_list

    @staticmethod
    def _compute_fold(dots: set[GridLoc], curr_fold: Fold) -> set[GridLoc]:
        LOG.debug("Given dot set of %s and fold %r", dots, curr_fold)
        return {
            curr_fold.fold_loc(dot) if curr_fold.compare_loc(dot) else dot
            for dot in dots
        }

    def part1(self, data: tuple[set[GridLoc], list[Fold]]) -> int:
        """Return number of dots visible after doing one fold."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        new_dots = self._compute_fold(data[0], data[1][0])
        LOG.debug(
            "Got new dots of %s",
            sorted(new_dots, key=lambda obj: (obj.y_pos, obj.x_pos)),
        )
        return len(new_dots)

    @staticmethod
    def _dump(dots: set[GridLoc]) -> None:
        x_max = y_max = 0
        for dot in dots:
            x_max = max(x_max, dot.x_pos)
            y_max = max(y_max, dot.y_pos)

        for curr_y in range(y_max + 1):
            for curr_x in range(x_max + 1):
                if GridLoc(curr_x, curr_y) in dots:
                    print("#", end="")
                else:
                    print(".", end="")
            print()

    def part2(self, data: tuple[set[GridLoc], list[Fold]]) -> None:
        """Print code onto stdout for human to read."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        dots, folds = data
        for fold in folds:
            dots = self._compute_fold(dots, fold)

        self._dump(dots)


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 13, 2021
    day = Day13()

    if args["--example"] or args["--verbose"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.log", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)

    if args["--example"]:
        data = EXAMPLE
    elif args["--local"]:
        data = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        sys.exit(0)
    # can't auto submit 'b' since its based on the text layout from folding
    for ans, part in zip(answers, ["a"], strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
