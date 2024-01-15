"""Advent of Code Day12 problem.

Usage:
    day12.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

Options:
    --example       Use example input rather than running personal input.
    --local         Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose       Use python logging to get verbose output of what is going on
                    in a log file.
    --quiet         Disable logging for example mode.
    --parts PART    Do only specified part, options are 'a', 'b', or 'ab'. [default: ab]
"""
# Standard Library
from collections import deque
from collections.abc import Iterator
import contextlib
from itertools import pairwise
import logging
import os
from pathlib import Path
from string import ascii_lowercase as lwrcase
import sys
from textwrap import dedent

# External Party
from aocd import get_data
from aocd import submit
from docopt import docopt

try:
    # My Modules
    from common.template import AnswerNotFoundError
    from common.template import Day
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.template import AnswerNotFoundError
    from common.template import Day

LOG_NAME = "day12"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi"""
)

TwoDimPoint = tuple[int, int]


class Day12(Day):
    """Day 12 of Advent of Code 2022."""

    def parse(self, data: str) -> tuple[list[list[int]], TwoDimPoint, TwoDimPoint]:
        """Parse input grid and find start and end points.

        Args:
            data (str): text block represeting elevation grid

        Returns:
            Tuple[List[List[int]], TwoDimPoint, TwoDimPoint]
        """
        grid = []
        start_loc = end_loc = -1, -1
        for row, line in enumerate(data.splitlines()):
            a_line = []
            for col, ch in enumerate(line):
                if (value := lwrcase.find(ch)) != -1:
                    a_line.append(value)
                elif ch == "S":
                    a_line.append(0)
                    start_loc = row, col
                else:
                    a_line.append(25)
                    end_loc = row, col
            grid.append(a_line)
        return grid, start_loc, end_loc

    _DIRECTS = (0, 1, 0, -1, 0)

    @staticmethod
    def _get_neighbors(grid: list[list[int]], pos: TwoDimPoint) -> list[TwoDimPoint]:
        n, m = len(grid), len(grid[0])
        neighs = []
        for drow, dcol in pairwise(Day12._DIRECTS):
            new_row, new_col = pos[0] + drow, pos[1] + dcol
            if (
                0 <= new_row < n
                and 0 <= new_col < m
                and grid[new_row][new_col] <= grid[pos[0]][pos[1]] + 1
            ):
                neighs.append((new_row, new_col))
        return neighs

    def new_neighbors(
        self,
        steps: int,
        visited_paths: dict[TwoDimPoint, int],
        grid: list[list[int]],
        loc: TwoDimPoint,
    ) -> Iterator[TwoDimPoint]:
        """Generate neighbors that match valid conditions.

        Only generate neighbors that have not been visited yet or ones that we
        can reach faster.
        """
        yield from (
            neighbor
            for neighbor in self._get_neighbors(grid, loc)
            if neighbor not in visited_paths or steps < visited_paths[neighbor]
        )

    def _bfs(
        self, data_grid: list[list[int]], start: TwoDimPoint, end: TwoDimPoint
    ) -> int:
        que: deque[tuple[TwoDimPoint, int]] = deque([(start, 0)])
        short_path_len: float | int = float("inf")
        visited: dict[TwoDimPoint, int] = {}
        while que:
            LOG.debug("queue after visiting %d spots - %s", len(visited), que)
            curr_loc, steps = que.popleft()
            new_step = steps + 1
            if curr_loc == end:
                short_path_len = min(short_path_len, steps)
            for neighbor in self.new_neighbors(new_step, visited, data_grid, curr_loc):
                LOG.debug("location %s generated neighbor %s", curr_loc, neighbor)
                visited[neighbor] = new_step
                que.append((neighbor, new_step))

        # if this is float, no int len path was found
        if isinstance(short_path_len, float):
            raise AnswerNotFoundError()
        return short_path_len

    def part1(self, data: tuple[list[list[int]], TwoDimPoint, TwoDimPoint]) -> int:
        """Find shortest path from start point to end point.

        See BFS implementation.

        Args:
            data (Tuple[List[List[int]], TwoDimPoint, TwoDimPoint]): Tuple of grid,
                start, end point

        Returns:
            int: path length
        """
        return self._bfs(*data)

    def part2(self, data: tuple[list[list[int]], TwoDimPoint, TwoDimPoint]) -> int:
        """Find shortest path length from end to a starting point of elevation 0 (a).

        Use BFS on all possible start points.
        Could build BFS backwards - from end to any a point too.

        Args:
            data (Tuple[List[List[int]], TwoDimPoint, TwoDimPoint]): Tuple of grid,
                start, end point

        Raises:
            ValueError: If can't find a valid path from E to lowest elevation point.

        Returns:
            int
        """
        grid, _, end_loc = data
        scenic_hike_dist: int | float = float("inf")
        for row, line in enumerate(grid):
            for col, val in enumerate(line):
                if val == 0:
                    with contextlib.suppress(ValueError):
                        scenic_hike_dist = min(
                            scenic_hike_dist, self._bfs(grid, (row, col), end_loc)
                        )
        if isinstance(scenic_hike_dist, float):
            raise AnswerNotFoundError()
        return scenic_hike_dist


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 12, 2022
    day = Day12()
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
        data = EXAMPLE
    elif args["--local"]:
        data = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data, parts=args["--parts"][0])
    print(answers)
    if args["--example"]:
        assert answers == (31, 29)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
