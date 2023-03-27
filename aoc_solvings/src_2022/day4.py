# Standard Library
from operator import add
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


class Day4(Day):
    """Day 4 of Advent of Code 2022."""

    def parse(self, data_input: str) -> list[str]:
        """Given input return each line."""
        return data_input.split("\n")

    @staticmethod
    def _split_range(range_str: str) -> list[int]:
        # always a length two list
        return [int(x) for x in range_str.split("-")]

    @staticmethod
    def _split_range_pair(pair_str: str) -> list[list[int]]:
        # always returns a list of one length 4 list
        return [add(*[Day4._split_range(pair) for pair in pair_str.split(",")])]

    def part1(self, data: list[str]) -> int:
        """Return number of pairs where one range fully contains the other.

        Each item in `data` is a pair of ranges - represented as '1-3,4-6'.
        Comma seperated pairs, each range is a start (inclusive) and end (inclusive).
        Start and end can be same.

        Args:
            data (List[str]): Input of range pairs

        Returns:
            int: Number of pairs where one range fully contains the other
        """
        return sum(
            (start_1 <= start_2 and end_1 >= end_2)
            or (start_1 >= start_2 and end_1 <= end_2)
            for pair in data
            for start_1, end_1, start_2, end_2 in self._split_range_pair(pair)
        )

    def part2(self, data: list[str]) -> int:
        """Return number of pairs where there is any overlap.

        Same as part1 for the input.

        Args:
            data (List[str]): input of range pairs

        Returns:
            int: Number of pairs where there is any amount of overlap
        """
        return sum(
            start_1 <= start_2 <= end_1 or start_2 <= start_1 <= end_2
            for pair in data
            for start_1, end_1, start_2, end_2 in self._split_range_pair(pair)
        )


if __name__ == "__main__":
    DAY, YEAR = 4, 2022
    day = eval(f"Day{DAY}()")
    answers = day.solve(get_data(day=DAY, year=YEAR))
    # print(answers)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
