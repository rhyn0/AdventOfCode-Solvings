# Standard Library

# External Party
from aocd import get_data
from aocd import submit
import numpy

# My Modules
from common.template import Day


class Day7(Day):
    """Day 7 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> list[int]:
        """Return int values from each line from input."""
        return [int(val.strip()) for val in puzzle_input.split(",")]

    def part1(self, data: list[int]):
        """Inefficiently return the variance of data.

        abuse median values
        """
        return sum(abs(val - numpy.median(data)) for val in data)

    def part2(self, data):
        """Something about moving and sum of that."""

        def increasing_sum(moves: int) -> int:
            return sum(range(moves + 1))

        return sum(increasing_sum(abs(val - int(numpy.mean(data)))) for val in data)


if __name__ == "__main__":
    day = Day7()
    answers = day.solve(get_data(day=7, year=2021))
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, part=part, day=7, year=2021)
