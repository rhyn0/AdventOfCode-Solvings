"""Advent of Code Day23 problem.

Usage:
    day23.py [--example | --local]

Options:
    --example   Use example input rather than running personal input.
    --local     Use problem data stored in local data folder as `inputYEAR-DAY.txt`
"""
from __future__ import annotations

# Standard Library
from collections import defaultdict
from collections import deque
from dataclasses import dataclass
from enum import Enum
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent
from typing import TYPE_CHECKING
from typing import Any
from typing import NamedTuple
from typing import TypeAlias

if TYPE_CHECKING:
    from collections.abc import Callable
    from collections.abc import Iterable

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

LOG_NAME = "day23"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    ....#..
    ..###.#
    #...#.#
    .#...##
    #.###..
    ##.#.##
    .#..#.."""
)


class GridLoc(NamedTuple):
    """Store a position in the grid as (x,y)."""

    x_pos: int
    y_pos: int

    def __add__(self, other: Any) -> GridLoc:
        """Shift location based on addition of another point."""
        if not isinstance(other, GridLoc):
            return NotImplemented
        return GridLoc(self.x_pos + other.x_pos, self.y_pos + other.y_pos)

    def __neg__(self) -> GridLoc:
        """Return negative version of point."""
        return GridLoc(-self.x_pos, -self.y_pos)


class LocEnum(GridLoc, Enum):
    """Enum mixin base class."""


class CompassDirection(LocEnum):
    """Enum of 4 cardinal directions using GridLoc."""

    NORTH = GridLoc(0, -1)
    SOUTH = GridLoc(0, 1)
    EAST = GridLoc(1, 0)
    WEST = GridLoc(-1, 0)
    # alias the directions for simplicity of section after
    N = NORTH
    S = SOUTH
    E = EAST
    W = WEST


CompassSearchArea: TypeAlias = tuple[GridLoc, GridLoc, GridLoc]
SEARCH_NORTH: CompassSearchArea = (
    CompassDirection.N + CompassDirection.W,
    CompassDirection.N.value,
    CompassDirection.N + CompassDirection.E,
)
SEARCH_SOUTH: CompassSearchArea = (
    CompassDirection.S + CompassDirection.W,
    CompassDirection.S.value,
    CompassDirection.S + CompassDirection.E,
)
SEARCH_EAST: CompassSearchArea = (
    CompassDirection.E + CompassDirection.N,
    CompassDirection.E.value,
    CompassDirection.E + CompassDirection.S,
)
SEARCH_WEST: CompassSearchArea = (
    CompassDirection.W + CompassDirection.N,
    CompassDirection.W.value,
    CompassDirection.W + CompassDirection.S,
)

ALL_DIRS = [*SEARCH_NORTH, *SEARCH_SOUTH, CompassDirection.E, CompassDirection.W]


@dataclass
class Elf:
    """Contains position on the grid and handles searching for new spot."""

    loc: GridLoc

    def _is_alone(self, other_elves: set[GridLoc]) -> bool:
        return all(self.loc + direct not in other_elves for direct in ALL_DIRS)

    def search_propose(
        self,
        other_locs: set[GridLoc],
        search_order: list[CompassSearchArea] | deque[CompassSearchArea],
    ) -> GridLoc | None:
        """Return new proposed position for this elf.

        Args:
            other_locs (set[GridLoc]): Set of other Elf positions
            search_order (list[CompassSearchArea]): Order to search space for a new spot

        Returns:
            GridLoc | None: None if no action to be taken this turn,
                else first proposed spot.
        """
        if self._is_alone(other_locs):
            LOG.debug("Elf at %s is alone, so now movements this round.", self.loc)
            return None
        for search_group in search_order:
            if not any(self.loc + adj in other_locs for adj in search_group):
                return self.loc + search_group[1]

        return None

    def _approve_proposal(self, new_loc: GridLoc) -> None:
        self.loc = new_loc


class Day23(Day):
    """Day 23 of Advent of Code 2022."""

    def parse(self, puzzle_input: str) -> list[GridLoc]:
        """Return list of starting locations for elves in shown grid."""
        return [
            GridLoc(col, row)
            for row, line in enumerate(puzzle_input.splitlines())
            for col, char in enumerate(line)
            if char == "#"
        ]

    @classmethod
    def _get_proposals(
        cls, elves: list[Elf], search_seq: list[CompassSearchArea] | deque
    ) -> dict[GridLoc, list[Elf]]:
        elf_locs = {elf.loc for elf in elves}
        proposals: defaultdict[GridLoc, list[Elf]] = defaultdict(list)
        for elf in elves:
            if not (elf_prop_loc := elf.search_propose(elf_locs, search_seq)):
                continue
            proposals[elf_prop_loc].append(elf)
        return dict(proposals)

    @classmethod
    def _handle_elf_rounds(cls, locs: list[GridLoc], rounds: int) -> list[GridLoc]:
        elves = [Elf(loc) for loc in locs]
        search_seq = deque([SEARCH_NORTH, SEARCH_SOUTH, SEARCH_WEST, SEARCH_EAST])
        # cls.dump(elves)
        for i in range(rounds):
            LOG.info("Starting round %d, with sequence of %s", i, search_seq)
            proposals = cls._get_proposals(elves, search_seq)
            for prop_loc, prop_elves in proposals.items():
                if len(prop_elves) != 1:
                    LOG.debug(
                        "Location %s was proposed by multiple elves %s",
                        prop_loc,
                        prop_elves,
                    )
                    continue
                LOG.debug("Elf %s can move new location %s", prop_elves[0], prop_loc)
                prop_elves[0]._approve_proposal(prop_loc)

            search_seq.append(search_seq.popleft())
            # cls.dump(elves)

        return [elf.loc for elf in elves]

    @staticmethod
    def grid_bounds(points: Iterable[GridLoc]) -> tuple[int, int, int, int]:
        """Return bounds of rectangle.

        Args:
            points (list[GridLoc]): Points that must be contained in the rectangle

        Returns:
            tuple[int, int, int, int]: x_min, y_min, x_max, y_max
        """
        h_min, h_max = float("inf"), 0
        w_min, w_max = float("inf"), 0
        for point in points:
            h_min = min(h_min, point.y_pos)
            h_max = max(h_max, point.y_pos)

            w_min = min(w_min, point.x_pos)
            w_max = max(w_max, point.x_pos)

        if isinstance(h_min, float) or isinstance(w_min, float):
            LOG.debug("There were no points to iterate over - returning empty rect")
            return 0, 0, 0, 0
        LOG.debug(
            "Rectangle goes from: x [%d, %d], y [%d, %d]", w_min, w_max, h_min, h_max
        )
        return w_min, h_min, w_max, h_max

    @classmethod
    def dump(cls, elves: list[Elf], printf: Callable = print) -> None:
        """Debugging tool to make sure rectangle is bounding properly.

        Prints to sysout by default.
        """
        points = {elf.loc for elf in elves}
        bounds = cls.grid_bounds(points)
        printf("=" * 32)
        for row in range(bounds[1], bounds[3] + 1):
            for col in range(bounds[0], bounds[2] + 1):
                printf("#" if (col, row) in points else ".", end="")
            printf()
        printf("=" * 32)

    def part1(self, data: list[GridLoc]) -> int:
        """Calculate the amount of free space in special rectangle.

        Simulates 10 rounds then finds the amount of space.
        The rectangle must contain all elves but limit free space.
        """
        LOG.info("-" * 20 + "starting part1" + "-" * 20)
        end_locations = self._handle_elf_rounds(data, rounds=10)
        LOG.info("Received final locations of %s", end_locations[:5])
        bounds = self.grid_bounds(end_locations)
        height, width = bounds[3] - bounds[1] + 1, bounds[2] - bounds[0] + 1
        LOG.info("Final grid has height %d and width %d", height, width)
        return height * width - len(end_locations)

    def part2(self, data: list[GridLoc]) -> int:
        """Return round number that the Elves stop moving."""
        LOG.info("-" * 20 + "starting part2" + "-" * 20)
        elves = [Elf(loc) for loc in data]
        search_seq = deque([SEARCH_NORTH, SEARCH_SOUTH, SEARCH_WEST, SEARCH_EAST])
        # cls.dump(elves)
        i = 0
        while True:
            i += 1
            LOG.info("Starting round %d, with sequence of %s", i, search_seq)
            proposals = self._get_proposals(elves, search_seq)
            if len(proposals) == 0:
                return i
            for prop_loc, prop_elves in proposals.items():
                if len(prop_elves) != 1:
                    LOG.debug(
                        "Location %s was proposed by multiple elves %s",
                        prop_loc,
                        prop_elves,
                    )
                    continue
                LOG.debug("Elf %s can move new location %s", prop_elves[0], prop_loc)
                prop_elves[0]._approve_proposal(prop_loc)

            search_seq.append(search_seq.popleft())
            # cls.dump(elves)


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 23, 2022
    day = Day23()

    if args["--example"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.log", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)
        data = EXAMPLE
    elif args["--local"]:
        data = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        sys.exit(0)
    for ans, part in zip(answers, ["a", "b"], strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
