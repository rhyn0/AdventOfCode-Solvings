# Standard Library
import os
import sys

# External Party
from aocd import get_data
from aocd import submit

try:
    # My Modules
    from common.template import Day
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.template import Day


class Day1(Day):
    """Day 1 of Advent of Code 2022."""

    def parse(self, data_input: str) -> list[str]:
        """Given input return each line."""
        return data_input.split("\n")

    def part1(self, data: list[str]) -> int:
        """Return maximum calories carried by a single elf.

        Elf calories amount is a list of numbers, elves seperated by a blank line.

        Args:
            data (List[str]): List of lines from input

        Returns:
            int: Maximum calories on one elf
        """
        elf_cals = []
        curr_cals = 0
        for val in data:
            if len(val) == 0:
                elf_cals.append(curr_cals)
                curr_cals = 0
            else:
                curr_cals += int(val)
        elf_cals.append(curr_cals)
        return max(elf_cals)

    def part2(self, data: list[str]) -> int:
        """Return summation of maximum calories from top 3 elves.

        Args:
            data (List[str]): List of lines from input

        Returns:
            int: Total calories on top 3 elves by calories carried
        """
        elf_cals = []
        curr_cals = 0
        for val in data:
            if len(val) == 0:
                elf_cals.append(curr_cals)
                curr_cals = 0
            else:
                curr_cals += int(val)
        elf_cals.append(curr_cals)
        elf_cals.sort()

        return sum(elf_cals[-3:])


if __name__ == "__main__":
    DAY, YEAR = 1, 2022
    day = Day1()
    answers = day.solve(get_data(day=DAY, year=YEAR))
    # print(answers)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
