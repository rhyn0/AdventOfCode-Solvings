# Standard Library
from collections import deque
from functools import reduce

# External Party
from aocd import get_data
from aocd import submit
from numpy import median

# My Modules
from common.template import Day


class CorruptedLineError(ValueError):
    """Custom Error when line input is invalid."""


class Day10(Day):
    """Day10 problems."""

    def parse(self, puzzle_input: str) -> list[str]:
        """Return input separated by new lines as list."""
        return puzzle_input.split("\n")

    @staticmethod
    def is_closing_pair(front: str, end: str) -> bool:
        """Given an enclosing pair, confirm that front is the connector for end."""
        match end:
            case ")":
                return front == "("
            case "]":
                return front == "["
            case "}":
                return front == "{"
            case ">":
                return front == "<"
            case _:
                return False

    @staticmethod
    def update_stack(char_string: str, stack: deque) -> None:
        """Update stack based on new character."""
        if char_string in [")", "]", "}", ">"]:
            if not Day10.is_closing_pair(stack[0], char_string):
                raise CorruptedLineError
            stack.popleft()  # matched
        else:
            stack.appendleft(char_string)

    def part1(self, data: list[str]) -> int:
        """Compute point value for each enclosing pair that is corrupted."""
        points_table = {
            ")": 3,
            "]": 57,
            "}": 1197,
            ">": 25137,
        }
        corrupted = []
        for line in data:
            stack = deque()
            for char in line:
                try:
                    self.update_stack(char, stack)
                except CorruptedLineError:
                    corrupted.append(char)
                    break

        return sum(points_table[c] for c in corrupted)

    def part2(self, data: list[str]):
        """Return points for incomplete enclosing pairs."""
        points_table = {
            "(": 1,
            "[": 2,
            "{": 3,
            "<": 4,
        }
        incomplete = []
        for line in data:
            stack = deque()
            for char in line:
                try:
                    self.update_stack(char, stack)
                except CorruptedLineError:
                    break
            else:
                incomplete.append(
                    reduce(lambda x, y: x * 5 + y, (points_table[c] for c in stack))
                )
        return int(median(incomplete))


if __name__ == "__main__":
    day = Day10()
    answers = day.solve(get_data(day=10, year=2021))
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, part=part, day=10, year=2021)
