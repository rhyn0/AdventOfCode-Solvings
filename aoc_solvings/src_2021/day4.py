# Standard Library
from dataclasses import dataclass
from dataclasses import field

# External Party
from aocd import get_data
from aocd import submit

# My Modules
from common.template import Day


class BingoSizeError(Exception):
    """Error for Bingo Grid size being invalid."""

    def __init__(
        self, grid: list[list[int]], *args: object, desired_size: int = 5
    ) -> None:
        """Adds size of bingo board to error."""
        super().__init__(
            f"Should be a {desired_size}x{desired_size} grid, got a\
                {len(grid)}x{max(len(row) for row in grid)}",
            *args,
        )


@dataclass
class BingoBoard:
    """BingoBoard object for use in AOCD problem."""

    numbers: list[list[int]]
    called: list[list[bool]] = field(default_factory=list, init=False)
    count: int = field(default=0, init=False)

    BINGO_LEN: int = 5

    def __post_init__(self):
        """Instantiate other board items, that aren't dataclass compliant."""
        if len(self.called) != self.BINGO_LEN:
            self.called = [[False for _ in range(5)] for _ in range(5)]
        if len(self.numbers) != self.BINGO_LEN or any(
            len(row) != self.BINGO_LEN for row in self.numbers
        ):
            raise BingoSizeError(self.numbers, desired_size=self.BINGO_LEN)

    def num_called(self, num: int) -> None:
        """Set the board for when a number is called."""
        self.count += 1
        for row, row_array in enumerate(self.numbers):
            for col, val in enumerate(row_array):
                if val == num:
                    self.called[row][col] = True
                    return

    def _check_row(self) -> bool:
        return any(all(x for x in row) for row in self.called)

    def _check_col(self) -> bool:
        return any(all(self.called[i][j] for i in range(5)) for j in range(5))

    def is_bingo(self) -> bool:
        """Check for winnning 5 in a row."""
        return (
            (self._check_col() or self._check_row())
            if self.count >= self.BINGO_LEN
            else False
        )

    def sum_uncalled(self) -> int:
        """Return cells uncalled."""
        return sum(
            self.numbers[i][j]
            for i, row in enumerate(self.called)
            for j, val in enumerate(row)
            if not val
        )


class Day4(Day):
    """Day 4 from Advent of Code 2021."""

    def __init__(self) -> None:
        """Add puzzles to object init."""
        super().__init__()
        self.puzzles: list[BingoBoard] = []

    def parse(self, puzzle_input: str) -> list[int]:
        """Parse data input."""
        lines_data = puzzle_input.split("\n")
        for i in range(2, len(lines_data), 6):
            self.puzzles.append(
                BingoBoard(
                    [
                        [int(x.strip()) for x in line.split()]
                        for line in lines_data[i : i + 5]
                    ]
                )
            )
        return [int(x.strip()) for x in lines_data[0].split(",")]

    def part1(self, data: list[int]) -> int:
        """Do part 1."""
        for val in data:
            for puz in self.puzzles:
                puz.num_called(val)
                if puz.is_bingo():
                    return puz.sum_uncalled() * val

        return -1

    def part2(self, data: list[int]) -> int:
        """Do part 2."""
        for val in data:
            for puz in self.puzzles:
                puz.num_called(val)
            if len(self.puzzles) == 1 and self.puzzles[0].is_bingo():
                return self.puzzles[0].sum_uncalled() * val
            self.puzzles = [puz for puz in self.puzzles if not puz.is_bingo()]
        return -1


if __name__ == "__main__":
    day = Day4()
    answers = day.solve(get_data(day=4, year=2021))
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=4, year=2021, part=part)
