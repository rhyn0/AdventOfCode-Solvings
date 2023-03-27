# External Party
from aocd import get_data
from aocd import submit
import numpy as np

# My Modules
from common.template import Day


class Day11(Day):
    """Day 11 of Advent of Code 2021."""

    P2_MAX_FLASHES = 100
    FISH_LIFE_MAX = 9
    BOARD_LEN = 10

    def parse(self, puzzle_input: str) -> np.ndarray:
        """Parse input."""
        return np.array([[int(x) for x in line] for line in puzzle_input.split("\n")])

    @classmethod
    def is_in_bounds(cls, row, col) -> bool:
        """If point is on board."""
        return (0 <= row < cls.BOARD_LEN) and (0 <= col < cls.BOARD_LEN)

    def flash(self, row, col, data: np.ndarray) -> int:
        """Fish at point will flash and affect those around it."""
        if not self.is_in_bounds(row, col):
            return 0
        if data[row, col] <= self.FISH_LIFE_MAX:
            return 0
        changes = 1
        data[row, col] = 0
        for d_row in [-1, 0, 1]:
            for d_col in [-1, 0, 1]:
                if (
                    not self.is_in_bounds(row + d_row, col + d_col)
                    or data[row + d_row, col + d_col] == 0
                ):
                    continue
                data[row + d_row, col + d_col] = data[row + d_row, col + d_col] + 1
                changes += self.flash(row + d_row, col + d_col, data)
        return changes

    def part1(self, data: np.ndarray) -> int:
        """Do part 1."""
        tot_flashes = 0
        # preset 100 steps
        for _ in range(100):
            data = data + 1
            row_ind, col_ind = np.where(data > self.FISH_LIFE_MAX)
            for row, col in zip(row_ind, col_ind, strict=True):
                tot_flashes += self.flash(row, col, data)
        return tot_flashes

    def part2(self, data: np.ndarray) -> int:
        """Do part 2."""
        step = 1
        while True:
            change = 0
            data = data + 1
            row_ind, col_ind = np.where(data > self.FISH_LIFE_MAX)
            for row, col in zip(row_ind, col_ind, strict=True):
                change += self.flash(row, col, data)
            if change == self.P2_MAX_FLASHES:
                return step
            step += 1


if __name__ == "__main__":
    day = Day11()
    answers = day.solve(get_data(day=11, year=2021))
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, part=part, day=11, year=2021)
