# Standard Library
from copy import deepcopy
import os
import re
import sys

# External Party
from aocd import get_data
from aocd import submit

try:
    # My Modules
    from common.template import Day
except ImportError:
    sys.path.insert(0, os.path.dirname(sys.path[0]))
    # My Modules
    from common.template import Day


class CommandError(Exception):
    """Error for invalid crate commands."""

    def __init__(self, *args: object) -> None:
        """Default message for invalid command."""
        super().__init__("Command doesn't match proper format.", *args)


class Day5(Day):
    """Day 5 of Advent of Code 2022."""

    def parse(self, data_input: str) -> list[str]:
        """Given input return each command line.

        Store initial layout in self.crates.
        """
        lines = data_input.split("\n")
        split_point = lines.index("")
        num_stacks = int(lines[split_point - 1].split()[-1])
        self.crates = [[] for _ in range(num_stacks)]
        for crate_num, stack in enumerate(
            re.finditer(r"\d", lines[split_point - 1], re.I)
        ):
            for line in reversed(lines[: split_point - 1]):
                crate_code = line[slice(*stack.span())]
                if crate_code.isalpha():
                    self.crates[crate_num].append(crate_code)

        return lines[split_point + 1 :]

    def part1(self, data: list[str]) -> str:
        """Return top crate code string after all commands executed.

        Each command will look like 'move 1 from 2 to 6', crate initial config
        is in self.crates.

        Args:
            data (List[str]): List of commands

        Returns:
            str: crate code from top level crate after commands
        """
        local_crate = deepcopy(self.crates)

        for command in data:
            match = re.match(r"^move (\d+) from (\d+) to (\d+)$", command, re.I)
            if match is None:
                raise CommandError()
            num_crates, src, dst = (int(g) for g in match.groups())
            for _ in range(num_crates):
                local_crate[dst - 1].append(local_crate[src - 1].pop())

        return "".join(stack[-1] for stack in local_crate)

    def part2(self, data: list[str]) -> str:
        """Return top crate code after stacks have been altered.

        Now when we move multiple from one stack to other -
        order of that stack stays the same.

        Args:
            data (List[str]): Input commands

        Raises:
            AttributeError: If a command is misformed

        Returns:
            str: top crate code for each stack
        """
        local_crate = deepcopy(self.crates)
        for command in data:
            match = re.match(r"^move (\d+) from (\d+) to (\d+)$", command, re.I)
            if match is None:
                raise CommandError()
            num_crates, src, dst = (int(g) for g in match.groups())
            local_crate[dst - 1].extend(local_crate[src - 1][-num_crates:])
            for _ in range(num_crates):
                local_crate[src - 1].pop()
        return "".join(stack[-1] for stack in local_crate)


if __name__ == "__main__":
    DAY, YEAR = 5, 2022
    day = eval(f"Day{DAY}()")
    answers = day.solve(get_data(day=DAY, year=YEAR))
    # print(answers)
    for ans, part in zip(answers, "ab", strict=True):
        submit(ans, day=DAY, year=YEAR, part=part)
