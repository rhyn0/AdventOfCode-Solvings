"""Advent of Code Day25 problem.

Usage:
    day25.py [--example | --local] [--verbose]

Options:
    --example   Use example input rather than running personal input.
    --local     Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose   Use python logging to get verbose output of what is going
                on in a log file.
"""
from __future__ import annotations

# Standard Library
from dataclasses import dataclass
from dataclasses import field
from enum import Enum
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent
from typing import Any
from typing import SupportsInt

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

LOG_NAME = "day25"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    1=-0-2
    12111
    2=0=
    21
    2=01
    111
    20012
    112
    1=-1=
    1-12
    12
    1=
    122"""
)


class SNAFUCharacter(str, Enum):
    """Enum of a base 5 numbering system."""

    decimal: int

    def __new__(cls, value: str, dec_val: int) -> SNAFUCharacter:
        """Override Enum member creator to make the value the string representation."""
        obj = str.__new__(cls, value)
        obj._value_ = value
        obj.decimal = dec_val
        return obj

    @classmethod
    def from_int(cls, value: SupportsInt) -> SNAFUCharacter:
        """Create new SNAFUCharacter based on decimal value of enum."""
        int_val = int(value)
        for en in cls:
            if en.decimal == int_val:
                return en
        raise KeyError(f"'{value}'")

    DOUBLE_MINUS = ("=", -2)
    SINGLE_MINUS = ("-", -1)
    ZERO = ("0", 0)
    ONE = ("1", 1)
    TWO = ("2", 2)


SNAFU_RANGE = 5


@dataclass
class SNAFUNum:
    """Contain multiple SNAFUCharacters to make a full number."""

    nums: list[SNAFUCharacter] = field(default_factory=list, init=False)

    def __str__(self) -> str:
        """Output as SNAFU notation."""
        return f"{''.join(num.value for num in self.nums)}"

    def __add__(self, other: Any) -> SNAFUNum:
        """Add by turning into decimals and then back."""
        if not isinstance(other, SNAFUNum):
            return NotImplemented
        value = self.to_decimal() + other.to_decimal()
        return self.from_decimal(value)

    @classmethod
    def from_decimal(cls, dec_val: SupportsInt) -> SNAFUNum:
        """Create new SNAFUNum from an integer value."""
        obj = cls()
        int_val = int(dec_val)
        LOG.debug("Creating SNAFUNum from int %d", int_val)
        while int_val > 0:
            int_val, remain = divmod(int_val, SNAFU_RANGE)
            LOG.debug(
                "After divmod by %d, have next as %d and remain %d",
                SNAFU_RANGE,
                int_val,
                remain,
            )
            if remain > SNAFUCharacter.TWO.decimal:
                remain = (
                    SNAFUCharacter.SINGLE_MINUS.decimal
                    if remain == SNAFU_RANGE - 1
                    else SNAFUCharacter.DOUBLE_MINUS.decimal
                )
                int_val += 1
                LOG.debug(
                    "Remainder is invalid in snafu, adjusting to %d and remain %d",
                    int_val,
                    remain,
                )
            obj.add_digit(SNAFUCharacter.from_int(remain))
            LOG.debug("New SNAFUNum status is %r", obj)
        LOG.debug("Returning new SNAFUNum of %r", obj)
        return obj

    @classmethod
    def from_str(cls, string: str) -> SNAFUNum:
        """Create a SNAFUNum from SNAFU string."""
        obj = cls()
        for char in string:
            obj.add_digit(SNAFUCharacter(char), greater=False)
        return obj

    def add_digit(self, num: SNAFUCharacter, greater: bool = True) -> None:
        """Augment current number by adding an extra digit.

        Args:
            num (SNAFUCharacter): Character to add to the number
            greater (bool, optional): Add to the beginning of the number,
                or the greatest base exponent position. Defaults to True.
        """
        if greater:
            self.nums.insert(0, num)
        else:
            self.nums.append(num)

    def to_decimal(self) -> int:
        """Return decimal base value.

        Returns:
            int
        """
        return sum(
            num.decimal * 5**exp_pow
            for exp_pow, num in enumerate(reversed(self.nums), start=0)
        )


class Day25(Day):
    """Day 25 of Advent of Code 2022."""

    def parse(self, puzzle_input: str) -> list[SNAFUNum]:
        """Return the fuel amounts in SNAFUNum for all balloons."""
        return [SNAFUNum.from_str(line) for line in puzzle_input.splitlines()]

    def part1(self, data: list[SNAFUNum]) -> str:
        """Return SNAFUNum string of the sum of all fuel amounts."""
        LOG.info("-" * 20 + "starting part1" + "-" * 20)
        fuel_sum = sum(num.to_decimal() for num in data)
        new_num = SNAFUNum.from_decimal(fuel_sum)
        LOG.info(
            "Got a total fuel count of %d - which is %s in SNAFU", fuel_sum, new_num
        )
        return str(new_num)

    def part2(self, data: list[SNAFUNum]) -> None:
        """Do nothing."""
        LOG.info("There is no Part2 for this year.")


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 25, 2022
    day = Day25()

    if args["--example"] or args["--verbose"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.log", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)

    if args["--example"]:
        data = EXAMPLE
    elif args["--local"]:
        data = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        sys.exit(0)
    # B part is readable only in terminal, can't upload via API
    for ans, part in zip(answers, "a", strict=False):
        submit(ans, day=DAY, year=YEAR, part=part)
