"""Advent of Code Day9 problem.

Usage:
    day9.py [--example]

Options:
    --example   Use example input rather than running personal input.
"""

from __future__ import annotations

# Standard Library
from dataclasses import dataclass
from itertools import pairwise
import logging
import os
import sys
from textwrap import dedent
from typing import TYPE_CHECKING
from typing import Any

if TYPE_CHECKING:
    from collections.abc import Iterator

# External Party
from aocd import get_data
from aocd import submit
from docopt import docopt

try:
    # My Modules
    from common.template import Day
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.template import Day

LOG_NAME = "day9"
LOG = logging.getLogger(LOG_NAME)
LOG.addHandler(logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.txt", "w"))

EXAMPLE = dedent(
    """\
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2"""
)
EXAMPLE2 = dedent(
    """\
    R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20"""
)


@dataclass
class RopePoint:
    """A knot point in 2D space."""

    x_pos: int = 0
    y_pos: int = 0

    def __add__(self, other: Any) -> RopePoint:
        """Add two RopePoints together."""
        if not isinstance(other, self.__class__):
            return NotImplemented
        return self.__class__(self.x_pos + other.x_pos, self.y_pos + other.y_pos)

    def __sub__(self, other: Any) -> tuple[int, int]:
        """Return difference in x,y."""
        if not isinstance(other, self.__class__):
            return NotImplemented
        return self.x_pos - other.x_pos, self.y_pos - other.y_pos

    def __hash__(self) -> int:
        """Hash RopePoint as a tuple of x,y."""
        return hash((self.x_pos, self.y_pos))

    def __iter__(self) -> Iterator[int]:
        """Iterate over x,y."""
        yield self.x_pos
        yield self.y_pos

    def touching(self, other: Any) -> bool:
        """Return if two points are touching.

        Overlapping does count as touching.
        """
        if not isinstance(other, self.__class__):
            return NotImplemented
        return abs(self.x_pos - other.x_pos) <= 1 and abs(self.y_pos - other.y_pos) <= 1

    def move(self, direction: str) -> RopePoint:
        """Perform move command in given direction for one square.

        Mapping for directions is:
          R -> right
          L -> left
          U -> up
          D -> down

        Args:
            direction (str): direction key

        Returns:
            Self: self after having done the move
        """
        match direction:
            case "R":
                self.x_pos += 1
            case "L":
                self.x_pos -= 1
            case "D":
                self.y_pos -= 1
            case "U":
                self.y_pos += 1
            case _:
                raise ValueError("Direction")

        return self

    def follow(self, other: RopePoint) -> None:
        """Move self point in space to follow other.

        Will move when self is not touching other.

        Args:
            other (RopePoint): Point to follow
        """
        if self.touching(other):
            return
        vector_change = other - self
        LOG.info("performing a move based off vector %s", vector_change)
        match vector_change:
            case 0, 2:
                self.move("U")
            case 0, -2:
                self.move("D")
            case 2, 0:
                self.move("R")
            case -2, 0:
                self.move("L")
            case (x, y):
                self.move("R" if x > 0 else "L")
                self.move("U" if y > 0 else "D")


class Day9(Day):
    """Day 9 of Advent of Code 2022."""

    def parse(self, data_input: str) -> list[list[str]]:
        """Given input split on newlines and tokenize it."""
        return [line.split(" ") for line in data_input.splitlines()]

    @staticmethod
    def _sim_knot_moves(
        commands: list[list[str]], num_knots: int = 2
    ) -> set[tuple[int, int]]:
        knots = [RopePoint() for _ in range(num_knots)]
        tail_point_set = set()
        for direction, moves in commands:
            LOG.info("%s for %s", direction, moves)
            for _ in range(int(moves)):
                knots[0].move(direction=direction)
                for prev_knot, curr_knot in pairwise(knots):
                    curr_knot.follow(prev_knot)
                tail_point_set.add(tuple(knots[-1]))
        LOG.info("%s", tail_point_set)
        return tail_point_set  # type: ignore[return-value]

    def part1(self, data: list[list[str]]) -> int:
        """Return number of unique places the tail visited.

        A rope has a head and tail. Given instructions on how the head will move
        model what places in a 2D grid the tail will go to.
        Assume starting position where Head is on Tail.

        Args:
            data (List[List[str]]): List of tokenized instructions

        Returns:
            int
        """
        return len(self._sim_knot_moves(data))

    def part2(self, data: list[list[str]]) -> int:
        """Return number of unique positions visited by tail.

        This rope is now a 10 knot rope - one head with 9 following 'tails'.
        We only want to track the final knot - which will be dubbed tail.

        Args:
            data (List[List[str]]): Commands for head to execute

        Returns:
            int
        """
        if args["--example"]:
            data = self.parse(EXAMPLE2)

        return len(self._sim_knot_moves(data, num_knots=10))


if __name__ == "__main__":
    DAY, YEAR = 9, 2022
    day = eval(f"Day{DAY}()")
    global args
    args = docopt(__doc__)
    LOG.setLevel(logging.CRITICAL if not args["--example"] else logging.DEBUG)
    data = get_data(day=DAY, year=YEAR) if not args["--example"] else EXAMPLE
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        sys.exit(0)
    for ans, part in zip(answers, ["a", "b"], strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
