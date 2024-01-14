"""Advent of Code Day21 problem.

Usage:
    day21.py [--example | --local]

Options:
    --example   Use example input rather than running personal input.
    --local     Use problem data stored in local data folder as `inputYEAR-DAY.txt`
"""
from __future__ import annotations

# Standard Library
from dataclasses import dataclass
from dataclasses import field
import logging
from operator import add
from operator import eq
from operator import floordiv
from operator import mul
from operator import sub
import os
from pathlib import Path
import re
import sys
from textwrap import dedent
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Callable

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

LOG_NAME = "day21"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32"""
)


class MonkeyIsSmarterError(Exception):
    """Error for when human can't find a solution to beating monkeys."""

    def __init__(self, *args: object) -> None:
        """Error with default message and pass through *args."""
        super().__init__("Can't calculate better than monkeys", *args)


class HumanNotFoundError(Exception):
    """Error for when tree traversal fails to find the human."""

    def __init__(self, *args: object) -> None:
        """Error with default message and pass through *args."""
        super().__init__("Human can't be found", *args)


class InvalidMonkeyDefinitionError(Exception):
    """Error for when input strings don't yield a monkey."""

    def __init__(self, line_content: str, *args: object) -> None:
        """Error message shows line number that it failed on."""
        super().__init__(f"Monkey definition not found in line {line_content!r}", *args)


class InvalidValueMonkeyError(Exception):
    """Error for when input strings don't yield a monkey."""

    def __init__(self, *args: object) -> None:
        """Default error message and pass through args."""
        super().__init__(
            "Monkey can not return a value as it has no dependencies.", *args
        )


@dataclass(slots=True, kw_only=True)
class Monkey:
    """Monkey object to create graph of data flows."""

    name: str
    depends: list[str] = field(default_factory=list)
    monkey_sources: list[Monkey] = field(default_factory=list, repr=False)
    value: int | None = field(default=None)
    oper: Callable[[int, int], int] | None = field(default=None)

    REV_OPERS = {
        add: sub,
        sub: add,
        mul: floordiv,
        floordiv: mul,
    }

    def get_value(self) -> int:
        """Return value of this monkey.

        If this monkey is an operation monkey, then it will call children
        to offer up their numbers.

        Raises:
            AttributeError: If somehow an operation monkey has no children or operation
                setup

        Returns:
            int
        """
        if self.value:
            return self.value
        if self.monkey_sources is None or self.oper is None:
            raise InvalidValueMonkeyError()
        return self.oper(*[monkey.get_value() for monkey in self.monkey_sources])

    def find_human(self) -> Monkey | None:
        """Return if 'humn' is in child dependencies.

        Returns:
            Monkey | None
        """
        # return monkey parent that is (grand)parent to human
        if "humn" in self.depends or self.name == "humn":
            return self
        for monk in self.monkey_sources:
            if monk.find_human():
                return monk

        return None

    def decide_human_value(self, root_val: int) -> int | None:
        """Given a value from parents, descend down so 'humn' can decide on value.

        Args:
            root_val (int): Value from parent to here

        Returns:
            int | None: Value from 'humn' to match the parent one
        """
        if self.name == "humn":
            return root_val
        if not self.monkey_sources:
            return None
        left, right = self.monkey_sources
        if left.find_human():
            humn_branch = left
            remain = self.REV_OPERS[self.oper](root_val, right.get_value())  # type: ignore
        elif self.oper in (sub, floordiv):
            humn_branch = right
            remain = self.oper(left.get_value(), root_val)  # type: ignore
        else:
            humn_branch = right
            remain = self.REV_OPERS[self.oper](root_val, left.get_value())  # type: ignore

        LOG.debug(
            "From %r, human branch should be %r and sanity check %r",
            self.name,
            humn_branch,
            mk.name if (mk := humn_branch.find_human()) else mk,
        )
        return humn_branch.decide_human_value(remain)


class Day21(Day):
    """Day 21 of Advent of Code 2022."""

    MONKEY_OPS = {
        "+": add,
        "-": sub,
        "*": mul,
        "/": floordiv,
    }

    def parse(self, puzzle_input: str) -> dict[str, Monkey]:
        """Return dict of monkey names to Monkey."""
        monkey_pttrn = re.compile(r"(\w+):\s(\d+|\w+)\s?([-+*/])?\s?(\w+)?")
        monkeys: dict[str, Monkey] = {}
        for line in puzzle_input.splitlines():
            if not (match := monkey_pttrn.match(line)):
                raise InvalidMonkeyDefinitionError(line)
            if match.group(3):
                # operation monkey
                monkeys[match.group(1)] = Monkey(
                    name=match.group(1),
                    depends=list(match.group(2, 4)),
                    oper=self.MONKEY_OPS[match.group(3)],
                )
            else:
                # number source monkey
                monkeys[match.group(1)] = Monkey(
                    name=match.group(1), value=int(match.group(2))
                )

        for monkey in monkeys.values():
            monkey.monkey_sources = [monkeys[other] for other in monkey.depends]

        LOG.info("Parsed out root monkey as %s", monkeys["root"])
        return monkeys

    def part1(self, data: dict[str, Monkey]) -> int:
        """Return value that Monkey 'root' will return."""
        LOG.info("-" * 20 + "starting part1" + "-" * 20)
        return data["root"].get_value()

    def part2(self, data: dict[str, Monkey]) -> int:
        """Return value Monkey 'humn' needs to shout for equality.

        Misunderstanding from above. Monkey 'root' is an equality checker.
        Monkey 'humn' is a wild variable value monkey, needs to shout a number
        such that Monkey 'root' evaluates to equality

        Args:
            data (dict[str, Monkey]): Monkeys

        Raises:
            AttributeError: Searching for human leads to failure
            AttributeError: Unable to ask Monkey 'humn' for a value

        Returns:
            int: Value that Monkey 'humn' needs to contribute to the network
        """
        LOG.info("-" * 20 + "starting part2" + "-" * 20)
        # root is an equality check of its two halves
        root = data["root"]
        root.oper = eq
        human_grand_monk = root.find_human()
        non_humn_grand = next(
            mk for mk in root.monkey_sources if mk is not human_grand_monk
        )
        if human_grand_monk is None or non_humn_grand is None:
            raise HumanNotFoundError()
        LOG.info(
            "Human branch from root is %r and the other is %r",
            human_grand_monk.name,
            non_humn_grand.name,
        )
        test_val = human_grand_monk.decide_human_value(non_humn_grand.get_value())
        if test_val is None:
            raise MonkeyIsSmarterError()
        return test_val


if __name__ == "__main__":
    global args
    args = docopt(__doc__)  # type: ignore
    DAY, YEAR = 21, 2022
    day = Day21()

    if args["--example"]:
        handle = logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.log", "w")
        handle.setFormatter(
            logging.Formatter("%(funcName)s-%(levelname)s:%(lineno)d %(message)s")
        )
        LOG.addHandler(handle)
        LOG.setLevel(logging.DEBUG)
        data = EXAMPLE
    elif args["--local"]:
        data = (Path(sys.path[0]) / "data" / f"input{YEAR}-{DAY}.txt").open().read()
    else:
        data = get_data(day=DAY, year=YEAR)
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
