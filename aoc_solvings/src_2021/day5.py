"""Advent of Code Day5 problem.

Usage:
    day5.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

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
from collections import Counter
from itertools import chain
import logging
from pathlib import Path
import sys
from textwrap import dedent
from typing import TYPE_CHECKING
from typing import Any

if TYPE_CHECKING:
    from collections.abc import Iterable
    from collections.abc import Iterator


# External Party
from aocd import get_data
from aocd import submit
from docopt import docopt

# My Modules
from aoc_solvings.common.template import Day

LOG_NAME = "day5"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2"""
)


class Point:
    """Class to denote point in 2D cartesian plane."""

    def __init__(self, x_pos: str | int, y_pos: str | int) -> None:
        """Set instance parameters."""
        self.x = int(x_pos.strip()) if isinstance(x_pos, str) else x_pos
        self.y = int(y_pos.strip()) if isinstance(y_pos, str) else y_pos

    def __eq__(self, other: Any) -> bool:
        """Only compare against other Points."""
        if not isinstance(other, Point):
            return NotImplemented
        return self.x == other.x and self.y == other.y

    def __repr__(self) -> str:
        """Override repr to be prettier."""
        return f"P({self.x},{self.y})"

    def __sub__(self, other: Any) -> tuple[int, int]:
        """Allow subtraction against other Point."""
        if not isinstance(other, Point):
            return NotImplemented
        return (self.x - other.x, self.y - other.y)

    def __hash__(self) -> int:
        """Make hashable using tuple of coordinates."""
        return hash((self.x, self.y))


class Line:
    """Representation of Line in 2D Cartesian Plane using Points."""

    def __init__(self, e1: Point, e2: Point) -> None:
        """Instantiate endpoints with Point."""
        self.ends = e1, e2
        if e1 == e2:
            print("singular point line...")

    def __repr__(self) -> str:
        """Override repr to also use Point repr."""
        return f"Line({self.ends[0]} -> {self.ends[1]})"

    def __str__(self) -> str:
        """Make sure str matches repr."""
        return repr(self)

    def __len__(self) -> int:
        """Length of Line is Euclidean distance."""
        dist = self.ends[0] - self.ends[1]
        return int((dist[0] ** 2 + dist[1] ** 2) ** 0.5)

    def is_horizontal(self) -> bool:
        """Return if Line has no change in y value."""
        return (self.ends[0] - self.ends[1])[1] == 0

    def is_vertical(self) -> bool:
        """Return if Line has no change in x value."""
        return (self.ends[0] - self.ends[1])[0] == 0

    def get_points(self) -> Iterator[Point]:
        """Return a generator for all Points along Line."""
        length = self.ends[0] - self.ends[1]
        sta, sto = 0, -1
        match length:
            case (x, 0):
                sta, sto = (0, x + 1) if x > 0 else (x, 1)
                for i in range(sta, sto):
                    yield Point(self.ends[1].x + i, self.ends[1].y)
            case (0, y):
                sta, sto = (0, y + 1) if y > 0 else (y, 1)  # type: ignore[unreachable]
                for i in range(sta, sto):
                    yield Point(self.ends[1].x, self.ends[1].y + i)
            case (x, y):
                sta, sto = (0, x + 1) if x > 0 else (x, 1)  # type: ignore[unreachable]
                for i in range(sta, sto):
                    yield Point(self.ends[1].x + i, self.ends[1].y + (y // x) * i)

    def intersect(self, other: Line) -> Iterator[Point]:
        """Find intersection points between this and other Line."""
        for val_self in self.get_points():
            for val_o in other.get_points():
                if val_o == val_self:
                    yield val_self


class Day5(Day):
    """Day 5 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> list[Line]:
        """Given input build all Lines.

        Input looks like:
            x1, y1 -> x2, y2
        """
        lines = []
        for line in puzzle_input.split("\n"):
            left, right = (item.strip().split(",") for item in line.split("->"))
            lines.append(
                Line(
                    Point(*[int(val) for val in left]),
                    Point(*[int(val) for val in right]),
                )
            )
        return lines

    @staticmethod
    def count_intersections(points: Iterable[Point]) -> int:
        """Return the number of intersection points given lines."""
        thermals_count = Counter(points)
        return len([item for item in thermals_count.items() if item[1] > 1])

    def part1(self, data: list[Line]):
        """Return number of points where there is intersecting lines."""
        data = list(
            filter(lambda line: line.is_horizontal() or line.is_vertical(), data)
        )
        return self.count_intersections(chain(*[line.get_points() for line in data]))

    def part2(self, data: list[Line]):
        """Return total number of intersections."""
        return self.count_intersections(chain(*[line.get_points() for line in data]))


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 5, 2021
    day = Day5()
    if args["--example"] or args["--verbose"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.log", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)
    if args["--quiet"]:
        logging.disable(logging.CRITICAL)

    if args["--example"]:
        grid = EXAMPLE
    elif args["--local"]:
        grid = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        grid = get_data(day=DAY, year=YEAR)
    answers = day.solve(grid, parts=args["--parts"][0])
    print(answers)
    if args["--example"]:
        assert answers == (5, 12)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
