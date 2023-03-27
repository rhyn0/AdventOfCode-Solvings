# Standard Library
from collections import Counter
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


class Day6(Day):
    """Day 6 of Advent of Code 2022."""

    def parse(self, data_input: str) -> str:
        """Given input return it."""
        return data_input

    def part1(self, data: str) -> int:
        """Return number of characters processed until finding 4 unique in a row.

        Args:
            data (str): input string

        Returns:
            int
        """
        return self._count_n_distinct(data)

    def _count_n_distinct(self, data: str, num: int = 4) -> int:
        marker_set = Counter(data[:num])
        for process, char in enumerate(data[num:], start=num):
            if len(marker_set) == num:
                return process
            if marker_set[data[process - num]] == 1:
                marker_set.pop(data[process - num])
            else:
                marker_set[data[process - num]] -= 1
            marker_set[char] += 1

        return -1

    def part2(self, data: str) -> int:
        """Return number of characters processed until finding 14 unique in a row.

        Args:
            data (str): input string

        Returns:
            int
        """
        return self._count_n_distinct(data, 14)


if __name__ == "__main__":
    DAY, YEAR = 6, 2022
    day = eval(f"Day{DAY}()")
    answers = day.solve(get_data(day=DAY, year=YEAR))
    # print(answers)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
