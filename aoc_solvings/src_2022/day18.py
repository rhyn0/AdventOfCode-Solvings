"""Advent of Code Day18 problem.

Usage:
    day18.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

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
from collections import deque
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent
from typing import TYPE_CHECKING
from typing import TypeAlias

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

LOG_NAME = "day18"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5"""
)

LavaPoint: TypeAlias = tuple[int, int, int]


class Day18(Day):
    """Day 18 of Advent of Code 2022."""

    def parse(self, puzzle_input: str) -> set[LavaPoint]:
        """Return set of 3d points of lava."""
        # there are 3 points in each line
        return {tuple(map(int, line.split(","))) for line in puzzle_input.splitlines()}  # type: ignore[misc]

    @staticmethod
    def _gen_face_neighbors(point: LavaPoint) -> Iterator[LavaPoint]:
        for index in range(3):
            for delta in range(-1, 2):
                point_list = list(point)
                point_list[index] += delta
                yield tuple(point_list)  # type: ignore[misc]

    def gen_valid_faces(
        self, point: LavaPoint, min_range: int, max_range: int
    ) -> Iterator[LavaPoint]:
        """Return valid neighbors points in space."""
        for neighbor in self._gen_face_neighbors(point):
            if all(min_range <= coord_val <= max_range for coord_val in neighbor):
                yield neighbor

    def part1(self, data: set[LavaPoint]) -> int:
        """Return number of exposed faces on lava."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)

        return sum(
            1
            for lava in data
            for neighbor in (self._gen_face_neighbors(lava))
            if neighbor not in data
        )

    def part2(self, data: set[LavaPoint]) -> int:
        """Return number of external surface area squares of lava.

        External surface spots are ones that water could splash onto.
        This is because the lava could be creating a hollow cube.

        Args:
            data (set[tuple[int, int, int]]): Lava points

        Returns:
            int
        """
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)

        min_range = min(min(pt) for pt in data) - 1
        max_range = max(max(pt) for pt in data) + 2

        # start at a guaranteed point outside the ball of lava
        # Then map surface area by hitting all surface lava points
        queue = deque([(min_range, min_range, min_range)])
        visited = set()
        vis_faces = 0
        while queue:
            curr_pt = queue.popleft()
            if curr_pt in visited:
                continue
            visited.add(curr_pt)
            for neighbor in self.gen_valid_faces(curr_pt, min_range, max_range):
                if neighbor in data:
                    # known lava point, hit from outside surface
                    vis_faces += 1
                else:
                    queue.append(neighbor)

        return vis_faces


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 18, 2022
    day = Day18()
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
        assert answers == (64, 58)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
