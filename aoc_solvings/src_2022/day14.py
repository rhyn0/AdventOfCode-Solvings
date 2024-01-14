"""Advent of Code Day14 problem.

Usage:
    day14.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

Options:
    --example       Use example input rather than running personal input.
    --local         Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose       Use python logging to get verbose output of what is going on
                    in a log file.
    --quiet         Disable logging for example mode.
    --parts PART    Do only specified part, options are 'a', 'b', or 'ab'. [default: ab]
"""
# Standard Library
from collections import defaultdict
from collections.abc import Generator
from collections.abc import Iterator
from dataclasses import dataclass
from dataclasses import field
from enum import IntEnum
from itertools import pairwise
import logging
from operator import itemgetter
import os
from pathlib import Path
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

LOG_NAME = "day14"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9"""
)


class GridFillType(IntEnum):
    """Enumerate the items that fill in the grid."""

    AIR = 0
    ROCK = 1
    SAND = 2


@dataclass
class SandCave:
    """Class to handle the logic around the Cave filling with Sand."""

    grid: dict[tuple[int, int], int]
    lower_bound: int
    _orig_grid: dict[tuple[int, int], int] = field(
        init=False, repr=False, compare=False
    )
    _SAND_SPOT: tuple[int, int] = field(default=(500, 0), init=False)  # x dist, y dist

    def __post_init__(self) -> None:
        """Use an additional attribute to save the original grid."""
        # will use this to reset the grid after the calculation
        self._orig_grid = self.grid.copy()

    def calculate_sand_move(
        self, x_pos: int, y_pos: int, grid: dict[tuple[int, int], int]
    ) -> tuple[int, int]:
        """Calculate where the current piece of sand will move.

        Move down if spot below is empty, move diagonal left down if that spot,
        then move diagonal right down.

        Args:
            x_pos (int): sand's x position
            y_pos (int): sand's y position (0 ceiling)
            grid (Dict[Tuple[int, int], int]): current Cave state

        Returns:
            Tuple[int, int]: Change in x and y position
        """
        d_x = d_y = 0
        if grid[(x_pos, y_pos + 1)] == GridFillType.AIR and y_pos < self.lower_bound:
            # drop down
            LOG.debug(
                "Sand %d at %s is dropping down", self.sand_amount + 1, (x_pos, y_pos)
            )
            d_y = 1
        elif (
            grid[(x_pos - 1, y_pos + 1)] == GridFillType.AIR
            and y_pos < self.lower_bound
        ):
            # diag left
            LOG.debug(
                "Sand %d at %s is dropping diagonal left",
                self.sand_amount + 1,
                (x_pos, y_pos),
            )
            d_x = -1
            d_y = 1
        elif (
            grid[(x_pos + 1, y_pos + 1)] == GridFillType.AIR
            and y_pos < self.lower_bound
        ):
            # diag right
            LOG.debug(
                "Sand %d at %s is dropping diagonal right",
                self.sand_amount + 1,
                (x_pos, y_pos),
            )
            d_x = 1
            d_y = 1
        else:
            # resting
            LOG.debug(
                "Sand %d at %s can not move so stopping",
                self.sand_amount + 1,
                (x_pos, y_pos),
            )
            self.sand_amount += 1
            grid[(x_pos, y_pos)] = GridFillType.SAND
            self._draw_grid()
        return (d_x, d_y)

    def check_sand_reset(self, x_pos: int, y_pos: int) -> tuple[int, int]:
        """Adjust current position based on sand stop state."""
        if self.grid[(x_pos, y_pos)] == GridFillType.AIR:
            return (x_pos, y_pos)
        LOG.debug("Hit stop point, generating new sand drop")
        return self._SAND_SPOT

    def count_sand(self, *, floor: bool = False) -> int:
        """Simulate sand falling into cave until clear condition met.

        Based on floor input, will run until sand starts falling past the
        last known rock. Or it will drop sand until
        impossible to drop any more sand.

        Args:
            data (Dict[Tuple[int, int], int]): Dictionary of locations of
                pre-existing rocks
            floor (bool, optional): Whether the cave has an infinitely wide floor.
                Defaults to False.

        Raises:
            AnswerNotFoundError: If somehow sand flow can be stopped while
                having a bottomless abyss

        Returns:
            int: Amount of sand needed to clear
        """
        curr_x, curr_y = self._SAND_SPOT
        self.sand_amount = 0
        LOG.info("Maximum rock depth is %d", self.lower_bound)
        while True:
            curr_x, curr_y = self.check_sand_reset(curr_x, curr_y)
            if curr_y > self.lower_bound - 1 and not floor:
                LOG.debug(
                    "Sand %d has fallen past last known rock at %s",
                    self.sand_amount + 1,
                    (curr_x, curr_y),
                )
                return self.sand_amount

            delta_x, delta_y = self.calculate_sand_move(curr_x, curr_y, self.grid)
            if (delta_x, delta_y) != (0, 0):
                curr_x += delta_x
                curr_y += delta_y

            if (curr_x, curr_y) == self._SAND_SPOT:
                # just filled sand pouring,
                break
        if not floor:
            raise AnswerNotFoundError()
        return self.sand_amount

    def reset_grid(self) -> None:
        """Restore grid state to original argument."""
        self.grid = self._orig_grid.copy()

    def _draw_grid(
        self,
        side_padding: int = 10,
        depth_padding: int = 3,
    ) -> None:
        LOG.info("$" * 36)
        for row in range(self.lower_bound + depth_padding):
            line = ""
            for col in range(
                self._SAND_SPOT[0] - side_padding, self._SAND_SPOT[0] + side_padding + 1
            ):
                if self.grid[(col, row)] == GridFillType.AIR:
                    line += "."
                elif self.grid[(col, row)] == GridFillType.ROCK:
                    line += "#"
                else:
                    line += "O"
            LOG.info("%r", line)

        LOG.info("$" * 36)


class Day14(Day):
    """Day 14 of Advent of Code 2022."""

    @staticmethod
    def _gen_points_line(
        start: int, end: int, fixed_loc: int, *, fixed_row: bool = True
    ) -> Iterator[tuple[int, int]]:
        if start > end:
            start, end = end, start
        yield from (
            (gen_loc, fixed_loc) if fixed_row else (fixed_loc, gen_loc)
            for gen_loc in range(start, end + 1)
        )

    @classmethod
    def _gen_line(
        cls, x1: int, y1: int, x2: int, y2: int
    ) -> Generator[tuple[int, int], None, None]:
        if x1 != x2:
            yield from cls._gen_points_line(x1, x2, y1, fixed_row=True)
        elif y1 != y2:
            yield from cls._gen_points_line(y1, y2, x1, fixed_row=False)

    def parse(self, puzzle_input: str) -> SandCave:
        """Return grid of cave after parsing the rock locations.

        Rocks will be represented by 1, air by 0.
        """
        rocks = set()
        lower_bound = 0
        for line in puzzle_input.splitlines():
            for pt1, pt2 in pairwise(line.split("->")):
                x1, y1 = (int(num) for num in pt1.split(","))
                x2, y2 = (int(num) for num in pt2.split(","))
                lower_bound = max(lower_bound, y1, y2)
                rocks.update(self._gen_line(x1, y1, x2, y2))
        LOG.debug(
            "generated rocks at locations %s",
            sorted(rocks, key=itemgetter(0, 1)),
        )
        lower_bound += 1
        return SandCave(
            defaultdict(
                lambda: GridFillType.AIR,
                {rock_pos: GridFillType.ROCK for rock_pos in rocks},
            ),
            lower_bound,
        )

    def part1(self, data: SandCave) -> int:
        """Return amount of sand needed until one falls into the abyss."""
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        data.reset_grid()
        return data.count_sand()

    def part2(self, data: SandCave) -> int:
        """Return amount of sand at which point no more sand can flow."""
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        data.reset_grid()
        return data.count_sand(floor=True)


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 14, 2022
    day = Day14()
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
        assert answers == (24, 93)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
