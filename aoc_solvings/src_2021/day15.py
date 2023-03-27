"""Advent of Code 2021 Day15 problem.

Usage:
    day15.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

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
from bisect import bisect_left
from dataclasses import dataclass
from dataclasses import field
from itertools import pairwise
from itertools import repeat
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent
from typing import TYPE_CHECKING
from typing import TypeAlias

if TYPE_CHECKING:
    from collections.abc import Callable
    from collections.abc import Iterable
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

LOG_NAME = "day15"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581"""
)


@dataclass(slots=True, unsafe_hash=True)
class GridLoc:
    """Location on the given grid scan."""

    x_pos: int
    y_pos: int

    _neigh_direction: tuple[int, int, int, int, int] = field(
        default=(0, 1, 0, -1, 0), repr=False
    )

    def __len__(self) -> int:
        """Return number of attributes on this type of object."""
        return 2

    def __add__(self, other: object) -> GridLoc:
        """Create new location based on sum of parts."""
        if not isinstance(other, GridLoc | tuple) or len(other) != len(
            GridLoc.__dataclass_fields__
        ):
            return NotImplemented
        if isinstance(other, tuple):
            return GridLoc(self.x_pos + other[0], self.y_pos + other[1])
        return GridLoc(self.x_pos + other.x_pos, self.y_pos + other.y_pos)

    def __eq__(self, other: object) -> bool:
        """Comparison of location on grid for equality."""
        if not isinstance(other, GridLoc):
            return NotImplemented
        return self.x_pos == other.x_pos and self.y_pos == other.y_pos

    def cardinal_neighbors(self) -> Iterable[GridLoc]:
        """Generate the neighbors to this location."""
        yield from [
            (*self, delta_x, delta_y)
            for delta_x, delta_y in pairwise(self._neigh_direction)
        ]


# Don't need to track the actual path since we only care about
# summing the risk of that path

AStarLoc: TypeAlias = tuple[GridLoc, int]  # location and risk up to that point


@dataclass(slots=True)
class AStarQueue:
    """Object to handle logic of doing A* path finding."""

    grid_map: list[list[int]] | list[list[str]]

    grid_max_row: int = field(init=False)
    grid_max_col: int = field(init=False)

    def __post_init__(self) -> None:
        """Store size of map for later use."""
        self.grid_max_row = len(self.grid_map)
        self.grid_max_col = len(self.grid_map[0])

    def in_bounds(self, loc: GridLoc) -> bool:
        """Return if given location is in bounds of the map."""
        return 0 <= loc.x_pos < self.grid_max_col and 0 <= loc.y_pos < self.grid_max_row

    def neighbors_in_bounds(self, loc: GridLoc) -> Iterator[GridLoc]:
        """Iterate over cardinal neighbors that are in bounds to the map.

        Simplify the logic loop of getting cardinal neighbors but make sure
        they are in bounds

        Args:
            loc (GridLoc): Location on Grid to generate neighbors for

        Yields:
            Iterator[GridLoc]: Yields neighbors that are in bounds
        """
        yield from (
            neighbor
            for neighbor in loc.cardinal_neighbors()
            if self.in_bounds(neighbor)
        )

    def find_path(
        self,
        start: GridLoc,
        goal: GridLoc,
        /,
        key_func: Callable[[AStarLoc | int], int],
    ) -> int:
        """Return minimum risk of path between start and goal.

        Args:
            start (GridLoc): Starting position
            goal (GridLoc): end position
            key_func (Callable[[AStarLoc | int], int]): function to determine
                weight of possible paths

        Returns:
            int
        """
        LOG.info(
            "Initializing A* search. Start is %r going to %r with map of size %dx%d",
            start,
            goal,
            self.grid_max_row,
            self.grid_max_col,
        )
        unvisited: list[AStarLoc] = [(start, 0)]
        visited: set[GridLoc] = set()
        LOG.info("Initialized open list %r and closed set %r", unvisited, visited)
        while unvisited:
            curr_loc, curr_risk = unvisited.pop()
            if curr_loc == goal:
                return curr_risk
            if curr_loc in visited:
                continue
            LOG.debug(
                "Popped location %r of risk %d. Open list is now %r",
                curr_loc,
                curr_risk,
                unvisited,
            )
            for neighbor in self.neighbors_in_bounds(curr_loc):
                new_risk = curr_risk + int(
                    self.grid_map[neighbor.y_pos][neighbor.x_pos]
                )
                unvisited.insert(
                    bisect_left(unvisited, key_func(new_risk), key=key_func),
                    (neighbor, new_risk),
                )
            visited.add(curr_loc)

        # error out
        return -1


class Day15(Day):
    """Day 15 of Advent of Code 2021."""

    MAX_OCTO_RISK = 9

    def parse(self, puzzle_input: str) -> list[list[int]]:
        """Return tuple of starting template and polymer rules."""
        return [[int(char) for char in line] for line in puzzle_input.splitlines()]

    @staticmethod
    def _minimize_risk(obj: AStarLoc | int) -> int:
        if isinstance(obj, int):
            return -obj
        return -obj[1]

    @classmethod
    def _increase_risk(cls, orig: int, tiles_away: int) -> int:
        ret_val = sum(divmod(orig + tiles_away, 10))
        if ret_val > cls.MAX_OCTO_RISK:
            return ret_val % cls.MAX_OCTO_RISK
        return ret_val

    def part1(self, data: list[list[int]]) -> int:
        """Return count of max element minus count of min element after insertions."""
        LOG.info("-" * 20 + "starting part1" + "-" * 20)
        start, end = GridLoc(0, 0), GridLoc(len(data[0]) - 1, len(data) - 1)

        asq = AStarQueue(data)
        return asq.find_path(start, end, key_func=self._minimize_risk)

    def part2(self, data: list[list[int]]) -> int:
        """Return count of max element minus count of min element after insertions."""
        LOG.info("-" * 20 + "starting part2" + "-" * 20)
        new_grid = [
            [
                self._increase_risk(val, row_tile + col_tile)
                for col_tile, col in enumerate(repeat(line, 5))
                for val in col
            ]
            for row_tile, tile in enumerate(repeat(data, 5))
            for line in tile
        ]
        start, end = GridLoc(0, 0), GridLoc(len(new_grid[0]) - 1, len(new_grid) - 1)
        asq = AStarQueue(new_grid)
        return asq.find_path(start, end, key_func=self._minimize_risk)


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 15, 2021
    day = Day15()
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
        assert answers == (40, 315)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
