"""Advent of Code Day20 problem.

Usage:
    day20.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

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
from itertools import repeat
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent
from typing import TYPE_CHECKING
from typing import ClassVar

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

LOG_NAME = "day20"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    1
    2
    -3
    3
    -2
    0
    4"""
)


class Day20(Day):
    """Day 20 of Advent of Code 2022."""

    GROVE_INDICES: ClassVar = [1000, 2000, 3000]

    def parse(self, puzzle_input: str) -> list[str]:
        """Return numbers in original order."""
        ret_dict = {}
        # python dict keys are always iterable in order
        # also gives O(1) lookup for existence
        for pos, line in enumerate(puzzle_input.splitlines()):
            i = 0
            while line in ret_dict:
                i += 1
                line = f"{line.split('_')[0]}_{i}"

            ret_dict[line] = pos

        return list(ret_dict)

    @staticmethod
    def _mult_line(str_num: str, multiplier: int) -> str:
        return "_".join(
            [
                str(int(val) * multiplier) if ind == 0 else val
                for ind, val in enumerate(str_num.split("_"))
            ]
        )

    @staticmethod
    def generate_item_keys(nums: list[str]) -> Iterator[tuple[str, int]]:
        """Generate key and num to mix around."""
        yield from [(key, num) for key in nums if (num := int(key.split("_")[0])) != 0]

    def mix_coords(
        self, nums: list[str], mix_times: int = 1, multiplier: int = 1
    ) -> list[str]:
        """Given the num list, do the mixing of the array and return the mixed list.

        Args:
            nums (list[str]): Data nums to mix
            mix_times (int, optional): Number of times to mix the array. Defaults to 1.
            multiplier (int, optional): Modify the numbers by this
                multiplicand in the array prior to mixing. Defaults to 1.

        Returns:
            list[str]: Mixed array
        """
        num_len = len(nums)
        LOG.debug("Starting with nums array of %s", nums)
        # use data dict, since nums must be processed in original order
        if multiplier != 1:
            nums = [self._mult_line(num, multiplier) for num in nums]
        LOG.debug("after multiplying have nums %s", nums)
        for curr_nums in repeat(nums[:], mix_times):
            for key, num in self.generate_item_keys(curr_nums):
                start_ind = nums.index(key)
                # len - 1 here because there are N - 1 jumps
                # possible in array before wrapping
                end_ind = (start_ind + num) % (num_len - 1)
                if end_ind == 0:
                    end_ind = num_len - 1
                nums.insert(end_ind, nums.pop(start_ind))
                LOG.debug(
                    "Moved number %r from %d to %d. Nums are now %s",
                    key,
                    start_ind,
                    end_ind,
                    nums,
                )
            LOG.info("Nums after mixing this time is %s", nums)
        return nums

    def part1(self, data: list[str]) -> int:
        """Given the circular array of nums, mix them and return sum of Grove indices.

        Grove indices are 1000, 2000, 3000.
        Mixing is iterating over the numbers in original order and moving
        them to the right that number's value times
        If a number is negative, move it left for its value. e.g. -1 needs to
        be moved one index left

        Args:
            data (list[str]): Original nums array

        Returns:
            int: Sum of the grove indices of the now mixed list.
        """
        LOG.info("%s starting part1 %s", "-" * 20, "-" * 20)
        nums = self.mix_coords(data.copy())
        num_len = len(nums)
        zero_spot = nums.index("0")
        LOG.debug(
            "Final grove numbers are %s",
            [
                int(nums[(zero_spot + grove) % num_len].split("_")[0])
                for grove in self.GROVE_INDICES
            ],
        )
        return sum(
            int(nums[(zero_spot + grove) % num_len].split("_")[0])
            for grove in self.GROVE_INDICES
        )

    def part2(self, data: list[str]) -> int:
        """Given the circular array of numbers, return sum of grove indices.

        But modify the numbers by the decryption key and mix it 10 times.

        Args:
            data (list[str]): Num string

        Returns:
            int
        """
        LOG.info("%s starting part2 %s", "-" * 20, "-" * 20)
        nums = self.mix_coords(data.copy(), mix_times=10, multiplier=811589153)
        num_len = len(nums)
        zero_spot = nums.index("0")
        LOG.debug(
            "Final grove numbers are %s",
            [
                int(nums[(zero_spot + grove) % num_len].split("_")[0])
                for grove in self.GROVE_INDICES
            ],
        )
        return sum(
            int(nums[(zero_spot + grove) % num_len].split("_")[0])
            for grove in self.GROVE_INDICES
        )


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 20, 2022
    day = Day20()
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
        assert answers == (3, 1623178306)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
