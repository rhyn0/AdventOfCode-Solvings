# Standard Library
from itertools import chain
import os
from string import ascii_lowercase
from string import ascii_uppercase
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


class Day3(Day):
    """Day 3 of Advent of Code 2022."""

    _item_priority = {
        char: point
        for point, char in enumerate(chain(ascii_lowercase, ascii_uppercase), start=1)
    }

    def parse(self, data_input: str) -> list[str]:
        """Given input return each line."""
        return data_input.split("\n")

    def part1(self, data: list[str]) -> int:
        """Return sum of priority of double packed items.

        Item is considered double packed if it appears in both
        halves of an input string.
        Priority of each item is given by a-z -> 1-26, A-Z -> 27-52

        Args:
            data (List[str]): List of rucksack items

        Returns:
            int: Sum of priority of doubled items
        """
        priority_sum = 0
        for rucksack in data:
            rucksack_size = len(rucksack)
            compart1, compart2 = set(rucksack[: rucksack_size // 2]), set(
                rucksack[rucksack_size // 2 :]
            )
            priority_sum += self._item_priority[compart1.intersection(compart2).pop()]

        return priority_sum

    def part2(self, data: list[str]) -> int:
        """Return sum of priority for the shared items in each safety group.

        A safety group is a three adjacent lines. Each line belongs to one safety group.
        Each group will have only one item shared amongst all three bags.

        Args:
            data (List[str]): List of bags

        Returns:
            int: Sum of priority items from each safety group
        """
        safety_groups = [
            [data[index] for index in range(group_leader, group_leader + 3)]
            for group_leader in range(0, len(data), 3)
        ]

        return sum(
            self._item_priority[
                set(group[0])
                .intersection(set(group[1]))
                .intersection(set(group[2]))
                .pop()
            ]
            for group in safety_groups
        )


if __name__ == "__main__":
    DAY, YEAR = 3, 2022
    day = eval(f"Day{DAY}()")
    answers = day.solve(get_data(day=DAY, year=YEAR))
    # print(answers)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
