# Standard Library
from abc import ABC
from abc import abstractmethod
from pathlib import Path
from typing import Any

# External Party
from aocd import get_data
from aocd import submit


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


def get_data_args(
    args: dict[str, Any],
    day: int,
    year: int = 2021,
) -> str:
    """Return the data for the given day and year."""
    global EXAMPLE
    if args["--example"]:
        return EXAMPLE  # type: ignore[name-defined]
    if args["--local"]:
        return (
            (Path(__file__).parents[1] / "data" / f"input{year}-{day}.txt")
            .open()
            .read()
        )
    return get_data(day=day, year=year)


def submit_answers(
    answers: tuple[Any, Any], parts: str, day: int, year: int = 2021
) -> None:
    """Submit the answers for the given day and year.

    Args:
        answers (tuple[Any, Any]): Answer tuple - (part1, part2).
        parts (str): String of parts to submit. Ex: "ab".
        day (int): Which day to submit for
        year (int, optional): Which year to submit for. Defaults to 2021.
    """
    for ans, part in zip(answers, "ab", strict=True):
        if part not in parts:
            continue
        submit(ans, day=day, year=year, part=part)
