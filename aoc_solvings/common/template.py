# Standard Library
from abc import ABC
from abc import abstractmethod


class AnswerNotFoundError(Exception):
    """Error for when the answer couldn't be found."""

    def __init__(self, *args: object) -> None:
        """Default message and pass through args."""
        super().__init__("Correct answer couldn't be found", *args)


class Day(ABC):
    """Basic template for Advent of Code challenges."""

    @abstractmethod
    def parse(self, puzzle_input):
        """Parse input."""

    @abstractmethod
    def part1(self, data):
        """Solve part 1."""

    @abstractmethod
    def part2(self, data):
        """Solve part 2."""

    def solve(self, puzzle_input, /, parts: str = "ab"):
        """Solve the puzzle for the given input."""
        data = self.parse(puzzle_input)
        solution1 = self.part1(data) if "a" in parts else None
        solution2 = self.part2(data) if "b" in parts else None

        return solution1, solution2
