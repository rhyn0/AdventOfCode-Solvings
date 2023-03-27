# Standard Library
from functools import reduce

# External Party
from aocd import get_data
from aocd import submit

# My Modules
from common.template import Day


class Day9(Day):
    """Day 9 of Advent of Code 2021."""

    def parse(self, puzzle_input: str) -> list[list[int]]:
        """Return list of lists of ints for input."""
        return [[int(x) for x in line] for line in puzzle_input.split("\n")]

    @staticmethod
    def is_lowest_adj(array: list[list[int]], col: int, row: int) -> bool:
        """Return if there is a lower height point adjacent to given coordinate."""
        return all(
            array[row][col] < array[j][i]
            for i, j in zip(
                [col + 1, col, col - 1, col], [row, row + 1, row, row - 1], strict=False
            )
            if 0 <= i < len(array[0]) and 0 <= j < len(array)
        )

    def part1(self, data):
        """Return sum of low points in plane."""
        return sum(
            data[j][i] + 1
            for j in range(len(data))
            for i in range(len(data[0]))
            if self.is_lowest_adj(data, i, j)
        )

    @classmethod
    def check_basin(cls, row, col, depth_array: list[list[int]]) -> int:
        """Return max pooling of sludge for given topography map."""
        bottom, right = len(depth_array), len(depth_array[0])
        if depth_array[col][row] in [-1, 9]:
            return 0
        depth_array[col][row] = -1
        return (
            1
            + (cls.check_basin(row + 1, col, depth_array) if row < right - 1 else 0)
            + (cls.check_basin(row - 1, col, depth_array) if row > 0 else 0)
            + (cls.check_basin(row, col + 1, depth_array) if col < bottom - 1 else 0)
            + (cls.check_basin(row, col - 1, depth_array) if col > 0 else 0)
        )

    def part2(self, data) -> int:
        """Return sum of basin values."""
        return reduce(
            lambda x, y: x * y,
            sorted(
                (
                    self.check_basin(i, j, data)
                    for j in range(len(data))
                    for i in range(len(data[0]))
                    if self.is_lowest_adj(data, i, j)
                ),
                reverse=True,
            )[:3],
        )


if __name__ == "__main__":
    day = Day9()
    answers = day.solve(get_data(day=9, year=2021))
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, part=part, day=9, year=2021)
