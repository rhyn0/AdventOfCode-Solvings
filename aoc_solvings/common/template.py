# Standard Library
from abc import ABC
from abc import abstractmethod
import argparse
import logging
from pathlib import Path
import sys
from typing import Any
from typing import ClassVar

# External Party
from aocd import get_data
from aocd import submit

# My Modules
from common.log import edit_logger_for_verbosity


class AnswerNotFoundError(Exception):
    """Error for when the answer couldn't be found."""

    def __init__(self, *args: object) -> None:
        """Default message and pass through args."""
        super().__init__("Correct answer couldn't be found", *args)


class Day(ABC):
    """Basic template for Advent of Code challenges."""

    day: ClassVar[int]
    year: ClassVar[int]
    example: ClassVar[str]

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


def get_data_args(args: argparse.Namespace, day_problem: Day) -> str:
    """Return the data for the given day and year."""
    if args.example:
        return day_problem.example
    if args.local:
        return (
            (
                Path(__file__).parents[1]
                / "data"
                / f"input{day_problem.year}-{day_problem.day}.txt"
            )
            .open()
            .read()
        )
    return get_data(day=day_problem.day, year=day_problem.year)


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


def get_args(name: str, arglist: list[str] | None = None) -> argparse.Namespace:
    """Return the arguments from the command line."""
    parser = argparse.ArgumentParser(
        prog=name,
        description="Advent of Code Solver",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        "--parts",
        type=str,
        default="ab",
        choices=["a", "b", "ab"],
        help="Do only specified part, options are 'a', 'b', or 'ab'.",
    )
    # source of data is either example, local, or aocd, defaulting to aocd
    data_group = parser.add_mutually_exclusive_group(required=False)
    data_group.add_argument(
        "--example",
        action="store_true",
        help="Use example input rather than running personal input.",
    )
    data_group.add_argument(
        "--local",
        action="store_true",
        help="Use problem data stored in local data folder as `input/YEAR-DAY.txt`",
    )
    logging_group = parser.add_mutually_exclusive_group(required=False)
    logging_group.add_argument(
        "--verbose",
        action="store_true",
        help="Use python logging to get verbose"
        " output of what is going on in a log file.",
    )
    logging_group.add_argument(
        "--quiet", action="store_true", help="Disable logging for example mode."
    )

    return parser.parse_args(arglist)


def main(problem: Day):
    """Main function for running the problem."""
    day_name = problem.__class__.__name__.lower()
    args = get_args(day_name)
    log = logging.getLogger(day_name)
    edit_logger_for_verbosity(log, args.verbose or args.example, args.quiet)
    data = get_data_args(args, problem)
    answers = problem.solve(data, parts=args.parts)
    print(answers)
    if args.example:
        sys.exit(0)
    submit_answers(answers, args.parts, day=problem.day, year=problem.year)
