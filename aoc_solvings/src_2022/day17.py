"""Advent of Code Day17 problem.

Usage:
    day17.py [--example]

Options:
    --example   Use example input rather than running personal input.
"""

from __future__ import annotations

# Standard Library
from itertools import cycle
import logging
import os
import sys
from textwrap import dedent

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

LOG_NAME = "day17"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"""
)


class NotTetris:
    """Tetris like system for Day 17."""

    def __init__(
        self, jet_commands: str, cave_width: int = 7, vert_space: int = 3
    ) -> None:
        """Init object with cycle of jet streams."""
        self.jet_moves = cycle(jet_commands)
        # cave is representation of cave with rocks, 0 air 1 rock
        self.shapes = cycle(
            [self._add_flat, self._add_plus, self._add_l, self._add_vert, self._add_box]
        )
        self._cave_width = cave_width
        self._vert_space = vert_space
        # cave is setup to be each list is a row of with `cave_width` columns.
        # lower index is lower height.
        self.cave: list[int] = []

    @property
    def height(self) -> int:
        """How high rock is filling cave at this moment."""
        return len(self.cave)

    def _add_flat(self) -> list[str]:
        # add flat rock to cave
        return ["0011110"]

    def _add_plus(self) -> list[str]:
        # add plus rock to cave
        return [
            "0001000",
            "0011100",
            "0001000",
        ]

    def _add_l(self) -> list[str]:
        # add L rock to cave
        return [
            "0000100",
            "0000100",
            "0011100",
        ]

    def _add_vert(self) -> list[str]:
        # add flat rock to cave
        return ["0010000" for _ in range(4)]

    def _add_box(self) -> list[str]:
        # add box rock to cave
        return [
            "0011000",
            "0011000",
        ]

    def can_drop(self, rock: list[int], level: int) -> bool:
        """Return whether a rock can continue to descend.

        Compare if rock has reach bottom of cave or if it has touched
        existing cave elements.

        Args:
            rock (list[int]): rock shape
            level (int): level of cave that rock has descended to

        Returns:
            bool: True if it can descend, False otherwise
        """
        # return modified rock after drop
        for r_level, rock_layer in enumerate(rock):
            if (
                level + r_level + 1 >= len(self.cave)
                or rock_layer & self.cave[level + r_level + 1]
            ):
                LOG.debug(
                    "Cannot drop rock %s further: depth %d compared to height %d",
                    rock,
                    level + r_level + 1,
                    self.height,
                )
                return False
        return True

    def shift(self, rock: list[int], direction: str, level: int) -> list[int]:
        """Attempt to shift the rock sideways according to direction.

        If rock can't shift, early escape

        Args:
            rock (list[int]): rock to shift
            direction (str): direction, '<' or '>'
            level (int): Level of cave that deepest part of rock has descended to

        Returns:
            list[int]: Modified rock after shift
        """
        for r_level, rock_layer in enumerate(rock):
            if (
                direction == "<"
                and (
                    (rock_layer & 2**6)
                    or (rock_layer << 1 & self.cave[level + r_level])
                )
            ) or (
                direction == ">"
                and (
                    (rock_layer & 2**0)
                    or (rock_layer >> 1 & self.cave[level + r_level])
                )
            ):
                LOG.debug(
                    "Failed shift %r of rock %s at level %d - %d",
                    direction,
                    rock,
                    r_level,
                    rock_layer,
                )
                # something is touching, either edge
                # or dropping rock with static cave
                return rock

        return [
            rock_layer << 1 if direction == "<" else rock_layer >> 1
            for rock_layer in rock
        ]

    def drop_rock(self) -> None:
        """Add another rock to cave.

        Main function of the system.
        """
        curr_rock = [int(layer, 2) for layer in next(self.shapes)()]
        self.cave = [0] * (self._vert_space + len(curr_rock)) + self.cave
        for level in range(len(self.cave)):
            curr_dir = next(self.jet_moves)
            curr_rock = self.shift(curr_rock, curr_dir, level)
            LOG.debug("After shift %r rock is %s", curr_dir, curr_rock)
            if not self.can_drop(curr_rock, level):
                LOG.debug("Rock cannot drop anymore, adding to cave")
                for r_level, rock_layer in enumerate(curr_rock):
                    self.cave[level + r_level] |= rock_layer
                while self.cave[0] == 0:
                    self.cave.pop(0)
                LOG.debug("After adding rock to cave, height is %d", self.height)
                # rock finishes droopping, its done
                return


class Day17(Day):
    """Day 17 of Advent of Code 2022."""

    def parse(self, puzzle_input: str) -> str:
        """Return directions of jets.

        Numerated out as left is -1, and right is 1.
        """
        return puzzle_input.strip()

    def _find_pattern(self, data: list[int]) -> tuple[list[int], list[int]]:
        # find seperation between random noise and repetition area
        for noise_pt in range(len(data)):
            search_space = data[noise_pt:]
            # unclear at what rate it repeats so try all
            for repeat_len in range(2, len(search_space) // 2):
                if (
                    search_space[0:repeat_len]
                    == search_space[repeat_len : 2 * repeat_len]
                ) and all(
                    search_space[0:repeat_len]
                    == search_space[chunk : chunk + repeat_len]
                    for chunk in range(
                        repeat_len, len(search_space) - repeat_len, repeat_len
                    )
                ):
                    return data[:noise_pt], data[noise_pt : noise_pt + repeat_len]
        return [], []

    def part1(self, data: str) -> int:
        """Drop 2022 rocks and return height of cave."""
        LOG.info("Received jet stream info of %r", data)
        system = NotTetris(data)
        for _ in range(2022):
            system.drop_rock()

        return system.height

    def part2(self, data: str) -> int:
        """Return height of cave after 1 trillion rocks.

        Drops a small sample of rocks and then finds a pattern and does
        math to calculate the rest.

        Args:
            data (str): jet stream cycle

        Returns:
            int
        """
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        num_rocks = 1_000_000_000_000
        sample = 10_000
        system = NotTetris(data)
        heights = []
        for _ in range(sample):
            prev = system.height
            system.drop_rock()
            heights.append(system.height - prev)

        preamble, repetition = self._find_pattern(heights)
        LOG.info(
            "Found noisy sequence of %s, and repetition sequence of length %d - %s",
            preamble,
            len(repetition),
            repetition,
        )

        p_len, r_len = len(preamble), len(repetition)
        even_repeats, leftovers = divmod(num_rocks - p_len, r_len)
        LOG.info(
            "Repetition sequence repeats %d times with %d numbers extra",
            even_repeats,
            leftovers,
        )
        return (
            sum(preamble) + sum(repetition) * even_repeats + sum(repetition[:leftovers])
        )


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 17, 2022
    day = Day17()
    if args["--example"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.txt", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)
        data = EXAMPLE
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
