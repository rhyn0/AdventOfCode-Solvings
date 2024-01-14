"""Advent of Code Day24 problem.

Usage:
    day24.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

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
from collections import defaultdict
from dataclasses import dataclass
from dataclasses import field
from enum import Enum
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent
from typing import Any
from typing import NamedTuple

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

LOG_NAME = "day24"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    #.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#"""
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

    def __mul__(self, scalar: int) -> GridLoc:
        """Treat somewhat like vector, able to be scaled by constants."""
        return GridLoc(self.x_pos * scalar, self.y_pos * scalar)


class BlizzardDir(str, Enum):
    """Enum of 4 cardinal directions using GridLoc."""

    loc: GridLoc

    def __new__(cls, value: str, loc_change: GridLoc) -> BlizzardDir:
        """Override Enum member creator to make the value the string representation."""
        obj = str.__new__(cls, value)
        obj._value_ = value
        obj.loc = loc_change
        return obj

    UP = ("^", GridLoc(0, -1))
    DOWN = ("v", GridLoc(0, 1))
    RIGHT = (">", GridLoc(1, 0))
    LEFT = ("<", GridLoc(-1, 0))
    STAY = (".", GridLoc(0, 0))


@dataclass(slots=True)
class Blizzard:
    """Contains position on the grid and handles new spot logic."""

    loc: GridLoc
    direction: BlizzardDir

    def __hash__(self) -> int:
        """Easy to match in hash structures."""
        return hash((self.loc, self.direction))

    def __eq__(self, other: object) -> bool:
        """Override EQ since something going wrong with set containment."""
        if not isinstance(other, GridLoc) and not isinstance(other, tuple):
            return NotImplemented
        return (
            len(other) == len(GridLoc._fields)
            and self.loc == other[0]
            and self.direction == other[1]
        )


@dataclass
class IceValley:
    """Contains the general information about the input valley for runtime."""

    blizzards: list[Blizzard]
    x_bounds: tuple[int, int]
    y_bounds: tuple[int, int]
    goal: GridLoc
    _lookup_blizzs: set[Blizzard] = field(init=False)

    def __post_init__(self) -> None:
        """Add fast lookup field to class."""
        self._lookup_blizzs = set(self.blizzards)

    def __str__(self) -> str:
        """Only show the general info about the runtime object."""
        return dedent(
            f"""      IceValley(x_bounds: {self.x_bounds}, y_bounds: {self.y_bounds},
                      goal_loc: {self.goal})"""
        )

    def __repr__(self) -> str:
        """Show the original grid locations from input."""
        blizz_loc_dir: defaultdict[GridLoc, list[str]] = defaultdict(
            list,
        )
        for blizz in self.blizzards:
            blizz_loc_dir[blizz.loc].append(blizz.direction.value)
        outer = ["\n" + "#" * (sum(self.x_bounds) + 2)]
        for row in range(self.y_bounds[0], self.y_bounds[1]):
            line = "#"
            for col in range(self.x_bounds[0], self.x_bounds[1]):
                loc = GridLoc(col, row)
                if len(blizz_loc_dir[loc]) == 0:
                    line += "."
                elif len(blizz_loc_dir[loc]) == 1:
                    line += str(blizz_loc_dir[loc][0])
                else:
                    line += str(len(blizz_loc_dir[loc]))
            outer.append(line + "#")
        outer.append(
            "".join(
                "#" if self.goal.x_pos != x_ind - 1 else "."
                for x_ind in range(self.x_bounds[1] + 2)
            )
        )
        return "\n".join(outer)

    def is_open_spot(self, prop_loc: GridLoc, steps: int) -> bool:
        """Return if this spot is open after `steps` amount of time.

        Args:
            prop_loc (GridLoc): Desired location
            steps (int): amount of time passage

        Returns:
            bool: True if available for expedition to move to, False otherwise
        """
        LOG.debug(
            "Testing open-ness of %s, blockers are %s",
            prop_loc,
            [
                (prop_loc + (-direct.loc * steps), direct.name)
                for direct in BlizzardDir
                if (prop_loc + (-direct.loc * steps), direct) in self._lookup_blizzs
            ],
        )
        return all(
            (self.wrap_check(prop_loc + (-direct.loc * steps)), direct)
            not in self._lookup_blizzs
            for direct in BlizzardDir
        )

    def is_in_bounds(self, prop_loc: GridLoc) -> bool:
        """Return if point is located in the movable grid.

        This does return False for the start and end of the valley.
        """
        return (
            self.x_bounds[0] <= prop_loc.x_pos < self.x_bounds[1]
            and self.y_bounds[0] <= prop_loc.y_pos < self.y_bounds[1]
        )

    def wrap_check(self, prop_loc: GridLoc) -> GridLoc:
        """Return a new position that has been properly bounded to movable grid."""
        return GridLoc(
            prop_loc.x_pos % self.x_bounds[1], prop_loc.y_pos % self.y_bounds[1]
        )


class Day24(Day):
    """Day 24 of Advent of Code 2022."""

    def parse(self, puzzle_input: str) -> tuple[GridLoc, IceValley]:
        """Return the starting position and the IceValley."""
        lines = puzzle_input.splitlines()
        grid = [line[1:-1] for line in lines[1:-1]]
        blizzs = [
            Blizzard(GridLoc(x_ind, y_ind), BlizzardDir(char))
            for y_ind, line in enumerate(grid)
            for x_ind, char in enumerate(line)
            if char not in (".", "#")
        ]
        # minus two because last entry is a wall
        x_max, y_max = len(grid[0]), len(grid)
        maze_enter = GridLoc(lines[0].index(".") - 1, -1)
        maze_exit = GridLoc(lines[-1].index(".") - 1, y_max)
        return maze_enter, IceValley(blizzs, (0, x_max), (0, y_max), maze_exit)

    def _simple_cell_auto(self, data: tuple[GridLoc, IceValley], step: int = 1) -> int:
        curr_gen_positions = {data[0]}
        iv = data[1]
        LOG.info("IceValley looks like %s: %r", iv, iv)
        LOG.info("There are blizzards at %s", iv.blizzards)
        time = step
        while True:
            LOG.debug("Working at time %d from positions %s.", time, curr_gen_positions)
            next_gen_positions = set()
            for loc in curr_gen_positions:
                # create all choices for this spot
                for direct in BlizzardDir:
                    new_loc = loc + direct.loc
                    if new_loc == iv.goal:
                        return time
                    LOG.debug(
                        "Testing location %s - conditions bounds %r and open %r",
                        new_loc,
                        iv.is_in_bounds(new_loc),
                        iv.is_open_spot(new_loc, time),
                    )
                    if iv.is_in_bounds(new_loc) and iv.is_open_spot(new_loc, time):
                        LOG.debug(
                            "At time %d adding %s to next generation of positions",
                            time,
                            new_loc,
                        )
                        next_gen_positions.add(new_loc)
            curr_gen_positions = next_gen_positions
            if not curr_gen_positions:
                LOG.warning("Hit a dead end! time is %d", time)
                curr_gen_positions.add(data[0])
            time += 1

    def part1(self, data: tuple[GridLoc, IceValley]):
        """Return number of minutes to go from start to finish."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        return self._simple_cell_auto(data)

    def part2(self, data: tuple[GridLoc, IceValley]) -> int:
        """Return number of minutes to go start to finish, back and forth."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        orig_start, iv = data
        orig_goal = iv.goal
        step1 = self._simple_cell_auto(data)
        iv.goal = orig_start
        step2 = self._simple_cell_auto((orig_goal, iv), step=step1)
        iv.goal = orig_goal
        return self._simple_cell_auto(data, step=step2)


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 24, 2022
    day = Day24()
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
        assert answers == (18, 54)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
