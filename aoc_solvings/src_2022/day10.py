"""Advent of Code Day10 problem.

Usage:
    day10.py [--example [--quiet] | --local] [--verbose] [--parts=<char> ...]

Options:
    --example       Use example input rather than running personal input.
    --local         Use problem data stored in local data folder as `inputYEAR-DAY.txt`
    --verbose       Use python logging to get verbose output of what is going on
                    in a log file.
    --quiet         Disable logging for example mode.
    --parts PART    Do only specified part, options are 'a', 'b', or 'ab'. [default: ab]
"""

# Standard Library
from collections.abc import Callable
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

LOG_NAME = "day10"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop"""
)


class Day10(Day):
    """Day 10 of Advent of Code 2022."""

    SIGNAL_STRENGTH_START = 20
    SIGNAL_STRENGTH_REPEAT = 40

    def parse(self, data_input: str) -> list[list[str]]:
        """Given input split on newlines and tokenize it."""
        return [line.split(" ") for line in data_input.splitlines()]

    @staticmethod
    def _print_crt(
        print_f: Callable[..., None], cycle_no: int, register_x: int
    ) -> None:
        pixel = (cycle_no - 1) % 40
        LOG.debug(
            "putting a %r during cycle %d with register %d",
            "#" if register_x - 1 <= pixel <= register_x + 1 else " ",
            cycle_no,
            register_x,
        )
        print_f("#" if register_x - 1 <= pixel <= register_x + 1 else " ", end="")
        if cycle_no != 0 and cycle_no % 40 == 0:
            print_f("")

    @classmethod
    def _process_signal_change(
        cls, cycle_up: int, cycle_no: int, register_x: int, *, verbose: bool
    ) -> int:
        """Return increment to signal_strengths."""
        signal_strengths_incr = 0
        for _ in range(cycle_up):
            cycle_no += 1
            cls._print_crt(
                print if verbose else lambda *args, **kwargs: None,  # type: ignore [arg-type]
                cycle_no,
                register_x,
            )
            if (
                cycle_no == cls.SIGNAL_STRENGTH_START
                or (cycle_no - cls.SIGNAL_STRENGTH_START) % cls.SIGNAL_STRENGTH_REPEAT
                == 0
            ):
                LOG.debug(
                    "cycle %d is going to add strength of %d",
                    cycle_no,
                    cycle_no * register_x,
                )
                signal_strengths_incr += cycle_no * register_x
        return signal_strengths_incr

    @staticmethod
    def _process_command_format(command: list[str]) -> tuple[int, int]:
        """Return num cycles to complete and the value to change register X by."""
        # noop takes 1 cycle, addx takes 2 cycles
        return (1, 0) if len(command) == 1 else (2, int(command[1]))

    @classmethod
    def _process_opcodes(cls, commands: list[list[str]], verbose: bool = False) -> int:
        cycle_no, register_x = 0, 1
        signal_strengths = 0
        for command in commands:
            cycles_to_complete, register_change = cls._process_command_format(command)
            LOG.debug(
                "got command %r at cycle %d, register %d", command, cycle_no, register_x
            )

            signal_strengths += cls._process_signal_change(
                cycles_to_complete, cycle_no, register_x, verbose=verbose
            )
            cycle_no += cycles_to_complete
            register_x += register_change

        return signal_strengths

    def part1(self, data: list[list[str]]) -> int:
        """Find sum of first 6 interesting signal strength.

        Interesting signals come on cycle 20, and then every 40 after.
        Signal strength is cycle number * register value.
        noop commands take 1 cycle, adding takes 2.
        register starts at 1.

        Args:
            data (list[list[str]]): List of opcodes for CPU to execute

        Returns:
            int
        """
        return self._process_opcodes(data)

    def part2(self, data: list[list[str]]) -> None:
        """Render the sprite position across commands.

        CRT (cathode-ray tube) is able to draw one pixel per cycle.
        The CRT screen is a 40 wide by 6 high - meaning a total of 240 cycles
        to fill screen. Render this on screen to get answer

        Args:
            data (list[list[str]]): commands of opcodes
        """
        self._process_opcodes(data, verbose=True)


if __name__ == "__main__":
    global args
    args = docopt(__doc__)
    DAY, YEAR = 10, 2022
    day = Day10()
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
        data = EXAMPLE
    elif args["--local"]:
        data = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data, parts=args["--parts"][0])
    print(answers)
    if args["--example"]:
        assert answers == (13140, None)
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        if part not in args["--parts"]:
            continue
        submit(ans, day=DAY, year=YEAR, part=part)
