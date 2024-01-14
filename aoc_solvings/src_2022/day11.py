"""Advent of Code Day11 problem.

Usage:
    day11.py [--example]

Options:
    --example   Use example input rather than running personal input.
"""
from __future__ import annotations

# Standard Library
from collections import deque
from copy import deepcopy
from dataclasses import dataclass
from dataclasses import field
from functools import reduce
import logging
from operator import add
from operator import mul
from operator import sub
import os
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

LOG_NAME = "day11"
LOG = logging.getLogger(LOG_NAME)
LOG.setLevel(logging.CRITICAL)

EXAMPLE = dedent(
    """\
    Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

    Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

    Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

    Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1"""
)


@dataclass
class Monkey:
    """Representative of monkey that is messing with our stuff.

    Mostly holds their inspection multiplier, testing components, and routes to pass.
    """

    monkey_id: int
    worry_op: Callable[[int, int], int]
    worry_component: int
    test_div: int
    monkey_dest: tuple[int, int]
    items: deque[int] = field(default_factory=deque)
    inspect_no: int = field(default=0)

    _MOD = 0

    def receive_item(self, worry_item: int) -> None:
        """Add worry item to end of queue of items to process."""
        self.items.append(worry_item)

    def inspect(self, item_no: int, worry_reset: bool) -> bool:
        """Perform inspection, and change worry level of item."""
        new_item_val = self.worry_op(self.items[item_no], self.worry_component)
        if worry_reset:
            new_item_val //= 3
        self.items[item_no] = new_item_val % self._MOD

        self.inspect_no += 1
        return self.items[item_no] % self.test_div == 0

    def throw_item(self, monkeys: list[Monkey], worry_reset: bool) -> None:
        """Perform decision on item and throw to destination monkey."""
        item_no = len(self.items) - 1
        test_result = self.inspect(item_no, worry_reset)
        monkeys[self.monkey_dest[test_result]].receive_item(self.items.pop())

    def perform_turn(self, monkeys: list[Monkey], worry_reset: bool) -> None:
        """Perform this monkey's turn in the group."""
        while len(self.items):
            self.throw_item(monkeys, worry_reset)


class UnknownOperationError(Exception):
    """Error for when monkeys do math in an unknown way."""

    def __init__(self, *args: object) -> None:
        """Default message and pass through args."""
        super().__init__("Unknown monkey math occurring", *args)


class Day11(Day):
    """Day 11 of Advent of Code 2022."""

    def parse(self, data_input: str) -> list[Monkey]:
        """Given input split create monkeys and starting items."""
        datalines = data_input.splitlines()
        monkeys: list[Monkey] = []
        line_no = 0
        while line_no < len(datalines):
            items = [
                int(item.strip())
                for item in datalines[line_no + 1][
                    datalines[line_no + 1].index(":") + 1 :
                ].split(",")
                if item.strip().isnumeric()
            ]
            oper_equation = datalines[line_no + 2][
                datalines[line_no + 2].index("=") + 2 :
            ].split()
            match oper_equation:
                case [_, "*", "old"]:
                    oper = lambda x, _: x**2  # noqa: E731
                    oper_component = 1
                case [_, "*", val]:
                    oper = mul
                    oper_component = int(val)
                case [_, "+", val]:
                    oper = add
                    oper_component = int(val)
                case [_, "-", val]:
                    oper = sub
                    oper_component = int(val)
                case _:
                    print(oper_equation)
                    raise UnknownOperationError()

            test_divider = int(
                datalines[line_no + 3][datalines[line_no + 3].index("by") + 3 :]
            )

            true_monk = int(
                datalines[line_no + 4][datalines[line_no + 4].index("monkey") + 7 :]
            )
            false_monk = int(
                datalines[line_no + 5][datalines[line_no + 5].index("monkey") + 7 :]
            )
            curr_monk = Monkey(
                len(monkeys),
                oper,
                oper_component,
                test_divider,
                (false_monk, true_monk),
                deque(items),
            )
            monkeys.append(curr_monk)
            line_no += 7
        return monkeys

    @staticmethod
    def _perform_n_rounds(
        monkeys: list[Monkey], num: int = 20, worry_reset: bool = True
    ) -> list[int]:
        """Peform n rounds of monkey business.

        Round is each monkey peforms turn

        Args:
            monkeys (list[Monkey]): monkeys for the round
            num (int, optional): Number of rounds to perform. Defaults to 20.
            worry_reset (bool, optional): Whether worry levels will reset after
                not damaging items. Default to True.

        Returns:
            list[int]: Number of inspects per monkey
        """
        local_monkey = deepcopy(monkeys)
        max_mod = reduce(mul, [monk.test_div for monk in local_monkey], 1)
        Monkey._MOD = max_mod
        for _ in range(num):
            for monkey in local_monkey:
                monkey.perform_turn(local_monkey, worry_reset)

        return [monk.inspect_no for monk in local_monkey]

    def part1(self, data: list[Monkey]) -> int:
        """Perform 20 rounds of monkey business and return score.

        Args:
            data (list[Monkey]): list of monkeys with items

        Returns:
            int: monkey business score
        """
        inspects = self._perform_n_rounds(data)
        top_monkey = max(inspects)
        inspects.remove(top_monkey)
        return max(inspects) * top_monkey

    def part2(self, data: list[Monkey]) -> int:
        """Perform 10,000 rounds of monkey business and return score."""
        inspects = self._perform_n_rounds(data, num=10_000, worry_reset=False)
        top_monkey = max(inspects)
        inspects.remove(top_monkey)
        return max(inspects) * top_monkey


if __name__ == "__main__":
    DAY, YEAR = 11, 2022
    day = eval(f"Day{DAY}()")
    global args
    args = docopt(__doc__)
    if args["--example"]:
        LOG.addHandler(logging.FileHandler(f"{sys.path[1]}/{LOG_NAME}.txt", "w"))
        LOG.setLevel(logging.DEBUG)

    data = get_data(day=DAY, year=YEAR) if not args["--example"] else EXAMPLE
    answers = day.solve(data)
    print(answers)
    if args["--example"]:
        sys.exit(0)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
