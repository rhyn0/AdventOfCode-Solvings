"""Advent of Code Day2 problem.

Usage:
    day2.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

Options:
    --example       Use example input rather than running personal input.
    --local         Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose       Use python logging to get verbose output of what is going on
                    in a log file.
    --quiet         Disable logging for example mode.
    --parts PART    Do only specified part, options are 'a', 'b', or 'ab'. [default: ab]
"""
# Standard Library
from enum import IntEnum
import logging
import os
from pathlib import Path
import sys
from textwrap import dedent

# External Party
from aocd import get_data
from aocd import submit
from docopt import docopt

try:
    # My Modules
    from common.template import Day
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.template import Day

LOG_NAME = "day2"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    A Y
    B X
    C Z"""
)


class RPS(IntEnum):
    """Enum of choices in Rock Paper Scissors game."""

    ROCK = 1
    PAPER = 2
    SCISSORS = 3


class RPSResult(IntEnum):
    """Enum of possibilities for RockPaperScissors results."""

    P1Win = 1
    DRAW = 2
    P2Win = 3


class Day2(Day):
    """Day 2 of Advent of Code 2022."""

    def parse(self, data_input: str) -> list[str]:
        """Given input return each line."""
        return data_input.split("\n")

    @staticmethod
    def _rps_winner(player1: int, player2: int) -> int:
        # 0 - player1 wins, 1 - draw, 2 - player2 wins
        if player1 == player2:
            return RPSResult.DRAW.value
        if player1 in (RPS.SCISSORS, RPS.ROCK) and player2 in (RPS.SCISSORS, RPS.ROCK):
            return (
                RPSResult.P2Win.value
                if player1 == RPS.SCISSORS
                else RPSResult.P1Win.value
            )
        return RPSResult.P2Win.value if player1 < player2 else RPSResult.P1Win.value

    def part1(self, data: list[str]) -> int:
        """Points earned for following strategy guide as RPS.

        X is rock, Y is paper, Z is scissors

        Args:
            data (List[str]): Player inputs for RPS per round

        Returns:
            int: Points earned for following the guide
        """
        event_total = 0
        for rps_round in data:
            elf, mine = rps_round.split(" ")
            elf_val, me_val = ord(elf) - ord("A"), ord(mine) - ord("X")
            event_total += self._rps_winner(elf_val + 1, me_val) * 3 + me_val + 1
        return event_total

    @staticmethod
    def _rps_counter(player1: int, round_result: int) -> int:
        # 1 - for rock, 2 - for paper, 3 - for scissors
        if round_result == RPSResult.DRAW:
            # tie
            return player1
        if round_result == RPSResult.P1Win:
            return RPS.SCISSORS.value if player1 == RPS.ROCK else player1 - 1
        return RPS.PAPER.value if player1 == RPS.SCISSORS else player1 + 1

    def part2(self, data: list[str]) -> int:
        """Calculate points from actually following strategy guide.

        X is a loss, Y is a draw, Z is a win

        Args:
            data (List[str]): ELf play and desired result per round

        Returns:
            int: Total points accumulated from playing
        """
        event_total = 0
        for rps_round in data:
            elf, result = rps_round.split(" ")
            elf_val, result_val = ord(elf) - ord("A"), ord(result) - ord("X")
            event_total += result_val * 3 + self._rps_counter(elf_val + 1, result_val)

        return event_total


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 2, 2022
    day = Day2()
    if args["--example"] or args["--verbose"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.log", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)
    if args["--quiet"]:
        logging.disable(logging.CRITICAL)

    if args["--example"]:
        grid = EXAMPLE
    elif args["--local"]:
        grid = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        grid = get_data(day=DAY, year=YEAR)
    answers = day.solve(grid, parts=args["--parts"][0])
    print(answers)
    if args["--example"]:
        assert answers == (15, 12)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
